/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), c| {
            c.to_digit(10)
                .map(|d| if count % 2 == 0 { d } else { d * 2 })
                .map(|d| if d > 9 { d - 9 } else { d })
                .map(|d| (sum + d, count + 1))
        })
        .map_or(false, |(sum, count)| count > 1 && sum % 10 == 0)
}
