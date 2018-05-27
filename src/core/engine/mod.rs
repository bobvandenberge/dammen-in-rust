use core::Bord;
use core::SchijfKleur;
use core::Zet;
use core::Schijf;

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
        let (bron, doel) = zet.converteer_naar_indexen()?;

        // Regel
        if bord.get_veld(bron).unwrap().get_schijf().is_none() {
            return Err(String::from("Op het begin veld staat geen schijf."));
        }

        // Regel
        if bord.get_veld(doel).unwrap().get_schijf().is_some() {
            return Err(String::from("Er staat al een schijf op het doel veld."));
        }

        // Regel
        let schijf_kleur = match bord.get_veld(bron).unwrap().get_schijf().unwrap() {
            Schijf::Enkel(_kleur) => _kleur,
            Schijf::Dam(_kleur) => _kleur,
        };

        if schijf_kleur != *aan_de_beurt {
            return Err(format!("Het is niet de beurt van {:?}", schijf_kleur));
        }



        bord.verplaats(bron, doel);

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

    #[test]
    fn voer_zet_uit_gaat_fout_als_bron_geen_schijf_bevat() {
        let engine: Box<Engine> = Box::new(StandaardRegels);
        let beurt = SchijfKleur::Wit;
        let mut bord = Bord::new();
        let zet = Zet {
            begin_positie: String::from("A5"),
            doel_positie: String::from("B6"),
        };

        let result = (*engine).voer_zet_uit(&beurt, &mut bord, &zet);

        assert!(result.is_err());
    }

    #[test]
    fn voer_zet_uit_gaat_fout_als_doel_schijf_bevat() {
        let engine: Box<Engine> = Box::new(StandaardRegels);
        let beurt = SchijfKleur::Wit;
        let mut bord = Bord::new();
        let zet = Zet {
            begin_positie: String::from("A3"),
            doel_positie: String::from("B4"),
        };

        let result = (*engine).voer_zet_uit(&beurt, &mut bord, &zet);

        assert!(result.is_err());
    }

    #[test]
    fn voer_zet_uit_faalt_als_niet_jou_beurt_is() {
        let engine: Box<Engine> = Box::new(StandaardRegels);
        let beurt = SchijfKleur::Zwart;
        let mut bord = Bord::new();
        let zet = Zet {
            begin_positie: String::from("B4"),
            doel_positie: String::from("A5"),
        };

        let result = (*engine).voer_zet_uit(&beurt, &mut bord, &zet);

        assert!(result.is_err());
    }
}
