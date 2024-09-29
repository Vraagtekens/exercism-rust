const NUCLEOTIDES: [char; 4] = ['A', 'U', 'G', 'C'];

pub fn translate(rna: &str) -> Option<Vec<&str>> {

    if rna.is_empty() {
        let x = Vec::new();
        return Some(x)
    }

    if rna.chars().all(|x| !NUCLEOTIDES.contains(&x)) {
        return None
    }

     let mut result = Vec::new();

     for chunk in rna.chars().collect::<Vec<_>>().chunks(3) {
         let codon = chunk.iter().collect::<String>();
 
         match codon.as_str() {
             "AUG" => result.push("Methionine"),
             "UUU" | "UUC" => result.push("Phenylalanine"),
             "UUA" | "UUG" => result.push("Leucine"),
             "UCU" | "UCC" | "UCA" | "UCG" => result.push("Serine"),
             "UAU" | "UAC" => result.push("Tyrosine"),
             "UGU" | "UGC" => result.push("Cysteine"),
             "UGG" => result.push("Tryptophan"),
             "UAA" | "UAG" | "UGA" => return Some(result),  
             _ => return None,  
         }
     }
 
     Some(result) 
}
