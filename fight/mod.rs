use num_enum::TryFromPrimitive;
use num_enum::IntoPrimitive;
use std::convert::TryFrom;
use crate::fight::fight_round_info::FightRoundInfo;
use crate::fight::skill_const::{MAX_CONATTACK_TIMES, MAX_IMPACT_NUMBER, MAX_MATRIX_CELL_COUNT};

mod fight_cell;
mod fight_object;
mod impact;
mod skill;
mod attack_info;
mod skill_attack;
mod impact_info;
mod fight_round_info;
mod utils;





#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum EmTypeFight{
    #[default]
    EmTypeFightNormal = 0,
    EmTypeFightStair,
    //单排
    EmTypeFightCount,
}

#[derive(Debug, Default, Clone)]
pub struct FightDBData {
    guid: u32,
    /// 对象guid
    table_id: u32,
    /// 表格id
    r#type: u32,
    /// 英雄出处
    quality: u32,
    /// 品质
    matrix_id: u32,
    /// 阵法Id
    profession: u32,
    /// 职业
    level: u32,
    /// 等级
    hp: u32,
    /// 血量
    mp: u32,
    /// 魔法值
    skill_count: u32,
    /// 技能数量
    skill: u32,
    /// 技能列表
    equip_skill_count: u32,
    equip_skill: u32,
    /// 装备技能列表

    /// 战斗属性
    physic_attack: u32,
    /// 物理攻击
    magic_attack: u32,
    /// 魔法攻击，
    physic_defence: u32,
    /// 物理防御，
    magic_defence: u32,
    /// 魔法防御，
    max_hp: u32,
    /// 最大生命
    max_mp: u32,
    /// 最大魔法值
    hit: u32,
    /// 命中值，
    dodge: u32,
    /// 闪避值，
    strike: u32,
    /// 暴击值，
    strike_hurt: u32,
    /// 暴击伤害
    continuous: u32,
    /// 连击值
    con_atk_hurt: u32,
    /// 连击伤害
    con_atk_times: u32,
    /// 连击次数
    back_attack: u32,
    /// 反击值
    back_atk_hurt: u32,
    /// 反击伤害
    attack_speed: u32,
    /// 攻击速度，
    physic_hurt_decay: u32,
    /// 物理减免
    magic_hurt_decay: u32,
    /// 魔法减免

    ///                 DamageReduce:u32,				/// 总伤害减免（百分比）
    ///                 nReduceCriticalDamage:u32,	/// 减免暴击伤害
    ///                 nExtraHurt:u32,				/// 附加伤害
    /// /新增属性
    exp: u32,
    /// 经验
    grow_rate: u32,
    /// 成长
    bear_point: u32,
    /// 负重
    equip: u32,
    /// 装备ID
    /// 颜色
    color: u32,

}

#[derive(Debug, Default, Clone)]
pub struct FObjectInfo {
    m_guid: u32,
    //对象guid
    m_hp: u32,
    //血量
    m_max_hp: u32,
    //最大血量
    m_mp: u32,
    //魔法值
    m_max_mp: u32,
    //最大魔法
    m_fight_distance: u32,
    //战斗条长度
    m_attack_speed: u32,
    //速度
    m_end_distance: u32,
    //最后位置
    m_impact_count: u32,
    m_impact_list: [u32; MAX_IMPACT_NUMBER as usize],
    //身上impact
    m_impact_hurt: [i32; MAX_IMPACT_NUMBER as usize],
    //持续impact伤害 + 掉血 - 加血
    m_impact_mp: [i32; MAX_IMPACT_NUMBER as usize],    //持续impact 蓝  + 掉蓝 - 加蓝
}

#[derive(Debug, Default, Clone)]
pub struct FightInfo {
    //静态数据
    attack_object_data: Vec<FObjectData>,
    attack_object_count: u32,
    defend_object_data: Vec<FObjectData>,
    defend_object_count: u32,
    //防守方类型 0 怪物 1 人
    defend_type: u32,
    round_info: Vec<FightRoundInfo>,
    //总回合
    rounds: u32,
    //战斗条长度
    max_fight_distance: u32,
    //挑战者是否胜利
    b_win: bool,
}

#[derive(Debug, Default, Clone)]
pub struct FObjectData {
    //对象guid
    guid: u32,
    //表格id
    table_id: u32,
    //品质
    quality: u32,
    //颜色
    color: u32,
    //职业
    profession: u32,
    //等级
    level: u32,
    //位置
    matrix_id: u32,
}

//技能
#[allow(dead_code)]
pub mod skill_const {
    //技能数量
    pub const MAX_SKILL_NUM: u32 = 2;
    //技能最高等级
    pub const MAX_SKILL_LEVEL: u32 = 7;
    //技能升级最多使用道具
    pub const MAX_SKILLUP_ITEM_COUNT: u32 = 5;
    //技能最大选择目标
    pub const MAX_MATRIX_CELL_COUNT: u32 = 6;
    //冷却数量
    pub const MAX_SKILL_COOLDOWN_NUMBER: u32 = 10;
    //impact数量
    pub const MAX_IMPACT_NUMBER: u32 = 20;
    //强化类buff数量
    pub const MAX_BUFF_NUMBER: u32 = 10;
    //削弱类buff数量
    pub const MAX_DEBUFF_NUMBER: u32 = 10;
    //最大连击次数+1
    pub const MAX_CONATTACK_TIMES: u32 = 4;
    //最大战斗回合
    pub const MAX_FIGHT_ROUND: u32 = 128;
    //最大技能所带impact
    pub const MAX_SKILL_IMPACT_COUNT: u32 = 4;
    //impact参数数量
    pub const MAX_IMPACT_LOGIC_PARAM_COUNT: u32 = 4;
    //最大离线战斗数据池
    pub const MAX_OFFLINE_FIGHTDATA_SIZE: u32 = 1000;
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum EmAttribute {
    #[default]
    EmAttributeInvalid = 0,
    ///最大生命
    EmAttributeMaxHp = 1,
    ///移动速度
    EmAttributeMoveSpeed = 2,
    ///攻击速度
    EmAttributeAttackSpeed,
    ///物理攻击
    EmAttributePhysicAttack,
    ///物理防御
    EmAttributePhysicDefence,
    ///命中点数
    EmAttributeHit,
    ///闪避点数
    EmAttributeDodge,
    ///暴击
    EmAttributeStrike,
    ///连击
    EmAttributeContinuous,
    ///反击
    EmAttributeBackAttack,
    ///连击次数
    EmAttributeContinuousTimes,
    ///连击伤害
    EmAttributeHurtContinuous,
    ///反击伤害
    EmAttributeHurtBackAttack,
    ///暴击伤害
    EmAttributeHurtStrike,
    ///物理伤害减免
    EmAttributePhysicHurtDecay,
    ///暴击伤害减免
    EmAttributeStrikeHurtDecay,
    ///附加伤害
    EmAttributeHurtExtra,
    ///普通攻击伤害
    EmAttributeHurtPhysic,
    ///魔法攻击
    EmAttributeMagicAttack,
    ///魔法防御
    EmAttributeMagicDefence,
    ///魔法伤害减免
    EmAttributeMagicHurtDecay,
    ///最大魔法值
    EmAttributeMaxMp,
    ///攻击速度百分比
    EmAttributePercentAttackSpeed,
    ///物理攻击百分比
    EmAttributePercentPhysicAttack,
    ///魔法攻击百分比
    EmAttributePercentMagicAttack,
    ///物理防御百分比
    EmAttributePercentPhysicDefence,
    ///魔法防御百分比
    EmAttributePercentMagicDefence,
    ///最大生命值百分比
    EmAttributePercentMaxHp,
    ///最大魔法值百分比
    EmAttributePercentMaxMp,
    ///等级
    EmAttributeLevel,
    ///血量
    EmAttributeHp,
    ///魔法值
    EmAttributeMp,
    ///当前经验
    EmAttributeCurrentExp,
    ///行动力
    EmAttributeAction,
    EmAttributeNumber,
}

impl EmAttribute {
    pub fn into_u8(self) -> u8 {
        let res: u8 = self.into();
        res
    }

    pub fn into_u32(self) -> u32 {
        let res = self.into_u8() as u32;
        res
    }

    pub fn into_usize(self) -> usize {
        let res = self.into_u8() as usize;
        res
    }
}

