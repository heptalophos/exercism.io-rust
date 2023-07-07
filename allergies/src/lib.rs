pub struct Allergies {
    score: u32
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
    pub fn new(score: u32) -> Allergies { 
        Allergies { score: score } 
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergic_to = 1 << *allergen as u32;
        self.score & allergic_to >= 1
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergies::allergens()
        .iter()
        .filter(|allergen| self.is_allergic_to(allergen))
        .map(|x| *x).collect()
    }

    fn allergens() -> Vec<Allergen> {
        vec![Allergen::Eggs, 
             Allergen::Peanuts, 
             Allergen::Shellfish, 
             Allergen::Strawberries, 
             Allergen::Tomatoes, 
             Allergen::Chocolate, 
             Allergen::Pollen, 
             Allergen::Cats]
    }
}