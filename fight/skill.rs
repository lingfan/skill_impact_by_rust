

struct CoolDown {
    id: u32,
    end_time: u32,
}

struct CoolDownList {
    count: u32,
    cd_list: Vec<CoolDown>,
}

#[derive(Debug, Default, Clone)]
pub struct Skill {}
