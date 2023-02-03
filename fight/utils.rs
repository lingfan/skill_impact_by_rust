use rand::Rng;
//属性值计算公式
pub fn calcattr1(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32) -> f32 {
    (a + b * c * 0.01) * (1.0 + d * 0.01 + e * 0.01) + f + g
}

pub fn calcattr2(a: f32, b: f32, c: f32) -> f32 {
    a + b + c
}

pub fn calcattr3(a: f32, b: f32, c: f32) -> u32 {
    ((1.0 - (1.0 - a * 0.01) * (1.0 - b * 0.01) * (1.0 - c * 0.01)) * 100.0) as u32
}

pub fn calcdamage(a: f32, b: f32, c: f32) -> u32 {
    ((a - b) * (1.0 - c * 0.01)) as u32
}

pub fn chkmin(a: &mut u32, b: u32) {
    if *a < b {
        *a = b
    }
}

pub fn getskillgroup(a: u32) -> u32 {
    a / 100
}

pub fn getskilllevel(a: u32) -> u32 {
    a % 100
}

pub fn get_rand(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let n_rand = rng.gen_range(min..max);
    n_rand
}
