pub struct Allergies{
    score: u32
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(mut score: u32) -> Self {
        if score % 256 > 0 {
            score %= 256
        } 

        Allergies{
            score
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut list:Vec<Allergen> = Vec::new();
        let allergies_list: [u32; 8] = [128, 64, 32, 16, 8, 4, 2, 1];

        let mut score: u32 = self.score;

        for allergie in allergies_list {
            if score >= allergie {
                match allergie {
                    128 => list.push(Allergen::Cats),
                    64 => list.push(Allergen::Pollen),
                    32 => list.push(Allergen::Chocolate),
                    16 => list.push(Allergen::Tomatoes),
                    8 => list.push(Allergen::Strawberries),
                    4 => list.push(Allergen::Shellfish),
                    2 => list.push(Allergen::Peanuts),
                    1 => list.push(Allergen::Eggs),
                    _ => println!("Ain't special"),
                }


                score -= allergie;
            }
        }
        
        list.into_iter().rev().collect()
    }
}
