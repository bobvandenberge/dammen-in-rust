use core::Bord;
use core::engine::ZetUitkomst;
use core::Schijf;
use core::SchijfKleur;
use core::veld::VeldKleur;
use core::Veld;
use core::bord::COLUMN_BREEDTE;

type VoorVerplaatsenRegelUitkomst = Result<(), String>;
pub type VoorVerplaatsenRegel = Fn(&SchijfKleur, &mut Bord, (usize, usize)) -> VoorVerplaatsenRegelUitkomst;

type NaVerplaatsenRegelUitkomst = Option<ZetUitkomst>;
pub type NaVerplaatsenRegel = Fn(&SchijfKleur, &mut Bord, (usize, usize)) -> NaVerplaatsenRegelUitkomst;

/// Als je een schijf wilt verplaatsen van A naar B, moet op veld A wel een schijf aanwezig zijn
pub fn bron_veld_moet_schijf_bevatten(_: &SchijfKleur, bord: &mut Bord, (bron, _): (usize, usize)) -> VoorVerplaatsenRegelUitkomst {
    if bord.get_veld(bron).unwrap().get_schijf().is_none() {
        return Err(String::from("Op het begin veld staat geen schijf."));
    }

    Ok(())
}

/// Als je een schijf wilt verplaatsen van A naar B, moet op veld B nog geen schijf aanwezig zijn
pub fn doel_veld_moet_leeg_zijn(_: &SchijfKleur, bord: &mut Bord, (_, doel): (usize, usize)) -> VoorVerplaatsenRegelUitkomst {
    if bord.get_veld(doel).unwrap().get_schijf().is_some() {
        return Err(String::from("Er staat al een schijf op het doel veld."));
    }

    Ok(())
}

/// Als het de beurt van Wit is, mogen alleen de Witte schijven plaatsen worden en idem voor Zwart
pub fn eigen_beurt_eigen_kleur(aan_de_beurt: &SchijfKleur, bord: &mut Bord, (bron, _): (usize, usize)) -> VoorVerplaatsenRegelUitkomst {
    let schijf_kleur = match bord.get_veld(bron).unwrap().get_schijf().unwrap() {
        Schijf::Enkel(_kleur) => _kleur,
        Schijf::Dam(_kleur) => _kleur,
    };

    if schijf_kleur != *aan_de_beurt {
        return Err(format!("Het is niet de beurt van {:?}", schijf_kleur));
    }

    Ok(())
}

/// Je mag alleen een bruin vlak als doel hebben
pub fn alleen_bruine_vlakken_gebruiken(aan_de_beurt: &SchijfKleur, bord: &mut Bord, (_, doel): (usize, usize)) -> VoorVerplaatsenRegelUitkomst {
    if Bord::bepaald_kleur_veld(doel as u32) != VeldKleur::Bruin {
        return Err(String::from("Je mag alleen verplaatsen naar bruine vlakken"));
    }

    Ok(())
}

/// Je mag maar 1 stap per keer zetten als je niemand slaat
pub fn maar_1_stap_per_keer_voor_enkel_schijf(aan_de_beurt: &SchijfKleur, bord: &mut Bord, (bron, doel): (usize, usize)) -> VoorVerplaatsenRegelUitkomst {
    match bord.get_veld(bron).unwrap().get_schijf().unwrap() {
        Schijf::Enkel(_) => (),
        _ => return Ok(())
    }

    if kan_slaan(aan_de_beurt, bord).len() != 0 {
        return Ok(());
    }

    if verschuif_naar_links_boven(bron as u32) == doel as u32 || verschuif_naar_rechts_boven(bron as u32) == doel as u32 ||
        verschuif_naar_links_onder(bron as u32) == doel as u32 || verschuif_naar_rechts_onder(bron as u32) == doel as u32 {
        return Ok(());
    } else {
        return Err(String::from("Je mag maar 1 stap per keer zetten"))
    }
}

/// Als je kan slaan, is dat verplicht
pub fn slaan_is_verplicht(aan_de_beurt: &SchijfKleur, bord: &mut Bord, (bron, doel): (usize, usize)) -> VoorVerplaatsenRegelUitkomst {
    match bord.get_veld(bron).unwrap().get_schijf().unwrap() {
        Schijf::Enkel(_) => (),
        _ => return Ok(())
    }
    let slaan_mogelijkheden = kan_slaan(aan_de_beurt, bord);

    if slaan_mogelijkheden.len() == 0 || slaan_mogelijkheden.contains(&doel) {
        return Ok(());
    }

    Err(String::from("Je kan slaan, dit is verplicht om te doen!"))
}

/// Als een speler geen schijven meer heeft op het bord is het spel afgelopen
pub fn geen_schijven_is_einde_spel(aan_de_beurt: &SchijfKleur, bord: &mut Bord, (_, _): (usize, usize)) -> NaVerplaatsenRegelUitkomst {
    if bord.tel_schijven_voor_kleur(aan_de_beurt.tegenovergestelde()) == 0 {
        return Some(ZetUitkomst::Afgelopen);
    }

    None
}


//////
// Helper functies
/////

/// Kijk of de kleur die aan de beurt is een andere schijf kan slaan
/// Returneerd de validate nieuwe posities
fn kan_slaan(aan_de_beurt: &SchijfKleur, bord: &Bord) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    for (index, veld) in bord.get_velden().iter().enumerate() {
        // continue als de schijf niet onze kleur is
        match veld.get_schijf() {
            &Some(Schijf::Enkel(kleur)) if kleur == *aan_de_beurt => (),
            &Some(Schijf::Dam(kleur)) if kleur == *aan_de_beurt => (),
            _ => continue
        }

        for verschuif in [verschuif_naar_links_boven, verschuif_naar_rechts_boven, verschuif_naar_links_onder, verschuif_naar_rechts_onder].iter() {
            let te_slaan_veld_index = verschuif(index as u32);
            let te_slaan_veld = bord.get_veld(te_slaan_veld_index as usize);

            let schijf = match te_slaan_veld {
                Some(veld) if veld.get_schijf().is_some() => veld.get_schijf().unwrap(),
                _ => continue
            };

            // Kijk of de schijf van een andere kleur is
            match schijf {
                Schijf::Enkel(kleur) if kleur == aan_de_beurt.tegenovergestelde() => (),
                _ => continue
            };

            // Kijk of er achter de schijf nog een andere schijf ligt
            let veld_achter_te_slaan_veld_index = verschuif(te_slaan_veld_index) as usize;
            match bord.get_veld(veld_achter_te_slaan_veld_index) {
                Some(veld) if veld.get_schijf().is_none() => (),
                _ => continue
            }

            result.push(veld_achter_te_slaan_veld_index as usize);
        }
    }

    result
}

fn verschuif_naar_links_boven(index: u32) -> u32 {
    if index > (COLUMN_BREEDTE + 1) {
        return index - (COLUMN_BREEDTE + 1)
    }

    <u32>::max_value()
}

fn verschuif_naar_rechts_boven(index: u32) -> u32 {
    if index > (COLUMN_BREEDTE - 1) {
        return index - (COLUMN_BREEDTE - 1);
    }

    <u32>::max_value()
}

fn verschuif_naar_links_onder(index: u32) -> u32 {
    index + COLUMN_BREEDTE - 1
}

fn verschuif_naar_rechts_onder(index: u32) -> u32 {
    index + COLUMN_BREEDTE + 1
}

#[cfg(test)]
mod tests {
    use core::Bord;
    use core::SchijfKleur;
    use core::engine::regels::kan_slaan;

    #[test]
    fn kan_slaan_vooruit() {
        let mut bord = Bord::new();

        bord.verplaats(30, 52);

        assert_eq!(kan_slaan(&SchijfKleur::Wit, &bord), vec![43 as usize, 41 as usize]);
    }

    #[test]
    fn kan_slaan_verkeerde_beurt() {
        let mut bord = Bord::new();

        bord.verplaats(30, 52);

        let vector: Vec<usize> = Vec::new();
        assert_eq!(kan_slaan(&SchijfKleur::Zwart, &bord), vector);
    }

    #[test]
    fn kan_slaan_achteruit() {
        let mut bord = Bord::new();

        bord.verplaats(30, 52);
        bord.verplaats(63, 41);

        assert_eq!(kan_slaan(&SchijfKleur::Wit, &bord), vec![63 as usize, 43 as usize]);
    }

    #[test]
    fn kan_slaan_meerdere() {
        let mut bord = Bord::new();

        bord.verplaats(30, 52);
        bord.verplaats(32, 54);

        assert_eq!(kan_slaan(&SchijfKleur::Wit, &bord), vec![43 as usize, 41 as usize, 45 as usize, 43 as usize]);
    }

    #[test]
    fn kan_slaan_dubbele_schijf() {
        let mut bord = Bord::new();

        bord.verplaats(30, 52);
        bord.verplaats(32, 41);
        bord.verplaats(34, 43);

        let vector: Vec<usize> = Vec::new();
        assert_eq!(kan_slaan(&SchijfKleur::Wit, &bord), vector);
    }
}