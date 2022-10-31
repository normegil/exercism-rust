pub struct Allergies {
    score: u32
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl Allergen {
    fn as_array() -> Vec<Allergen> {
        vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats
        ]
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        match allergen {
            Allergen::Eggs => self.check_flag(0), 
            Allergen::Peanuts => self.check_flag(1), 
            Allergen::Shellfish => self.check_flag(2), 
            Allergen::Strawberries => self.check_flag(3),
            Allergen::Tomatoes => self.check_flag(4), 
            Allergen::Chocolate => self.check_flag(5), 
            Allergen::Pollen => self.check_flag(6),
            Allergen::Cats => self.check_flag(7),
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies_list: Vec<Allergen> = Vec::new();
        for allergen in Allergen::as_array().iter() {
            if self.is_allergic_to(allergen) {
                allergies_list.push(*allergen)
            }
        }
        allergies_list
    }

    fn check_flag(&self, flag_nb: u8) -> bool {
        ((1 << flag_nb) & self.score) != 0
    }
}
