/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if !code.chars().all(|c| c.is_numeric() || c.is_whitespace()) {
        return false;
    }

    let cleaned: Vec<_> = code.chars().filter(|c| !c.is_whitespace()).rev().collect();

    if cleaned.len() == 1 && cleaned[0] == '0' {
        return false;
    }

    let sum: u32 = cleaned
        .iter()
        .map(|c| c.to_digit(10).unwrap())
        .zip(0..16)
        .map(|(d, i)| if i % 2 == 0 { d } else { d * 2 })
        .map(|d| if d > 9 { d - 9 } else { d })
        .sum();

    sum % 10 == 0
}
