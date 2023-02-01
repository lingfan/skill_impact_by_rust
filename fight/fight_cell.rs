use crate::fight::fight_object::FightObjList;
use crate::fight::{EmTypeFight, FightInfo, FightRoundInfo};

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

    pub fn get_fight_type(&self) -> EmTypeFight {
        EmTypeFight::EmTypeFightNormal
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
}