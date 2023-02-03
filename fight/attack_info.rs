use crate::fight::skill_attack::SkillAttack;

/// 出手信息
#[derive(Debug, Default, Clone)]
pub struct AttackInfo {
    /// 对象guid
    pub cast_guid: u32,
    /// 魔法攻击还是普通攻击
    pub b_skilled: bool,
    /// 魔法攻击
    pub skill_attack_list: Vec<SkillAttack>,
    /// 如果是普通攻击先进行装备技能攻击
    /// 普通攻击
    pub skill_attack_count: u32,
    /// 普通攻击目标
    pub skill_target: u32,
    /// 是否命中
    pub b_hit: bool,
    /// 是否暴击
    pub b_strike: bool,
    /// 伤害值
    pub hurt: u32,
    /// 是否有反击
    pub b_back_attack: bool,
    /// 反击伤害
    pub back_attack_hurt: u32,
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



    pub fn alloc_skill_attack(&mut self) -> Option<&SkillAttack> {
        let index = self.skill_attack_count;
        self.skill_attack_count += 1;
        self.skill_attack_list.get(index as usize)
    }
}
