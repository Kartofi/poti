use rand::Rng;

static WORDSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn gen_id() -> String {
    let mut output = String::new();
    let mut rng = rand::rng();

    for i in 0..4 {
        for _ in 0..4 {
            output.push(
                WORDSET.chars()
                    .nth(rng.random_range(0..WORDSET.len()))
                    .unwrap()
            );
        }
        if i != 3 {
            output.push('-');
        }
    }

    return output;
}
