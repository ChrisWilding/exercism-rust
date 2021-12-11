use std::collections::HashMap;

fn is_valid(nucleotide: char) -> bool {
    match nucleotide {
        'A' => true,
        'C' => true,
        'G' => true,
        'T' => true,
        _ => false,
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if is_valid(nucleotide) {
        dna.chars()
            .try_fold(0, |sum, c| match (is_valid(c), c == nucleotide) {
                (true, true) => Ok(sum + 1),
                (false, _) => Err(c),
                _ => Ok(sum),
            })
    } else {
        return Err(nucleotide);
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let init = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    dna.chars().try_fold(init, |mut counts, nucleotide| {
        if is_valid(nucleotide) {
            *counts.entry(nucleotide).or_insert(0) += 1;
            Ok(counts)
        } else {
            return Err(nucleotide);
        }
    })
}
