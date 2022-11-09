#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    nucleotids: Vec<DNANucleotide>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucleotids: Vec<RNANucleotide>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut nucleotids = Vec::new();
        for (index, nucleotid) in dna.chars().enumerate() {
            match DNANucleotide::new(nucleotid) {
                Result::Err(_) => return Result::Err(index),
                Result::Ok(nuc) => nucleotids.push(nuc),
            }
        }   
        Result::Ok(Dna{ nucleotids })
    }

    pub fn into_rna(self) -> Rna {
        let nucleotids = self.nucleotids.iter().map(|nuc| {
            match nuc {
                DNANucleotide::A => RNANucleotide::U,
                DNANucleotide::T => RNANucleotide::A,
                DNANucleotide::C => RNANucleotide::G,
                DNANucleotide::G => RNANucleotide::C,
            }
        }).collect();
        Rna { nucleotids }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut nucleotids = Vec::new();
        for (index, nucleotid) in rna.chars().enumerate() {
            match RNANucleotide::new(nucleotid) {
                Result::Err(_) => return Result::Err(index),
                Result::Ok(nuc) => nucleotids.push(nuc),
            }
        }   
        Result::Ok(Rna{ nucleotids })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum DNANucleotide { A,T,C,G }

impl DNANucleotide {
    fn new(c: char) -> Result<Self, String> {
        match c {
            'A' => Result::Ok(DNANucleotide::A),
            'T' => Result::Ok(DNANucleotide::T),
            'C' => Result::Ok(DNANucleotide::C),
            'G' => Result::Ok(DNANucleotide::G),
            _ => Result::Err(format!("Unrecognized DNA Nucleotide: {}", c))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RNANucleotide { A,U,C,G }

impl RNANucleotide {
    fn new(c: char) -> Result<Self, String> {
        match c {
            'A' => Result::Ok(RNANucleotide::A),
            'U' => Result::Ok(RNANucleotide::U),
            'C' => Result::Ok(RNANucleotide::C),
            'G' => Result::Ok(RNANucleotide::G),
            _ => Result::Err(format!("Unrecognized RNA Nucleotide: {}", c))
        }
    }
}