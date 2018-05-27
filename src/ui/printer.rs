use core::Spel;
use core::Bord;
use core::bord::COLUMN_BREEDTE;
use core::Schijf;
use core::SchijfKleur;

pub fn print_spel(spel: &Spel) {
    clear_console();

    print_status(spel);
    print_bord(spel.get_bord());
}

fn clear_console() {
    print!("{}[2J", 27 as char);
}

fn print_status(spel: &Spel) {
    println!("Aan de beurt: {:?}", spel.get_beurt());
    println!();
}

fn print_bord(bord: &Bord) {
    for (index, veld) in bord.get_velden().iter().enumerate() {
        if index % 10 == 0 {
            print!("{:2}  ", COLUMN_BREEDTE - (index / 10) as u32);
        }

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

    println!("      A     B     C     D     E     F     G     H     I     J");
}