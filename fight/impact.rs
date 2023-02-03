use crate::fight::fight_object::FightObject;
use crate::fight::{EmAttribute, EmTypeFight};
use crate::fight::skill_const::MAX_IMPACT_LOGIC_PARAM_COUNT;
use crate::fight::utils::{calcdamage, chkmin};

#[derive(Debug, Default, Clone)]
pub struct TableRowImpact {
    pub impact_id: u32,
    //impact_id
    pub description: String,
    //策划描述
    pub logic_id: u32,
    //Impact逻辑id
    pub param: [i32; MAX_IMPACT_LOGIC_PARAM_COUNT as usize],
    //逻辑参数
    pub icon: String,
    //Buff图标
    pub impact_mutex_id: u32,
    //IMPACT互斥id
    pub replace_level: u32,
    //顶替优先级
    pub dead_disapeer: u32,
    //死亡后是否消失
    pub offline_time_go: u32,
    //下线是否计时	INT
    pub script_id: u32,
    //脚本ID
    pub sz_effect: String,
    //特效
    pub sz_name: String,
    //名称
    pub sz_desc: String,
    //描述
    pub sz_skill_effect: String,                        //技能效果
}

#[derive(Debug, Default, Clone)]
pub struct Impact {
    impact_id: u32,
    //ID
    p_holder: Box<FightObject>,
    //拥有者
    p_caster: Box<FightObject>,
    //施放者
    skill_id: u32,
    //
    start_time: u32,
    //开始时间
    life: u32,
    //生命周期
    logic_time: u32,
    //开始作用时间
    term: u32,
    //开始间隔
    con_att_times: u32,                    //连击次数
}


impl Impact {
    pub fn new() -> Impact {
        Impact::default()
    }

    pub fn init(&mut self, impact_id: u32, con_att_times: u32, n_round: u32, p_caster: FightObject, p_holder: FightObject, n_skill_id: u32) {
        self.impact_id = impact_id;
        self.p_holder = Box::from(p_holder);
        self.p_caster = Box::from(p_caster);
        self.skill_id = n_skill_id;
        self.start_time = n_round;
        self.life = n_round;
        self.logic_time = n_round;
        self.term = 1;
        self.con_att_times = 0;

        let row = self.get_impact_row();
        match row.logic_id {
            0 | 1 | 6 => {
                self.con_att_times = con_att_times;
            }
            2 | 3 => {
                self.life += (row.param[2] - row.param[3]) as u32;
                self.term = row.param[3] as u32
            }
            4 | 5 => {
                self.life += row.param[2] as u32 - 1;
            }
            _ => {}
        }
    }

    pub fn cleanup(&mut self) {
        self.impact_id = 0;
    }

    pub fn is_valid(&self) -> bool {
        if self.impact_id > 0 {
            return true;
        }
        false
    }


    pub fn get_id(&self) -> u32 {
        self.impact_id
    }

    pub fn get_impact_row(&self) -> TableRowImpact {
        TableRowImpact::default()
    }

    pub fn get_holder(&self) {}

    pub fn get_caster(&self) {}


    /// 触发逻辑
    pub fn heartbeat(&mut self, u_time: u32) -> bool {
        if !self.is_valid() {
            return false;
        }

        if u_time == self.logic_time {
            let row = self.get_impact_row();
            match row.logic_id {
                0 => self.impact_logic0(),
                1 => self.impact_logic1(),
                2 => self.impact_logic2(),
                3 => self.impact_logic3(),
                4 => self.impact_logic4(),
                5 => self.impact_logic5(),
                6 => self.impact_logic6(),
                _ => {}
            }
        }

        true
    }

    /// # 0=单次物理攻击；
    /// * 逻辑参数1：从英雄物理攻击中取得的倍率（例：150）
    /// * 逻辑参数2：额外增加的物理伤害（例：50）
    /// * 如英雄物理攻击为100，则最终的技能物理攻击=英雄物理攻击（100）*逻辑参数1（150/100）+逻辑参数2（50）
    pub fn impact_logic0(&mut self) {
        let p_attack_info = self.p_caster.get_attack_info();

        if !p_attack_info.is_valid() {
            return;
        }

        let res_skill_attack = p_attack_info.get_skill_attack(self.skill_id);

        if res_skill_attack.is_none(){
            return;
        }
        let p_skill_attack = res_skill_attack.unwrap();

        if !p_skill_attack.is_valid() {
            return;
        }

        let mut p_impact_info = p_skill_attack.get_impact_info(self.impact_id);

        if !p_impact_info.is_valid() {
            return;
        }

        let target_index = p_impact_info.get_target_index(self.p_holder.get_guid());
        if None == target_index {
            return;
        }

        let index = target_index.unwrap();

        let p_fight_cell = self.p_holder.get_fight_cell();
        let n_physic_attack = self.p_caster.get_physic_attack();
        let p_row = self.get_impact_row();

        let mut n_attack = n_physic_attack as f32 * p_row.param[0] as f32 * 0.01 + p_row.param[1] as f32;
        if p_fight_cell.get_fight_type() == EmTypeFight::EmTypeFightNormal && self.p_caster.is_attacker() {
            n_attack += n_attack * p_fight_cell.get_plus_atk() as f32 / 100.0;
        }

        let n_defend = self.p_holder.get_physic_defend() as f32;
        let n_decay = self.p_holder.get_physic_hurt_decay() as f32;
        //本次物理攻击伤害=(自身当前经过连击计算后的物理攻击-目标物理防御)*(1-目标物理伤害减免/100)
        let mut n_damage = calcdamage(n_attack, n_defend, n_decay);
        chkmin(&mut n_damage, 0);

        self.p_holder.set_hp(self.p_holder.get_hp() - n_damage);

        p_impact_info.hurts[index][0] = n_damage as i32;

        //连击
        let f_con_att_hurt = self.p_caster.get_con_atk_hurt() as f32 * 0.01;
        for i in 1..=self.con_att_times {
            n_attack = n_attack * f_con_att_hurt;
            n_damage = calcdamage(n_attack, n_defend, n_decay);
            chkmin(&mut n_damage, 0);

            self.p_holder.set_hp(self.p_holder.get_hp() - n_damage);

            p_impact_info.hurts[index][i as usize] = n_damage as i32;
        }
    }

    /// # 1=单次魔法攻击
    /// * 逻辑参数1：从英雄物理攻击（因为英雄默认没有魔法攻击）中取得的倍率（例：150）
    /// * 逻辑参数2：额外增加的魔法伤害（例：50）
    /// * 如英雄物理攻击为100，则最终的技能魔法攻击=英雄物理攻击（100）*逻辑参数1（150/100）+逻辑参数2（50）
    pub fn impact_logic1(&mut self) {
        let p_attack_info = self.p_caster.get_attack_info();

        if !p_attack_info.is_valid() {
            return;
        }

        let res_skill_attack = p_attack_info.get_skill_attack(self.skill_id);

        if res_skill_attack.is_none() {
            return;
        }
        let p_skill_attack = res_skill_attack.unwrap();

        if !p_skill_attack.is_valid() {
            return;
        }

        let mut p_impact_info = p_skill_attack.get_impact_info(self.impact_id);

        if !p_impact_info.is_valid() {
            return;
        }

        let target_index = p_impact_info.get_target_index(self.p_holder.get_guid());
        if None == target_index {
            return;
        }

        let index = target_index.unwrap();

        let p_fight_cell = self.p_holder.get_fight_cell();
        let n_physic_attack = self.p_caster.get_physic_attack();
        let p_row = self.get_impact_row();

        let mut n_attack = n_physic_attack as f32 * p_row.param[0] as f32 * 0.01 + p_row.param[1] as f32;
        if p_fight_cell.get_fight_type() == EmTypeFight::EmTypeFightStair && self.p_caster.is_attacker() {
            n_attack += n_attack * p_fight_cell.get_plus_atk() as f32 / 100.0;
        }

        let n_defend = self.p_holder.get_magic_defend() as f32;
        let n_decay = self.p_holder.get_magic_hurt_decay() as f32;
        //本次物理攻击伤害=(自身当前经过连击计算后的物理攻击-目标物理防御)*(1-目标物理伤害减免/100)
        let mut n_damage = calcdamage(n_attack, n_defend, n_decay);
        chkmin(&mut n_damage, 0);

        self.p_holder.set_hp(self.p_holder.get_hp() - n_damage);

        p_impact_info.hurts[index][0] = n_damage as i32;

        //连击
        let f_con_att_hurt = self.p_caster.get_con_atk_hurt() as f32 * 0.01;
        for i in 1..=self.con_att_times {
            n_attack = n_attack * f_con_att_hurt;
            n_damage = calcdamage(n_attack, n_defend, n_decay);
            chkmin(&mut n_damage, 0);

            self.p_holder.set_hp(self.p_holder.get_hp() - n_damage);

            p_impact_info.hurts[index][i as usize] = n_damage as i32;
        }
    }

    /// # 2=物理持续攻击
    /// * 逻辑参数1：从英雄物理攻击中取得的倍率（例：150）
    /// * 逻辑参数2：额外增加的物理伤害（例：50）
    /// * 逻辑参数3：持续时间，单位回合（10）
    /// * 逻辑参数4：生效间隔，单位回合（2）
    /// * 此效果的最终效果为：每2回合对目标造成一次物理伤害，持续10回合（生效5次），每次造成的物理攻击具体数值=英雄物理攻击（100）*逻辑参数1（150/100）+逻辑参数2（50）
    pub fn impact_logic2(&mut self) {
        let p_attack_info = self.p_caster.get_attack_info();

        if !p_attack_info.is_valid() {
            return;
        }

        let res_skill_attack = p_attack_info.get_skill_attack(self.skill_id);

        if res_skill_attack.is_none() {
            return;
        }
        let p_skill_attack = res_skill_attack.unwrap();

        if !p_skill_attack.is_valid() {
            return;
        }

        let mut p_impact_info = p_skill_attack.get_impact_info(self.impact_id);

        if !p_impact_info.is_valid() {
            return;
        }

        let target_index = p_impact_info.get_target_index(self.p_holder.get_guid());
        if None == target_index {
            return;
        }

        let index = target_index.unwrap();

        let p_fight_cell = self.p_holder.get_fight_cell();
        let n_physic_attack = self.p_caster.get_physic_attack();
        let p_row = self.get_impact_row();

        if self.logic_time <= self.life {
            let mut n_attack = n_physic_attack as f32 * p_row.param[0] as f32 * 0.01 + p_row.param[1] as f32;
            if p_fight_cell.get_fight_type() == EmTypeFight::EmTypeFightStair && self.p_caster.is_attacker() {
                n_attack += n_attack * p_fight_cell.get_plus_atk() as f32 / 100.0;
            }

            let n_defend = self.p_holder.get_physic_defend() as f32;
            let n_decay = self.p_holder.get_physic_hurt_decay() as f32;
            //本次物理攻击伤害=(自身当前经过连击计算后的物理攻击-目标物理防御)*(1-目标物理伤害减免/100)
            let mut n_damage = calcdamage(n_attack, n_defend, n_decay);
            chkmin(&mut n_damage, 0);

            self.p_holder.set_hp(self.p_holder.get_hp() - n_damage);

            if self.start_time == self.logic_time {
                p_impact_info.hurts[index][0] = n_damage as i32;
            } else if self.start_time < self.logic_time {
                let p_round_info = p_fight_cell.get_round_info();
                let mut p_fobjectinfo = p_round_info.get_fobjectinfo_by_guid(self.p_holder.get_guid());

                if p_fobjectinfo.is_some() {
                    let obj = p_fobjectinfo.as_mut().unwrap();
                    for (i, id) in obj.m_impact_list.iter().enumerate() {
                        if *id == self.impact_id {
                            obj.m_impact_hurt[i] = n_damage as i32;
                        }
                    }
                }
            }
        }
        self.logic_time += self.term;
    }

    /// # 3=持续魔法攻击
    /// * 逻辑参数1：从英雄物理攻击（因为英雄默认没有魔法攻击）中取得的倍率（例：150）
    /// * 逻辑参数2：额外增加的魔法伤害（例：50）
    /// * 逻辑参数3：持续时间，单位回合（10）
    /// * 逻辑参数4：生效间隔，单位回合（2）
    /// * 此效果的最终效果为：每2回合对目标造成一次魔法伤害，持续10回合（生效5次），每次造成的魔法攻击具体数值=英雄物理攻击（100）*逻辑参数1（150/100）+逻辑参数2（50）
    pub fn impact_logic3(&mut self) {
        let p_attack_info = self.p_caster.get_attack_info();

        if !p_attack_info.is_valid() {
            return;
        }

        let res_skill_attack = p_attack_info.get_skill_attack(self.skill_id);

        if res_skill_attack.is_none() {
            return;
        }
        let p_skill_attack = res_skill_attack.unwrap();

        if !p_skill_attack.is_valid() {
            return;
        }

        let mut p_impact_info = p_skill_attack.get_impact_info(self.impact_id);

        if !p_impact_info.is_valid() {
            return;
        }

        let target_index = p_impact_info.get_target_index(self.p_holder.get_guid());
        if None == target_index {
            return;
        }

        let index = target_index.unwrap();

        let p_fight_cell = self.p_holder.get_fight_cell();
        let n_physic_attack = self.p_caster.get_physic_attack();
        let p_row = self.get_impact_row();

        if self.logic_time <= self.life {
            let mut n_attack = n_physic_attack as f32 * p_row.param[0] as f32 * 0.01 + p_row.param[1] as f32;
            if p_fight_cell.get_fight_type() == EmTypeFight::EmTypeFightStair && self.p_caster.is_attacker() {
                n_attack += n_attack * p_fight_cell.get_plus_atk() as f32 / 100.0;
            }

            let n_defend = self.p_holder.get_magic_defend() as f32;
            let n_decay = self.p_holder.get_magic_hurt_decay() as f32;
            //本次物理攻击伤害=(自身当前经过连击计算后的物理攻击-目标物理防御)*(1-目标物理伤害减免/100)
            let mut n_damage = calcdamage(n_attack, n_defend, n_decay);
            chkmin(&mut n_damage, 0);

            self.p_holder.set_hp(self.p_holder.get_hp() - n_damage);

            if self.start_time == self.logic_time {
                p_impact_info.hurts[index][0] = n_damage as i32;
            } else if self.start_time < self.logic_time {
                let p_round_info = p_fight_cell.get_round_info();
                let mut p_fobjectinfo = p_round_info.get_fobjectinfo_by_guid(self.p_holder.get_guid());

                if p_fobjectinfo.is_some() {
                    let obj = p_fobjectinfo.as_mut().unwrap();
                    for (i, id) in obj.m_impact_list.iter().enumerate() {
                        if *id == self.impact_id {
                            obj.m_impact_hurt[i] = n_damage as i32;
                        }
                    }
                }
            }
        }
        self.logic_time += self.term;
    }

    /// # 4=buff强化类
    /// * 逻辑参数1：改变的英雄属性id，读取AttributeData.tab表。
    /// * 逻辑参数2：改变的具体数值
    /// * 逻辑参数3：持续时间，单位回合（10）
    /// * 最终可实现的效果如英雄攻击增加X点持续10回合。
    pub fn impact_logic4(&mut self) {
        let p_attack_info = self.p_caster.get_attack_info();

        if !p_attack_info.is_valid() {
            return;
        }

        let res_skill_attack = p_attack_info.get_skill_attack(self.skill_id);

        if res_skill_attack.is_none() {
            return;
        }
        let p_skill_attack = res_skill_attack.unwrap();

        if !p_skill_attack.is_valid() {
            return;
        }

        let mut p_impact_info = p_skill_attack.get_impact_info(self.impact_id);

        if !p_impact_info.is_valid() {
            return;
        }

        let target_index = p_impact_info.get_target_index(self.p_holder.get_guid());
        if None == target_index {
            return;
        }

        let index = target_index.unwrap();

        let p_fight_cell = self.p_holder.get_fight_cell();
        let n_physic_attack = self.p_caster.get_physic_attack();
        let p_row = self.get_impact_row();

        if self.logic_time <= self.life {
            let n = p_row.param[0] as u8;
            let tp = EmAttribute::try_from(n);
            let attr_type = tp.unwrap();
            let n_attr_type = attr_type.into_usize();
            self.p_holder.change_effect(n_attr_type, p_row.param[1], false);

            if self.start_time == self.logic_time {
                if attr_type == EmAttribute::EmAttributeHp {
                    self.p_holder.set_hp(self.p_holder.get_hp());
                    p_impact_info.hurts[index][0] = p_row.param[1] as i32 * -1;
                }
                if attr_type == EmAttribute::EmAttributeMp {
                    p_impact_info.mp[index] = p_row.param[1] as i32 * -1;
                }
            } else if self.start_time < self.logic_time {
                let p_round_info = p_fight_cell.get_round_info();
                let mut p_fobjectinfo = p_round_info.get_fobjectinfo_by_guid(self.p_holder.get_guid());

                if p_fobjectinfo.is_some() {
                    let obj = p_fobjectinfo.as_mut().unwrap();
                    for (i, id) in obj.m_impact_list.iter().enumerate() {
                        if *id == self.impact_id {
                            if attr_type == EmAttribute::EmAttributeHp {
                                obj.m_impact_hurt[i] = p_row.param[1] as i32 * -1;
                            }
                            if attr_type == EmAttribute::EmAttributeMp {
                                obj.m_impact_mp[i] = p_row.param[1] as i32 * -1;
                            }
                            break;
                        }
                    }
                }
            }
        }
        self.logic_time += 1;
    }

    /// # 5=debuff削弱类
    /// * 逻辑参数1：改变的英雄属性id，读取AttributeData.tab表。
    /// * 逻辑参数2：改变的具体数值
    /// * 逻辑参数3：持续时间，单位回合（10）
    /// * 最终可实现的效果如敌人攻击减少X点持续10回合。
    pub fn impact_logic5(&mut self) {
        let p_attack_info = self.p_caster.get_attack_info();

        if !p_attack_info.is_valid() {
            return;
        }

        let res_skill_attack = p_attack_info.get_skill_attack(self.skill_id);

        if res_skill_attack.is_none() {
            return;
        }
        let p_skill_attack = res_skill_attack.unwrap();

        if !p_skill_attack.is_valid() {
            return;
        }

        let mut p_impact_info = p_skill_attack.get_impact_info(self.impact_id);

        if !p_impact_info.is_valid() {
            return;
        }

        let target_index = p_impact_info.get_target_index(self.p_holder.get_guid());
        if None == target_index {
            return;
        }

        let index = target_index.unwrap();

        let p_fight_cell = self.p_holder.get_fight_cell();
        let n_physic_attack = self.p_caster.get_physic_attack();
        let p_row = self.get_impact_row();

        if self.logic_time <= self.life {
            let n = p_row.param[0] as u8;
            let tp = EmAttribute::try_from(n);
            let attr_type = tp.unwrap();
            let n_attr_type = attr_type.into_usize();
            self.p_holder.change_effect(n_attr_type, p_row.param[1] * -1, false);

            if self.start_time == self.logic_time {
                if attr_type == EmAttribute::EmAttributeHp {
                    self.p_holder.set_hp(self.p_holder.get_hp());
                    p_impact_info.hurts[index][0] = p_row.param[1] as i32;
                }
                if attr_type == EmAttribute::EmAttributeMp {
                    p_impact_info.mp[index] = p_row.param[1] as i32;
                }
            } else if self.start_time < self.logic_time {
                let p_round_info = p_fight_cell.get_round_info();
                let mut p_fobjectinfo = p_round_info.get_fobjectinfo_by_guid(self.p_holder.get_guid());

                if p_fobjectinfo.is_some() {
                    let obj = p_fobjectinfo.as_mut().unwrap();
                    for (i, id) in obj.m_impact_list.iter().enumerate() {
                        if *id == self.impact_id {
                            if attr_type == EmAttribute::EmAttributeHp {
                                obj.m_impact_hurt[i] = p_row.param[1] as i32;
                            }
                            if attr_type == EmAttribute::EmAttributeMp {
                                obj.m_impact_mp[i] = p_row.param[1] as i32;
                            }
                            break;
                        }
                    }
                }
            }
        }
        self.logic_time += 1;
    }

    /// # 6=单次魔法攻击
    /// * 逻辑参数1：从英雄魔法攻击中取得的倍率（例：150）
    /// * 逻辑参数2：额外增加的魔法伤害（例：50）
    /// * 如英雄魔法攻击为100，则最终的技能魔法攻击=英雄魔法攻击（100）*逻辑参数1（150/100）+逻辑参数2（50）
    pub fn impact_logic6(&mut self) {
        let p_attack_info = self.p_caster.get_attack_info();

        if !p_attack_info.is_valid() {
            return;
        }

        let res_skill_attack = p_attack_info.get_skill_attack(self.skill_id);

        if res_skill_attack.is_none() {
            return;
        }
        let p_skill_attack = res_skill_attack.unwrap();

        if !p_skill_attack.is_valid() {
            return;
        }

        let mut p_impact_info = p_skill_attack.get_impact_info(self.impact_id);

        if !p_impact_info.is_valid() {
            return;
        }

        let target_index = p_impact_info.get_target_index(self.p_holder.get_guid());
        if None == target_index {
            return;
        }

        let index = target_index.unwrap();

        let p_fight_cell = self.p_holder.get_fight_cell();
        let n_magic_attack = self.p_caster.get_magic_attack();
        let p_row = self.get_impact_row();

        let mut n_attack = n_magic_attack as f32 * p_row.param[0] as f32 * 0.01 + p_row.param[1] as f32;
        if p_fight_cell.get_fight_type() == EmTypeFight::EmTypeFightStair && self.p_caster.is_attacker() {
            n_attack += n_attack * p_fight_cell.get_plus_atk() as f32 / 100.0;
        }

        let n_defend = self.p_holder.get_magic_defend() as f32;
        let n_decay = self.p_holder.get_magic_hurt_decay() as f32;
        //本次魔法攻击伤害=(自身当前经过连击计算后的魔法攻击-目标魔法防御)*(1-目标魔法伤害减免/100)
        let mut n_damage = calcdamage(n_attack, n_defend, n_decay);
        chkmin(&mut n_damage, 0);

        self.p_holder.set_hp(self.p_holder.get_hp() - n_damage);

        p_impact_info.hurts[index][0] = n_damage as i32;

        //连击
        let f_con_att_hurt = self.p_caster.get_con_atk_hurt() as f32 * 0.01;
        for i in 1..=self.con_att_times {
            n_attack = n_attack * f_con_att_hurt;
            n_damage = calcdamage(n_attack, n_defend, n_decay);
            chkmin(&mut n_damage, 0);

            self.p_holder.set_hp(self.p_holder.get_hp() - n_damage);

            p_impact_info.hurts[index][i as usize] = n_damage as i32;
        }
    }
}