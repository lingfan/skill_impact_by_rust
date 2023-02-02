use crate::fight::attack_info::AttackInfo;
use crate::fight::FObjectInfo;

#[derive(Debug, Default, Clone)]
pub struct FightRoundInfo {
    attack_object_info: Vec<FObjectInfo>,
    attack_object_count: u32,
    defend_object_info: Vec<FObjectInfo>,
    defend_object_count: u32,
    attack_info: Vec<AttackInfo>,
    //出手数据
    attack_info_count: u32,
}

impl FightRoundInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_fobjectinfo_by_guid(&self, guid: u32) -> Option<Box<FObjectInfo>> {
        if guid <= 0 {
            return None;
        }

        for v in self.attack_object_info.iter() {
            if v.m_guid == guid {
                return Some(Box::new(v.clone()));
            }
        }

        for v in self.defend_object_info.iter() {
            if v.m_guid == guid {
                return Some(Box::new(v.clone()));
            }
        }

        None
    }
}