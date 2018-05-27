use core::Schijf;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum VeldKleur {
    Wit,
    Bruin
}

#[derive(Copy, Clone)]
pub struct Veld {
    schijf: Option<Schijf>
}

impl Veld {
    /// Haal de schijf op die op dit veld staan -> immutable
    pub fn get_schijf(&self) -> &Option<Schijf> {
        &self.schijf
    }

    /// Zet een schijf op dit veld
    pub fn set_schijf(&mut self, schijf: Schijf) {
        self.schijf = Some(schijf);
    }

    pub fn verwijder_schijf(&mut self) -> Option<Schijf> {
        let schijf = self.schijf;

        self.schijf = None;

        schijf
    }

    pub fn new() -> Veld {
        Veld {
            schijf: None
        }
    }
}
