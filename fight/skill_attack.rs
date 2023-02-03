use crate::fight::impact_info::ImpactInfo;
use crate::fight::skill_const::MAX_SKILL_IMPACT_COUNT;

#[derive(Debug, Default, Clone)]
pub struct SkillAttack {
    /// 技能ID
    pub skill_id: u32,
    /// 技能目标
    pub skill_target: u32,
    /// 消耗魔法量
    pub cost_mp: u32,
    /// impact列表
    pub impact:Vec<ImpactInfo>,
    /// impact个数
    pub impact_count: usize,
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


    pub fn  add_import_info(&mut self, info: ImpactInfo)  {
        self.impact.push(info);
    }
}
