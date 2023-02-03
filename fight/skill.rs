use crate::fight::{EmImpactTarget, EmSkillTargetOpt, EmSkillType, TableRowSKill, TargetList};
use crate::fight::fight_object::FightObject;
use crate::fight::skill_const::{MAX_MATRIX_CELL_COUNT, MAX_SKILL_IMPACT_COUNT};
use rand::Rng;
use crate::fight::EmTypeFight::EmTypeFightStair;
use crate::fight::impact::{Impact, TableRowImpact};
use crate::fight::impact_info::ImpactInfo;
use crate::fight::utils::{calcdamage, chkmin, get_rand};

struct CoolDown {
    id: u32,
    end_time: u32,
}

struct CoolDownList {
    count: u32,
    cd_list: Vec<CoolDown>,
}

#[derive(Debug, Default, Clone)]
pub struct Skill {
    /// 技能ID
    skill_id: u32,
    /// 技能类型
    skill_type: EmSkillType,
    /// 技能时间
    skill_time: u32,
    /// 冷却时间
    cool_down_time: u32,
    /// 释放者
    p_caster: Box<FightObject>,
    /// 技能的目标
    p_target: Box<FightObject>,
}

impl Skill {
    pub fn new(&self) -> Self {
        Self::default()
    }

    pub fn init(&mut self, n_skill_id: u32, p_caster: FightObject) {
        self.skill_id = n_skill_id;
        self.p_caster = Box::from(p_caster);
        let p_skill_row = self.get_skill_row();
        self.skill_type = EmSkillType::try_from(p_skill_row.skill_type).unwrap();
        self.skill_time = p_skill_row.start_round;
        self.cool_down_time = p_skill_row.cool_down_time;
    }
    pub fn clean_up(&self) {}
    pub fn is_valid(&self) -> bool { true }

    /// 技能数据
    /// 获取技能ID
    pub fn get_skill_id(&self) -> u32 {
        self.skill_id
    }

    pub fn get_skill_type(&self) -> EmSkillType {
        self.skill_type
    }

    /// 技能静态数据
    pub fn get_skill_row(&self) -> TableRowSKill {
        TableRowSKill::default()
    }

    /// 获取施放者
    pub fn get_caster(&self) -> &FightObject {
        &self.p_caster
    }
    /// 获得技能释放几率
    pub fn get_skill_cast_rate(&self) -> u32 { 0 }
    ///  设置技能目标列表
    pub fn set_skill_target(pTarget: &FightObject) {}

    /// 逻辑流程//魔法技能-主动
    pub fn skill_logic(&mut self, nRound: u32) -> bool {
        if !self.is_valid() {
            return false;
        }

        if !self.check_condition(nRound) {
            return false;
        }

        if !self.select_target() {
            return false;
        }

        if !self.cast_skill(nRound) {
            return false;
        }

        true
    }
    /// 魔法技能-被动
    pub fn passive_skill_logic(&mut self, n_round: u32) -> bool {
        if !self.is_valid() {
            return false;
        }

        if n_round != 1 {
            return false;
        }

        if !self.check_condition(n_round) {
            return false;
        }

        if !self.select_target() {
            return false;
        }

        if !self.cast_skill(n_round) {
            return false;
        }

        true
    }

    pub fn check_condition(&self, n_round: u32) -> bool {
        if self.skill_type == EmSkillType::EmSkillTypeHeroPassive {
            return false;
        }
        if n_round < self.skill_time {
            return false;
        }

        let p_skill_row = self.get_skill_row();

        if self.p_caster.get_mp() < p_skill_row.need_mp {
            return false;
        }

        let mut rng = rand::thread_rng();
        let n_rand = rng.gen_range(0..10000);
        if n_rand > p_skill_row.skill_rate {
            return false;
        }

        true
    }

    ///
    /// #选择目标
    /// ```
    /// 6 7 8
    /// 3 4 5
    /// 0 1 2
    /// ------
    /// 0 1	2
    /// 3 4	5
    /// 6 7 8
    /// ```
    pub fn select_target(&mut self) -> bool {
        let p_skill_row = self.get_skill_row();
        let mut p_enemy_list = self.p_caster.get_enemy_list();
        let opt = EmSkillTargetOpt::try_from(p_skill_row.select_target_opt).unwrap();

        match opt {
            EmSkillTargetOpt::EmSkillTargetOptAuto => {
                self.p_target = self.p_caster.clone();
                return true;
            }
            EmSkillTargetOpt::EmSkillTargetOptNumber => {
                for i in 0..=MAX_MATRIX_CELL_COUNT / 2 {
                    let front_index = (self.p_caster.get_matrix_id() + i) % (MAX_MATRIX_CELL_COUNT / 2);
                    let back_index = front_index + MAX_MATRIX_CELL_COUNT / 2;
                    let mut fight_object = p_enemy_list.get_fight_object(front_index);
                    if fight_object.is_active() {
                        self.p_target = self.p_caster.clone();
                        return true;
                    }

                    fight_object = p_enemy_list.get_fight_object(back_index);
                    if fight_object.is_active() {
                        self.p_target = self.p_caster.clone();
                        return true;
                    }
                }
            }
            EmSkillTargetOpt::EmSkillTargetOptRand => {
                let mut n_count = 0;
                let mut n_index_list = [0; MAX_MATRIX_CELL_COUNT];
                for i in 0..MAX_MATRIX_CELL_COUNT {
                    let fight_object = p_enemy_list.get_fight_object(i as usize);
                    if fight_object.is_active() {
                        n_index_list[n_count] = i;
                        n_count += 1;
                    }
                }
                if n_count > 0 {
                    let mut rng = rand::thread_rng();
                    let rand_index = rng.gen_range(0..n_count);
                    let fo = p_enemy_list.get_fight_object(n_index_list[rand_index]);
                    self.p_target = Box::from(fo.clone());
                    return true;
                }
            }

            EmSkillTargetOpt::EmSkillTargetOptSlow => {
                let front_index = self.p_caster.get_matrix_id() % (MAX_MATRIX_CELL_COUNT / 2);
                for i in 0..MAX_MATRIX_CELL_COUNT / 2 {
                    let index = (front_index + i) % (MAX_MATRIX_CELL_COUNT / 2) + MAX_MATRIX_CELL_COUNT / 2;
                    let fight_object = p_enemy_list.get_fight_object(index as usize);
                    if fight_object.is_active() {
                        self.p_target = Box::from(fight_object.clone());
                        return true;
                    }
                }
                for i in 0..MAX_MATRIX_CELL_COUNT / 2 {
                    let index = (front_index + i) % (MAX_MATRIX_CELL_COUNT / 2);
                    let fight_object = p_enemy_list.get_fight_object(index as usize);
                    if fight_object.is_active() {
                        self.p_target = Box::from(fight_object.clone());
                        return true;
                    }
                }
            }
            _ => {}
        }

        false
    }

    pub fn cast_skill(&mut self, nRound: u32) -> bool {
        let p_skill_row = self.get_skill_row();
        self.p_caster.set_mp(self.p_caster.get_mp() - p_skill_row.need_mp);
        self.skill_time = nRound + p_skill_row.cool_down_time;

        let mut p_attack_info = self.p_caster.get_attack_info();
        p_attack_info.cast_guid = self.p_caster.get_guid();
        p_attack_info.b_skilled = true;

        let o_skill_attack = p_attack_info.alloc_skill_attack();
        if o_skill_attack.is_none() {
            return false;
        }

        let mut p_skill_attack = o_skill_attack.unwrap().clone();
        p_skill_attack.skill_id = self.skill_id;
        p_skill_attack.skill_target = self.p_target.get_guid();
        p_skill_attack.cost_mp = p_skill_row.need_mp;

        let mut n_con_att_times = self.get_con_atk_times();
        for i in 0..MAX_SKILL_IMPACT_COUNT {
            let n_rand = get_rand(1, 10000);
            if n_rand <= p_skill_row.impact_rate[i as usize] && p_skill_row.impact_id[i as usize] > 1 {
                let p_row_impact = TableRowImpact::default();
                if !(p_row_impact.logic_id == 0 || p_row_impact.logic_id == 1) {
                    n_con_att_times = 0;
                }

                let mut impact_info = ImpactInfo::new();
                impact_info.skill_id = self.skill_id;
                impact_info.impact_id = p_skill_row.impact_id[i as usize];
                impact_info.con_atk_times = n_con_att_times;

                let mut target_list = TargetList::new();

                let n = p_skill_row.impact_target_type[i as usize];
                let n_type = EmImpactTarget::try_from(n).unwrap();
                self.get_target_list(n_type, &mut target_list);
                if target_list.object_list.len() == 0 {
                    continue;
                }

                for index in 0..target_list.object_list.len() {
                    let p_target = target_list.get_fight_object(index);
                    if p_target.is_active() {
                        let idx = impact_info.target_count;
                        impact_info.target_list.push(p_target.get_guid());
                    }
                }

                p_skill_attack.add_import_info(impact_info);

                for index in 0..target_list.object_list.len() {
                    let p_target = target_list.get_fight_object(index);
                    if p_target.is_active() {
                        let nResult = p_target.add_impact(p_skill_row.impact_id[i as usize], n_con_att_times, nRound, self.p_caster.clone(), self.skill_id);
                    }
                }
            }
        }


        true
    }
    /// 本次技能连击次数
    pub fn get_target_list(&self, nType: EmImpactTarget, targetList: &mut TargetList) -> bool {
        targetList.cleanup();
        match nType {
            EmImpactTarget::EmImpactTargetOptSelf => {}
            EmImpactTarget::EmImpactTargetOwnerSingle => {}
            EmImpactTarget::EmImpactTargetOwnerAll => {}
            EmImpactTarget::EmImpactTargetEnemySingle => {}
            EmImpactTarget::EmImpactTargetEnemyFront => {}
            EmImpactTarget::EmImpactTargetEnemyBehind => {}
            EmImpactTarget::EmImpactTargetEnemyAll => {}
            EmImpactTarget::EmImpactTargetEnemyLine => {}
            EmImpactTarget::EmImpactTargetEnemyAround => {}
            EmImpactTarget::EmImpactTargetEnemyBehindOne => {}
            EmImpactTarget::EmImpactTargetOwnerMinHp => {}
            EmImpactTarget::EmImpactTargetOwnerMinMp => {}
            _ => {}
        }
        true
    }
    pub fn get_con_atk_times(&self) -> u32 { 0 }

    /// 普通攻击技能
    pub fn common_skill_logic(&mut self, nRound: u32) -> bool {
        let mut p_attack_info = self.p_caster.get_attack_info();
        if !self.select_target() {
            return false;
        }

        p_attack_info.cast_guid = self.p_caster.get_guid();

        let b_hit = self.can_hit();
        p_attack_info.b_hit = b_hit;

        if !b_hit {
            return false;
        }

        let mut n_physic_attack = self.p_caster.get_physic_attack() as f32;

        let b_strike = self.can_strike();
        p_attack_info.b_strike = b_strike;
        p_attack_info.skill_target = self.p_target.get_guid();
        if b_strike {
            n_physic_attack = n_physic_attack  * (1.0 + self.p_caster.get_strike_hurt()  as f32 * 0.01);
        }

        let p_fight_cell = self.p_caster.get_fight_cell();
        if p_fight_cell.get_fight_type() == EmTypeFightStair && self.p_caster.is_attacker() {
            n_physic_attack += n_physic_attack  * p_fight_cell.get_plus_atk() as f32 / 100.0;
        }

        let n_defend = self.p_target.get_physic_defend() as f32;
        let n_decay = self.p_target.get_physic_hurt_decay() as f32;
        //本次物理攻击伤害=(自身当前经过连击计算后的物理攻击-目标物理防御)*(1-目标物理伤害减免/100)
        let mut n_damage = calcdamage(n_physic_attack, n_defend, n_decay);
        n_damage = (n_damage as f32 * (1.0 + self.p_caster.get_floating_hurt() as f32 * 0.01)) as u32;
        chkmin(&mut n_damage, 0);

        self.p_target.set_hp(self.p_target.get_hp() - n_damage);
        p_attack_info.skill_target = self.p_target.get_guid();
        p_attack_info.hurt = n_damage;

        if self.p_target.is_active() && self.can_back_attack() {
            let back_attack = self.p_target.get_physic_attack() * self.p_target.get_back_att_hurt() / 100;
            self.p_caster.set_hp(self.p_target.get_hp() + back_attack);

            p_attack_info.b_back_attack = true;
            p_attack_info.back_attack_hurt = back_attack;
        }


        true
    }
    /// 是否命中
    pub fn can_hit(&self) -> bool {
        let n_hit = self.p_caster.get_hit() - self.p_target.get_dodge();
        if n_hit >= 100 {
            return true;
        }

        if n_hit <= 0 {
            return false;
        }

        let n_rand = get_rand(1, 100);
        if n_rand <= n_hit {
            return true;
        }

        false
    }
    /// 是否暴击
    pub fn can_strike(&self) -> bool {
        let n_strike = self.p_caster.get_strike();
        if n_strike >= 100 {
            return true;
        }

        if n_strike <= 0 {
            return false;
        }

        let n_rand = get_rand(1, 100);
        if n_rand <= n_strike {
            return true;
        }

        false
    }
    /// 是否反击
    pub fn can_back_attack(&self) -> bool {
        let n_strike = self.p_caster.get_strike();
        if n_strike >= 100 {
            return true;
        }

        if n_strike <= 0 {
            return false;
        }

        let n_rand = get_rand(1, 100);
        if n_rand <= n_strike {
            return true;
        }

        false
    }
    /// 装备技能-主动
    pub fn equip_skill_logic(&self, nRound: u32, pTarget: &FightObject) -> bool { true }
}