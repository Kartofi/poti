use rand::Rng;

pub fn gen_id() -> String {
    let mut rng = rand::rng();
    let id: u64 = rng.random();
    return id.to_string();
}
