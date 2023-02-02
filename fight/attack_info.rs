use crate::fight::skill_attack::SkillAttack;

/// 出手信息
#[derive(Debug, Default, Clone)]
pub struct AttackInfo {
    /// 对象guid
    cast_guid: u32,
    /// 魔法攻击还是普通攻击
    b_skilled: u32,
    /// 魔法攻击
    skill_attack_list: Vec<SkillAttack>,
    /// 如果是普通攻击先进行装备技能攻击
    /// 普通攻击
    skill_attack_count: u32,
    /// 普通攻击目标
    skill_target: u32,
    /// 是否命中
    b_hit: u32,
    /// 是否暴击
    b_strike: u32,
    /// 伤害值
    hurt: u32,
    /// 是否有反击
    b_back_attack: u32,
    /// 反击伤害
    back_attack_hurt: u32,
}

impl AttackInfo {
    pub fn new() -> AttackInfo {
        AttackInfo::default()
    }

    pub fn is_valid(&self) -> bool {
        self.cast_guid > 0
    }

    pub fn get_skill_attack(&self, skill_id: u32) -> Option<&SkillAttack> {
        for v in self.skill_attack_list.iter() {
            if v.skill_id == skill_id {
                return Some(v);
            }
        }
        None
    }
}
