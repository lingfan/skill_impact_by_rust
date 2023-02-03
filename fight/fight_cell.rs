use crate::fight::fight_object::FightObjList;
use crate::fight::{EmTypeFight, FightInfo, FightRoundInfo};
use crate::fight::skill_const::{MAX_FIGHT_ROUND, MAX_MATRIX_CELL_COUNT};

#[derive(Debug, Default, Clone)]
pub struct FightCell {
    attack_list: FightObjList,
    defence_list: FightObjList,
    //战斗信息提取
    fight_info: FightInfo,
    round_info: FightRoundInfo,
    fight_type: EmTypeFight,
    //攻加成
    plus_atk: u32,
}


impl FightCell {
    pub fn new() {}


    pub fn init(&self) -> bool {
        true
    }
    pub fn clean_up(&self) {}


    pub fn get_attack_list(&self) -> &FightObjList {
        &self.attack_list
    }
    pub fn get_defence_list(&self) -> &FightObjList {
        &self.defence_list
    }

    pub fn init_attack_list(&self) {}

    pub fn init_defence_list(&self, nDefendType: u32) {}

    pub fn fight(&self) -> bool {
        self._init_fight_info();
        for nround in 1..=MAX_FIGHT_ROUND {
            if self.is_over() {
                if self.attack_list.get_active_count() > 0 {
                    self.fight_info.set_win(true);
                } else {
                    self.fight_info.set_win(false);
                }
                return true;
            }

            self._init_round_info();


            self.attack_list.impact_heart_beat(nRound);
            self.defence_list.impact_heart_beat(nRound);
            self.attack_list.heart_beat(nRound);
            self.defence_list.heart_beat(nRound);
            self.fight_info.AddRoundInfo(m_RoundInfo);
        }
        true
    }
    pub fn is_over(&self) -> bool {
        true
    }
    pub fn is_win(&self) -> bool {
        true
    }

    pub fn _init_fight_info(&self) {}

    pub fn _init_round_info(&self) {}

    pub fn fix_data(&mut self, plus_atk: u32, plus_def: u32, plus_hp: u32) {
        if plus_atk > 0 {
            self.plus_atk = plus_atk;
        }

        if plus_def > 0 {
            for i in 0..MAX_MATRIX_CELL_COUNT {
                let  p_fight_obj = self.attack_list.get_fight_object(i);
                if p_fight_obj.is_active() {
                    let mut fight_db_data =  p_fight_obj.clone().fight_db_data;
                    fight_db_data.physic_defence += fight_db_data.physic_defence* plus_def /100;
                }
            }
        }

        if plus_hp > 0 {
            for i in 0..MAX_MATRIX_CELL_COUNT {
                let p_fight_obj = self.attack_list.get_fight_object(i);
                if p_fight_obj.is_active() {
                    let mut fight_db_data =  p_fight_obj.clone().fight_db_data;
                    fight_db_data.hp += fight_db_data.hp* plus_hp /100 as u32;
                    fight_db_data.max_hp += fight_db_data.max_hp* plus_hp /100 as u32;
                }
            }
        }
    }

    pub fn set_fight_type(&mut self, fight_type: u8) {
        let tp = EmTypeFight::try_from(fight_type).unwrap();
        self.fight_type = tp;
    }

    pub fn get_fight_type(&self) -> EmTypeFight {
        self.fight_type.clone()
    }

    pub fn set_plus_atk(&mut self, plus_atk: u32) {
        self.plus_atk = plus_atk;
    }

    pub fn get_plus_atk(&self) -> u32 {
        self.plus_atk
    }

    pub fn get_round_info(&self) -> &FightRoundInfo {
        &self.round_info
    }

    pub fn get_fight_info(&self) -> &FightInfo {
        &self.fight_info
    }
}