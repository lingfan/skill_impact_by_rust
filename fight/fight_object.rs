use std::borrow::BorrowMut;
use crate::fight::{EmAttribute, EmImpactLogic, EmTypeImpactLogic, FightDBData};
use crate::fight::attack_info::AttackInfo;
use crate::fight::EmImpactLogic::{EmImpactLogic0, EmImpactLogic1, EmImpactLogic6};
use crate::fight::EmTypeImpactLogic::EmTypeImpactLogicSingle;
use crate::fight::fight_cell::FightCell;
use crate::fight::fight_object_list::FightObjList;
use crate::fight::impact::{Impact, TableRowImpact};
use crate::fight::skill::Skill;
use crate::fight::skill_const::{MAX_IMPACT_NUMBER, MAX_MATRIX_CELL_COUNT};


#[derive(Debug, Default,Clone)]
pub struct FightObject {
    pub fight_db_data: FightDBData,
    /// 是否为攻击方
    is_attacker: bool,
    /// 战斗单元
    fight_cell: FightCell,
    /// 主动技能列表
    skill_list: Vec<Skill>,
    /// 普通攻击
    common_skill: Skill,
    equip_skill_count: u32,
    /// 装备技能
    equip_skill_list: Vec<Skill>,
    /// buff列表(所有buffer)
    implicit_list: [Impact;MAX_IMPACT_NUMBER as usize],
    /// 技能影响
    implicit_effect: [i32;30],
    /// 战斗条长度
    fight_distance: u32,
    /// 出手信息
    attack_info: AttackInfo,
}

impl FightObject {
    pub fn new() -> FightObject {
        FightObject::default()
    }

    pub fn get_fight_db_data_mut(&mut self) -> &mut FightDBData {
        let b = &mut self.fight_db_data;
        b
    }

    pub fn is_attacker(&self) -> bool{
        self.is_attacker
    }
    pub fn is_active(&self) -> bool{
        if self.get_hp() > 0 {
           return true;
        }
        false
    }


    pub fn set_attacker(&mut self, b_attacker:bool) {
        self.is_attacker = b_attacker
    }
    pub fn get_attack_info(&self) -> AttackInfo {
        self.attack_info.clone()
    }

    pub fn get_guid(&self) -> u32 {
        self.fight_db_data.guid
    }

    pub fn get_fight_cell(&self) -> FightCell {
        self.fight_cell.clone()
    }
    pub fn get_enemy_list(&self) -> FightObjList {
        FightObjList::default()
    }

    pub fn get_owner_list(&self) -> FightObjList {
        FightObjList::default()
    }

    /// 最大攻击速度
    pub fn get_attack_speed(&self) -> u32 {
        0
    }
     /// 物理攻击
    pub fn get_physic_attack(&self) -> u32 { 0 }
    ///  /// 魔法攻击
    pub fn get_magic_attack(&self) -> u32 { 0 }
    /// /// 物理防御
    pub fn get_physic_defend(&self) -> u32 { 0 }
    /// /// 魔法防御
    pub fn get_magic_defend(&self) -> u32 { 0 }
    /// 物理减免
    pub fn get_physic_hurt_decay(&self) -> u32 { 0 }
    /// 魔法减免
    pub fn get_magic_hurt_decay(&self) -> u32 { 0 }
    /// 连击
    pub fn get_continuous(&self) -> u32 { 0 }
    /// 连击次数
    pub fn get_con_atk_times(&self) -> u32 { 0 }
    /// 连击伤害
    pub fn get_con_atk_hurt(&self) -> u32 { 0 }
    /// 暴击
    pub fn get_strike(&self) -> u32 { 0 }
    /// 暴击伤害
    pub fn get_strike_hurt(&self) -> u32 { 0 }
    /// 伤害浮动
    pub fn get_floating_hurt(&self) -> u32 { 0 }

    /// 命中
    pub fn get_hit(&self) -> u32 { 0 }
    /// 闪避
    pub fn get_dodge(&self) -> u32 { 0 }
    /// 反击
    pub fn get_attack_back(&self) -> u32 { 0 }
    /// 反击伤害
    pub fn get_back_att_hurt(&self) -> u32 { 0 }


    pub fn get_hp(&self) -> u32 {
        self.fight_db_data.hp
    }

    pub fn set_hp(&mut self, n_val: u32) { self.fight_db_data.hp = n_val; }

    pub fn get_mp(&self) -> u32 {
        self.fight_db_data.mp
    }

    pub fn set_mp(&mut self, n_val: u32) { self.fight_db_data.mp = n_val; }





    pub fn change_effect(&mut self, n_attr_type:usize, n_value:i32, b_remove:bool)
    {
        if b_remove {
            self.implicit_effect[n_attr_type] -= n_value;
        }        else        {
            self.implicit_effect[n_attr_type] += n_value;
        }
    }

    pub fn get_matrix_id(&self) -> usize {
        self.fight_db_data.matrix_id as usize
    }

    pub fn get_impact_logic_type(&self, nImpactID:u32) -> Option<EmTypeImpactLogic> {
        let p_row_impact_new = TableRowImpact::default();
        let p_logic_id = EmImpactLogic::try_from( p_row_impact_new.logic_id as u8 ).unwrap();
        match p_logic_id {
             EmImpactLogic0 | EmImpactLogic1 |  EmImpactLogic6 => {
                return Some(EmTypeImpactLogicSingle)
            }
            _ => {

            }
        }
        None
    }

    pub fn add_impact(&self, n_impact_id:u32, conAttTimes:u32, nRound:u32, pCaster: Box<FightObject>, nSkillID:u32){
        let p_row_impact_new = TableRowImpact::default();
        let o_logic_type = self.get_impact_logic_type(n_impact_id);

        if o_logic_type.is_none() {
            return;
        }

        let logic_type = o_logic_type.unwrap();

        if logic_type == EmTypeImpactLogicSingle {

            return;
        }

        if p_row_impact_new.impact_mutex_id >= 0 {
            // for i in EmTypeImpactLogic::to_vec() {
            //
            // }
        }

    }

    pub fn clear_impact_effect(&mut self) {
        for i in EmAttribute::iter() {
            self.implicit_effect[i] = 0;
        }
    }

    pub fn impact_heart_beat(&mut self, uTime:u32) {
        for i in 0..MAX_IMPACT_NUMBER {
            if self.implicit_list[i].is_valid() {
                self.implicit_list[i].heartbeat(uTime);
            }
        }
    }
}
