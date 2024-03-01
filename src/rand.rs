use rand::{Rng, thread_rng, rngs::ThreadRng, distributions::Uniform};

pub fn rand_usize(min: usize, max: usize) -> usize {
    let mut rng: ThreadRng = thread_rng();
    rng.gen_range(min..=max)
}

pub fn rand_u32(min: u32, max: u32) -> u32 {
    let mut rng: ThreadRng = thread_rng();
    rng.gen_range(min..=max)
}

pub fn rand_string(min: usize, max: usize) -> String {
    let characters: &str = "abcdefghijklm nopqrstuvwxyz ABCDEFGHIJKLM NOPQRSTUVWXYZ 0123456789 ,.!?";
    let mut rng: ThreadRng = thread_rng();
    let len: usize = rand_usize(min, max);
    (0..len).map(|_| {
        let idx = rng.gen_range(0..characters.len());
        characters.chars().nth(idx).unwrap()
    })
    .collect()
}

pub fn rand_alphabets(min: usize, max: usize) -> String {
    let characters: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng: ThreadRng = thread_rng();
    let len: usize = rand_usize(min, max);
    (0..len).map(|_| {
        let idx = rng.gen_range(0..characters.len());
        characters.chars().nth(idx).unwrap()
    })
    .collect()
}

pub fn rand_f32(min: f32, max: f32, precision: usize) -> f32 {
    let range = Uniform::new_inclusive(min, max);
    let mut rng = rand::thread_rng();
    let rnd: f32 = rng.sample(range);
    let rnd_str = format!("{:.*}", precision, rnd);
    rnd_str.parse::<f32>().unwrap()
}