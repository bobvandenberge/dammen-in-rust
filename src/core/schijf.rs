#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SchijfKleur {
    Wit,
    Zwart,
}

impl SchijfKleur {
    pub fn tegenovergestelde(&self) -> SchijfKleur {
        match self {
            &SchijfKleur::Wit => SchijfKleur::Zwart,
            &SchijfKleur::Zwart => SchijfKleur::Wit,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Schijf {
    Enkel(SchijfKleur),
    Dam(SchijfKleur),
}
