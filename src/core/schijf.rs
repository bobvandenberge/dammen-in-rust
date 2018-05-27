#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SchijfKleur {
    Wit,
    Zwart,
}

#[derive(Copy, Clone)]
pub enum Schijf {
    Enkel(SchijfKleur),
    Dam(SchijfKleur),
}
