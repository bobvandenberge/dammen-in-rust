use core::Bord;
use core::SchijfKleur;
use core::Zet;

/// Verschillende uitkomsten van een zet
#[derive(Debug, PartialEq)]
pub enum ZetUitkomst {
    Winst,
    Verlies,
    BeurtWissel,
    BeurtBlijftGelijk,
}

/// Trait voor de engine. Doordat dit een trait is kan je zelf verschillende
/// implementaties bedenken met bijvoorbeeld je eigen regels.
pub trait Engine {
    fn voer_zet_uit(&self, aan_de_beurt: &SchijfKleur, bord: &mut Bord, zet: &Zet) -> Result<ZetUitkomst, String>;
}

/// Trait Object voor de standaard regels
pub struct StandaardRegels;

impl Engine for StandaardRegels {
    fn voer_zet_uit(&self, aan_de_beurt: &SchijfKleur, bord: &mut Bord, zet: &Zet) -> Result<ZetUitkomst, String> {
        let indexen = zet.converteer_naar_indexen()?;

        Ok(ZetUitkomst::BeurtWissel)
    }
}

#[cfg(test)]
mod tests {
    use core::engine::StandaardRegels;
    use core::SchijfKleur;
    use core::Bord;
    use core::Zet;
    use core::engine::Engine;
    use core::engine::ZetUitkomst;

    #[test]
    fn voer_zet_uit_verplaatst_schijf() {
        let engine: Box<Engine> = Box::new(StandaardRegels);
        let beurt = SchijfKleur::Wit;
        let mut bord = Bord::new();
        let zet = Zet {
          begin_positie: String::from("B4"),
          doel_positie: String::from("A5"),
        };

        let result = (*engine).voer_zet_uit(&beurt, &mut bord, &zet);

        assert_eq!(result, Ok(ZetUitkomst::BeurtWissel));
    }
}
