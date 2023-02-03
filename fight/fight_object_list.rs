use crate::fight::fight_object::FightObject;
use crate::fight::FightDB;
use crate::fight::skill_const::MAX_MATRIX_CELL_COUNT;

#[derive(Debug, Default, Clone)]
pub struct FightObjList {
    owner_guid: u32,
    object_list: Vec<FightObject>,
    object_list1:[FightObject;MAX_MATRIX_CELL_COUNT]
}


impl FightObjList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_active_count(&self) -> u32{
        let mut n_count = 0;
        for i in 0..self.object_list.len() {
            if self.object_list[i].is_active() {
                n_count += 1;
            }
        }
        n_count
    }

    pub fn get_inactive_count(&self) -> u32 {
        let mut n_count = 0;
        for i in 0..MAX_MATRIX_CELL_COUNT {
            if !self.object_list[i].is_active() {
                n_count += 1;
            }
        }
        n_count
    }

    pub fn get_owner_guid(&self) -> u32 {
        self.owner_guid
    }
    pub fn set_owner_guid(&mut self, guid: u32) {
        self.owner_guid = guid;
    }


    pub fn fill_object(&self, index: u32, object: FightObject) {
        object.set_matrix_id(index);
        self.object_list[index] = object;
    }


    pub fn init(&self, fightDB: &FightDB) {}


    pub fn get_fight_object(&mut self, index: usize) -> &FightObject {
        let b = &mut self.object_list;
        let c = b.get(index);
        let d = c.unwrap();
        d
    }

    pub fn impact_heart_beat(&self, u_time: u32) {
        for i in 1..MAX_MATRIX_CELL_COUNT {
            if self.object_list[i].is_active() {
                self.object_list[i].clear_impact_effect();
                self.object_list[i].impact_heart_beat(u_time);
            }
        }
    }

    pub fn heart_beat(&self, u_time: u32) {
        for i in 1..MAX_MATRIX_CELL_COUNT {
            if self.object_list[i].is_active() {
                self.object_list[i].heartbeat(u_time);
                let p_attack_info = self.object_list[i].get_attack_info();
                if p_attack_info.is_active() {
                    let p_fight_cell = self.object_list[i].get_fight_cell();
                    let p_round_info = p_fight_cell.get_round_info();
                    p_round_info.add_attack_info(p_attack_info);
                }
            }
        }
    }

    pub fn clear_impact_effect(&self) {
        for i in 1..MAX_MATRIX_CELL_COUNT {
            if self.object_list[i].is_active() {
                self.object_list[i].clear_impact_effect();
            }
        }
    }
}
