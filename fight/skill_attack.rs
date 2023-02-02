use crate::fight::impact_info::ImpactInfo;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SkillAttack {
    pub skill_id: u32,
    /// 技能ID
    pub skill_target: u32,
    /// 技能目标
    pub cost_mp: u32,
    /// 消耗魔法量
    pub impact: u32,
    /// impact列表
    /// impact个数
    pub impact_count: u32,
}

impl SkillAttack {
    pub fn new() -> SkillAttack {
        SkillAttack::default()
    }

    pub fn is_valid(&self) -> bool {
        self.skill_id > 0
    }

    pub fn get_impact_info(&self, impact_id: u32) -> ImpactInfo {
        ImpactInfo::default()
    }
}
