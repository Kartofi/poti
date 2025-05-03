use rand::{ rng, Rng };

static WORDSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

static ID_SEGMENTS: usize = 4;
static ID_SEGMENTS_WIDTH: usize = 6;

pub fn gen_id() -> String {
    let mut output = String::new();
    let mut rng = rand::rng();

    for i in 0..ID_SEGMENTS {
        for _ in 0..ID_SEGMENTS_WIDTH {
            output.push(
                WORDSET.chars()
                    .nth(rng.random_range(0..WORDSET.len()))
                    .unwrap()
            );
        }
        if i != ID_SEGMENTS - 1 {
            output.push('-');
        }
    }

    return output;
}
