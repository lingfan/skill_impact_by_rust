use crate::fight::{AttackInfo, FightDBData};
use crate::fight::fight_cell::FightCell;
use crate::fight::impact::Impact;
use crate::fight::object_define::EmAttribute::EmAttributeNumber;
use crate::fight::skill::Skill;
use crate::fight::skill_const::MAX_IMPACT_NUMBER;


#[derive(Debug, Default,Clone)]
pub struct FightObject {
    fight_db_data: FightDBData,
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
    implicit_effect: [u32;30],
    /// 战斗条长度
    fight_distance: u32,
    /// 出手信息
    attack_info: AttackInfo,
}

#[derive(Debug, Default,Clone)]
pub struct FightObjList {
    owner_guid: u32,
    object_list: Vec<FightObject>,
}


impl FightObject {
    pub fn new() -> FightObject {
        FightObject::default()
    }

    pub fn is_attacker(&self) -> bool{
        self.is_attacker
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
    pub fn get_enemy_list() -> FightObjList {
        FightObjList::default()
    }

    pub fn get_owner_list() -> FightObjList {
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
    pub fn get_con_att_times(&self) -> u32 { 0 }
    /// 连击伤害
    pub fn get_con_att_hurt(&self) -> u32 { 0 }
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


    pub fn get_hp(&self) -> u32 { 0 }

    pub fn set_hp(&mut self, hp: u32) { self.fight_db_data.hp = hp; }

    pub fn change_effect(&mut self, n_attr_type:usize, n_value:u32, b_remove:bool)
    {
        if b_remove {
            self.implicit_effect[n_attr_type] -= n_value;
        }        else        {
            self.implicit_effect[n_attr_type] += n_value;
        }
    }
}
