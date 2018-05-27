use core::{Schijf, SchijfKleur};
use core::Veld;
use core::veld::VeldKleur;

pub const COLUMN_BREEDTE: u32 = 10;

/// Representatie van het bord. Bevat alle 100 velden, dus zowel speelbaar als niet speelbaar.
pub struct Bord {
    velden: [Veld; 100]
}

impl Bord {
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

    /// Haal de velden op -> immutable
    pub fn get_velden(&self) -> &[Veld; 100] {
        &self.velden
    }

    /// Haal een enkel veld op -> immutable
    pub fn get_veld(&self, index: usize) -> Option<&Veld> {
        if index >= 100 || index < 0 {
            None
        } else {
            Some(&self.velden[index])
        }
    }

    /// Tell alle schijven van een bepaalde kleur
    pub fn tel_schijven_voor_kleur(&self, kleur: SchijfKleur) -> u32 {
        let mut teller = 0;

        for veld in self.velden.iter() {
            if veld.get_schijf().is_some() {
                match veld.get_schijf().unwrap() {
                    Schijf::Enkel(_kleur) if _kleur == kleur => teller += 1,
                    Schijf::Dam(_kleur) if _kleur == kleur => teller += 1,
                    _ => ()
                };
            }
        }

        teller
    }

    /// Verplaats een schijf naar een ander veld
    /// Deze methode gaat ervan uit dat de stap geldig is. Controleer dit dus
    /// van te voren!
    pub fn verplaats(&mut self, bron: usize, doel: usize) {
        let schijf = {
            let mut veld = &mut self.velden[bron];
            veld.verwijder_schijf().unwrap()
        };

        let veld = &mut self.velden[doel];
        veld.set_schijf(schijf);
    }

    /// Bepaal of een gegeven index een index is van een bruin of wit veld
    pub fn bepaald_kleur_veld(index: u32) -> VeldKleur {
        let rij_index = (index - (index % COLUMN_BREEDTE)) / COLUMN_BREEDTE;
        let is_even = rij_index % 2;

        let column_index = index - (rij_index * COLUMN_BREEDTE);

        let normalized_column_index = column_index + is_even;

        if normalized_column_index % 2 == 0 {
            VeldKleur::Wit
        } else {
            VeldKleur::Bruin
        }
    }
}

#[cfg(test)]
mod tests {
    use core::Bord;
    use core::SchijfKleur;
    use core::veld::VeldKleur;

    #[test]
    fn verplaatst_schijf_verplaatst_schijf() {
        let mut bord = Bord::new();

        bord.verplaats(30, 41);

        assert!(bord.get_veld(30).unwrap().get_schijf().is_none());
        assert!(bord.get_veld(41).unwrap().get_schijf().is_some());
    }

    #[test]
    fn tel_stenen_voor_kleur() {
        let bord = Bord::new();

        let result = bord.tel_schijven_voor_kleur(SchijfKleur::Wit);

        assert_eq!(result, 20);
    }

    #[test]
    fn bepaald_kleur_veld() {
        let bord = Bord::new();

        assert_eq!(Bord::bepaald_kleur_veld(0), VeldKleur::Wit, "Fout bij index 0");
        assert_eq!(Bord::bepaald_kleur_veld(23), VeldKleur::Bruin, "Fout bij index 23");
        assert_eq!(Bord::bepaald_kleur_veld(98), VeldKleur::Bruin, "Fout bij index 98");
    }
}