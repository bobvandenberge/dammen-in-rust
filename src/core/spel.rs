use core::Bord;
use core::SchijfKleur;
use core::Zet;
use core::engine::Engine;
use core::engine::StandaardRegels;
use core::engine::ZetUitkomst;

/// Representatie van een spelletje dammen
pub struct Spel {
    engine: Box<Engine>,
    bord: Bord,
    beurt: SchijfKleur,
    is_afgelopen: bool
}

impl Spel {
    fn new(engine: Box<Engine>) -> Spel {
        Spel {
            engine,
            bord: Bord::new(),
            beurt: SchijfKleur::Wit,
            is_afgelopen: false
        }
    }

    /// Maak een nieuw spel aan met de standaard regels
    pub fn met_standaard_regels() -> Spel {
        Spel::new(Box::new(StandaardRegels::new()))
    }

    /// Haal het speeldveld, a.k.a. bord, op -> immutable
    pub fn get_bord(&self) -> &Bord {
        &self.bord
    }

    /// Haal op wie er aan de beurt is -> immutable
    pub fn get_beurt(&self) -> &SchijfKleur {
        &self.beurt
    }

    /// Haal op of het spel afgelopen is
    pub fn is_afgelopen(&self) -> &bool {
        &self.is_afgelopen
    }

    /// Voer een zet uit. Dit resultaat of in een lege Ok of, als
    /// er iets fout is, in een String die aan de gebruiker getoond
    /// kan worden voor uitleg.
    pub fn zet(&mut self, zet: &Zet) -> Result<(), String> {
        let resultaat = self.engine.voer_zet_uit(&self.beurt, &mut self.bord, zet)?;

        match resultaat {
            ZetUitkomst::BeurtWissel => self.wissel_beurt(),
            ZetUitkomst::Afgelopen => self.is_afgelopen = true,
            _ => ()
        }

        Ok(())
    }

    fn wissel_beurt(&mut self) {
        self.beurt = match self.beurt {
            SchijfKleur::Wit => SchijfKleur::Zwart,
            SchijfKleur::Zwart => SchijfKleur::Wit
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Spel;
    use core::SchijfKleur;
    use core::Zet;
    use core::Bord;
    use core::engine::Engine;
    use core::engine::ZetUitkomst;

    pub struct GeenRegels;

    impl Engine for GeenRegels {
        fn voer_zet_uit(&self, _: &SchijfKleur, _: &mut Bord, _: &Zet) -> Result<ZetUitkomst, String> {
            Ok(ZetUitkomst::BeurtWissel)
        }
    }

    pub struct AltijdFout;

    impl Engine for AltijdFout {
        fn voer_zet_uit(&self, _: &SchijfKleur, _: &mut Bord, _: &Zet) -> Result<ZetUitkomst, String> {
            Err(String::from("Een reden"))
        }
    }

    #[test]
    fn creer_new_spel() {
        Spel::new(Box::new(GeenRegels));
    }

    #[test]
    fn zet_moet_beurt_veranderen_bij_beurt_wissel_uitkomst() {
        let mut spel = Spel::new(Box::new(GeenRegels));

        assert_eq!(*spel.get_beurt(), SchijfKleur::Wit);

        let zet = Zet {
            begin_positie: String::from("B4"),
            doel_positie: String::from("C5")
        };

        let result = spel.zet(&zet);

        assert_eq!(*spel.get_beurt(), SchijfKleur::Zwart);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn zet_moet_niets_doen_bij_invalide_zet() {
        let mut spel = Spel::new(Box::new(AltijdFout));

        let zet = Zet {
            begin_positie: String::from("A1"),
            doel_positie: String::from("B2")
        };

        let result = spel.zet(&zet);

        assert_eq!(*spel.get_beurt(), SchijfKleur::Wit);
        assert_eq!(result, Err(String::from("Een reden")));
    }
}