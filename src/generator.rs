use rand::{random_range};

pub fn password_generator(length: usize) -> String {
    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:,.<>?/~*";
    let _rng = rand::rngs::ThreadRng::default();

    (0..length)
        .map(|_| {
            let idx = random_range(0..characters.len());
            characters.chars().nth(idx).unwrap()
        })
        .collect()
}
pub fn security_level(password: &str) -> &'static str {
    let length = password.len();
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    let mut score = 0;

    if length >= 8 { score += 1; }
    if length >= 12 { score += 1; }
    if has_lower && has_upper { score += 1; }
    if has_digit { score += 1; }
    if has_symbol { score += 1; }

    match score {
        0..=2 => "Weak",
        3..=4 => "Medium",
        _ => "Strong",
    }
}