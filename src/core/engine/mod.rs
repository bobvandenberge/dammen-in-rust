use core::Bord;
use core::engine::regels::*;
use core::SchijfKleur;
use core::Zet;

mod regels;

/// Verschillende uitkomsten van een zet
#[derive(Debug, PartialEq)]
pub enum ZetUitkomst {
    Afgelopen,
    BeurtWissel,
    BeurtBlijftGelijk,
}

/// Trait voor de engine. Doordat dit een trait is kan je zelf verschillende
/// implementaties bedenken met bijvoorbeeld je eigen regels.
pub trait Engine {
    fn voer_zet_uit(&self, aan_de_beurt: &SchijfKleur, bord: &mut Bord, zet: &Zet) -> Result<ZetUitkomst, String>;
}

/// Trait Object voor de standaard regels
pub struct StandaardRegels {
    voor_verplaatsen: Vec<Box<VoorVerplaatsenRegel>>,
    na_verplaatsen: Vec<Box<NaVerplaatsenRegel>>,
}

impl Engine for StandaardRegels {
    fn voer_zet_uit(&self, aan_de_beurt: &SchijfKleur, bord: &mut Bord, zet: &Zet) -> Result<ZetUitkomst, String> {
        let (bron, doel) = zet.converteer_naar_indexen()?;

        // Voor alle voor_verplaatsen regels uit en return meteen als er een error is
        for function in &self.voor_verplaatsen {
            (*function)(aan_de_beurt, bord, (bron, doel))?;
        }

        bord.verplaats(bron, doel);

        // Voor alle na_verplaatsen regels uit en return meteen als er een uitzonderlijke uitkomst is
        for function in &self.na_verplaatsen {
            match (*function)(aan_de_beurt, bord, (bron, doel)) {
                Some(uitkomst) => return Ok(uitkomst),
                _ => ()
            }
        }

        Ok(ZetUitkomst::BeurtWissel)
    }
}

impl StandaardRegels {
    pub fn new() -> StandaardRegels {
        StandaardRegels {
            voor_verplaatsen: vec![
                Box::new(bron_veld_moet_schijf_bevatten),
                Box::new(doel_veld_moet_leeg_zijn),
                Box::new(eigen_beurt_eigen_kleur),
                Box::new(alleen_bruine_vlakken_gebruiken),
                Box::new(maar_1_stap_per_keer_voor_enkel_schijf),
                Box::new(slaan_is_verplicht)
            ],
            na_verplaatsen: vec![
                Box::new(geen_schijven_is_einde_spel)
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use core::Bord;
    use core::engine::Engine;
    use core::engine::StandaardRegels;
    use core::engine::ZetUitkomst;
    use core::SchijfKleur;
    use core::Zet;

    #[test]
    fn voer_zet_uit_verplaatst_schijf() {
        let engine: Box<Engine> = Box::new(StandaardRegels::new());
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
        let engine: Box<Engine> = Box::new(StandaardRegels::new());
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
        let engine: Box<Engine> = Box::new(StandaardRegels::new());
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
        let engine: Box<Engine> = Box::new(StandaardRegels::new());
        let beurt = SchijfKleur::Zwart;
        let mut bord = Bord::new();
        let zet = Zet {
            begin_positie: String::from("B4"),
            doel_positie: String::from("A5"),
        };

        let result = (*engine).voer_zet_uit(&beurt, &mut bord, &zet);

        assert!(result.is_err());
    }

    #[test]
    fn voer_zet_uit_faalt_als_doel_vlak_geen_bruin_vlak_is() {
        let engine: Box<Engine> = Box::new(StandaardRegels::new());
        let beurt = SchijfKleur::Wit;
        let mut bord = Bord::new();
        let zet = Zet {
            begin_positie: String::from("B4"),
            doel_positie: String::from("B5"),
        };

        let result = (*engine).voer_zet_uit(&beurt, &mut bord, &zet);

        assert!(result.is_err());
    }

    #[test]
    fn voer_zet_uit_faalt_als_je_meer_dan_1_vak_probeert_te_verplaatsen() {
        let engine: Box<Engine> = Box::new(StandaardRegels::new());
        let beurt = SchijfKleur::Wit;
        let mut bord = Bord::new();
        let zet = Zet {
            begin_positie: String::from("B4"),
            doel_positie: String::from("B6"),
        };

        let result = (*engine).voer_zet_uit(&beurt, &mut bord, &zet);

        assert!(result.is_err());
    }

    #[test]
    fn voer_zet_uit_verplicht_slaan_faalt_als_je_het_niet_doet() {
        let engine: Box<Engine> = Box::new(StandaardRegels::new());
        let mut bord = Bord::new();
        let zet = Zet {
            begin_positie: String::from("B4"),
            doel_positie: String::from("C5"),
        };

        (*engine).voer_zet_uit(&SchijfKleur::Wit, &mut bord, &zet);

        let zet = Zet {
            begin_positie: String::from("A7"),
            doel_positie: String::from("B6"),
        };

        (*engine).voer_zet_uit(&SchijfKleur::Zwart, &mut bord, &zet);

        let zet = Zet {
            begin_positie: String::from("C5"),
            doel_positie: String::from("D6"),
        };

        let result = (*engine).voer_zet_uit(&SchijfKleur::Wit, &mut bord, &zet);

        assert!(result.is_err());
    }

    #[test]
    fn voer_zet_uit_slaan_mag() {
        let engine: Box<Engine> = Box::new(StandaardRegels::new());
        let mut bord = Bord::new();
        let zet = Zet {
            begin_positie: String::from("B4"),
            doel_positie: String::from("C5"),
        };

        (*engine).voer_zet_uit(&SchijfKleur::Wit, &mut bord, &zet);

        let zet = Zet {
            begin_positie: String::from("A7"),
            doel_positie: String::from("B6"),
        };

        (*engine).voer_zet_uit(&SchijfKleur::Zwart, &mut bord, &zet);

        let zet = Zet {
            begin_positie: String::from("C5"),
            doel_positie: String::from("A7"),
        };

        let result = (*engine).voer_zet_uit(&SchijfKleur::Wit, &mut bord, &zet);

        assert!(result.is_ok());
    }
}
