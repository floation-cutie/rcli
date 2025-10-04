use rand::prelude::*;
use rand::rng;
use zxcvbn::zxcvbn;

const UPPER: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &str = "abcdefghijkmnopqrstuvwxyz";
const NUMBER: &str = "123456789";
const SPECIAL: &str = "!@#$%^&*_";

// random generate password
// we should decouple the logic from the cli parsing
// consider opts length、uppercase、lowercase、numbers and symbols
pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    special: bool,
) -> anyhow::Result<String> {
    let mut password = Vec::with_capacity(length as usize);
    let mut charset = String::new();
    let mut rng = rng();
    if uppercase {
        charset.push_str(UPPER);
        password.push(UPPER.chars().nth(rng.random_range(0..UPPER.len())).unwrap());
    }
    if lowercase {
        charset.push_str(LOWER);
        password.push(LOWER.chars().nth(rng.random_range(0..LOWER.len())).unwrap());
    }
    if number {
        charset.push_str(NUMBER);
        password.push(NUMBER.chars().nth(rng.random_range(0..NUMBER.len())).unwrap());
    }
    if special {
        charset.push_str(SPECIAL);
        password.push(SPECIAL.chars().nth(rng.random_range(0..SPECIAL.len())).unwrap());
    }

    // when password length is less than the number of selected types, return error
    if password.len() > length as usize {
        anyhow::bail!("Password length is less than the number of selected character types");
    }

    for _ in password.len()..length as usize {
        let idx = rng.random_range(0..charset.len());
        password.push(charset.chars().nth(idx).unwrap());
    }
    password.shuffle(&mut rng);
    let password = String::from_iter(password);
    let estimate = zxcvbn(&password, &[]);
    if estimate.score().to_string().parse::<u8>().unwrap() < 3 {
        // Put Info to stderr, not affect the stdout for pipe
        eprintln!(
            "Warning: The generated password is weak (score: {}). Consider increasing the length or adding more character types.",
            estimate.score()
        );
    }
    eprintln!("Password strength: {:?}", estimate.score());
    Ok(password)
}
