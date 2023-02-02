use crate::fight::skill_const::{MAX_CONATTACK_TIMES, MAX_MATRIX_CELL_COUNT};

#[derive(Debug, Default, Clone)]
pub struct ImpactInfo {
    pub skill_id: u32,
    pub impact_id: u32,
    pub target_list: Vec<u32>,
    pub target_count: u32,
    pub con_atk_times: u32,
    pub hurts: [[i32; MAX_CONATTACK_TIMES as usize]; MAX_MATRIX_CELL_COUNT as usize],
    /// 本次技能带给的魔法量变化 >0 减蓝 < 0 加蓝
    pub mp: [i32; MAX_MATRIX_CELL_COUNT as usize],
}

impl ImpactInfo {
    pub fn new() -> ImpactInfo {
        ImpactInfo::default()
    }

    pub fn is_valid(&self) -> bool {
        self.impact_id > 0
    }

    pub fn get_target_index(&self, target_guid: u32) -> Option<usize> {
        if !target_guid > 0 {
            return None;
        }
        for (i, v) in self.target_list.iter().enumerate() {
            if *v == target_guid {
                return Some(i as usize);
            }
        }

        None
    }
}