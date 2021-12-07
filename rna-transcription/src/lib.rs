#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let strand = dna
            .chars()
            .take_while(|&c| c == 'C' || c == 'G' || c == 'A' || c == 'T')
            .collect::<String>();

        if strand.len() == dna.len() {
            Ok(Self(strand))
        } else {
            Err(strand.len())
        }
    }

    pub fn into_rna(self) -> Rna {
        let strand = self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _   => 'X',
            })
            .collect();

        Rna(strand)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let strand = rna
            .chars()
            .take_while(|&c| c == 'C' || c == 'G' || c == 'A' || c == 'U')
            .collect::<String>();

        if strand.len() == rna.len() {
            Ok(Self(strand))
        } else {
            Err(strand.len())
        }
    }
}
