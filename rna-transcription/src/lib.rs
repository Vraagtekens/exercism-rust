
const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];
const RNA: [char; 4] = ['C', 'G', 'A', 'U'];

#[derive(Debug, PartialEq, Eq)]
pub struct Dna{
    value: String
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna{
    value: String
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {

        if let Some(invalid) = dna
            .chars()
            .enumerate()
            .find(|&x| !NUCLEOTIDES.contains(&x.1)) 
        {
            return Err(invalid.0);
        }

        Ok(Self{
            value: dna.to_string()
        })
    }

    pub fn into_rna(self) -> Rna {
        let x: String = self.value.chars().map(|x| {
            match x {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => x,
            }
        }).collect();

        Rna::new(&x).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        // todo!("Construct new Rna from '{rna}' string. If string contains invalid nucleotides return index of first invalid nucleotide");
        if let Some(invalid) = rna
            .chars()
            .enumerate()
            .find(|&x| !RNA.contains(&x.1)) 
        {
            return Err(invalid.0);
        }

        Ok(Self{
            value: rna.to_string()
        })
    }
}
