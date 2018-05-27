use core::{Schijf, SchijfKleur};
use core::Veld;

pub const COLUMN_BREEDTE: u32 = 10;

/// Representatie van het bord. Bevat alle 100 velden, dus zowel speelbaar als niet speelbaar.
pub struct Bord {
    velden: [Veld; 100]
}

impl Bord {
    /// Haal de velden op -> immutable
    pub fn get_velden(&self) -> &[Veld; 100] {
        &self.velden
    }

    /// Haal een enkel veld op -> immutable
    pub fn get_veld(&self, index: usize) -> Option<&Veld> {
        if index >= 100 {
            None
        } else {
            Some(&self.velden[index])
        }
    }

    /// Maak een nieuw bord. Op een nieuw borden worden de stenen geplaatst
    /// op de juiste posities
    pub fn new() -> Bord {
        let mut bord = Bord {
            velden: [Veld::new(); 100]
        };

        // Alle velden waarop een steen moet komen. Voor nu is hardcoden makkelijker
        // dan dynamische bepalen
        let velden_met_steen = [1, 3, 5, 7, 9, 10, 12, 14, 16, 18, 21, 23, 25, 27, 29, 30, 32, 34, 36, 38];

        // Vul met stenen
        for index in 0..40 {
            if velden_met_steen.contains(&index) {
                {
                    let mut veld = &mut bord.velden[index];
                    veld.set_schijf(Schijf::Enkel(SchijfKleur::Zwart));
                }

                {
                    let mut veld = &mut bord.velden[index + 60];
                    veld.set_schijf(Schijf::Enkel(SchijfKleur::Wit));
                }
            }
        }

        return bord;
    }

    /// Verplaats een schijf naar een ander veld
    pub fn verplaats(&mut self, bron: usize, doel: usize) -> Result<(), String> {
        if self.get_veld(bron).unwrap().get_schijf().is_none() {
            return Err(String::from("Op het begin veld staat geen schijf."));
        }

        if self.get_veld(doel).unwrap().get_schijf().is_some() {
            return Err(String::from("Er staat al een schijf op het doel veld."));
        }

        let schijf = {
            let mut veld = &mut self.velden[bron];
            veld.verwijder_schijf().unwrap()
        };

        let mut veld = &mut self.velden[doel];
        veld.set_schijf(schijf);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::Bord;

    #[test]
    fn verplaatst_schijf_gaat_fout_als_bron_geen_schijf_bevat() {
        let mut bord = Bord::new();

        let result = bord.verplaats(41, 50);

        assert!(result.is_err());
    }

    #[test]
    fn verplaatst_schijf_gaat_fout_als_doel_schijf_bevat() {
        let mut bord = Bord::new();

        bord.verplaats(30, 41);
        let result = bord.verplaats(32, 41);

        assert!(result.is_err());
    }

    #[test]
    fn verplaatst_schijf_verplaatst_schijf() {
        let mut bord = Bord::new();

        let result = bord.verplaats(30, 41);

        assert!(bord.get_veld(30).unwrap().get_schijf().is_none());
        assert!(bord.get_veld(41).unwrap().get_schijf().is_some());
    }
}