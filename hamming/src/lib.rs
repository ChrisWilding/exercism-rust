/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() == s2.len() {
        let distance: usize =
            s1.chars()
                .zip(s2.chars())
                .fold(0, |sum, (a, b)| if a == b { sum } else { sum + 1 });
        Some(distance)
    } else {
        None
    }
}