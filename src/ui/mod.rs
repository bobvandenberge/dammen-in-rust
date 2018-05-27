use super::core::{Spel, Schijf, SchijfKleur};

pub fn print_spel(spel: &Spel) {
    for (index, veld) in spel.get_bord().get_velden().iter().enumerate() {
        match veld.get_schijf() {
            &Some(Schijf::Enkel(SchijfKleur::Wit)) => print!("[ WE ]"),
            &Some(Schijf::Enkel(SchijfKleur::Zwart)) => print!("[ ZE ]"),
            &Some(Schijf::Dam(SchijfKleur::Wit)) => print!("[ WD ]"),
            &Some(Schijf::Dam(SchijfKleur::Zwart)) => print!("[ ZD ]"),
            &None => print!("[    ]")
        }

        if index % 10 == 9 {
            println!();
        }
    }
}