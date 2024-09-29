use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {

    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide)
    }

    if let Some(invalid) = dna.chars().find(|&x| !NUCLEOTIDES.contains(&x)) {
        return Err(invalid);
    }

    Ok(dna.chars().filter(|&x| x == nucleotide).count())

}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {

    if let Some(invalid) = dna.chars().find(|&x| !NUCLEOTIDES.contains(&x)) {
        return Err(invalid);  
    }

    let mut expected = HashMap::new();
    expected.insert('A', count('A', dna).unwrap_or(0));
    expected.insert('C', count('C', dna).unwrap_or(0));
    expected.insert('G', count('G', dna).unwrap_or(0));
    expected.insert('T', count('T', dna).unwrap_or(0));

    Ok(expected)
}
