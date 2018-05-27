mod printer;

use std::io;
use core::Spel;
use core::Zet;
use ui::printer::print_spel;

pub fn start_spel() {
    let mut spel = Spel::met_standaard_regels();

    while !spel.is_afgelopen() {
        print_spel(&spel);

        verkrijg_zet_en_voer_uit(&mut spel);
    }

    println!("Spel is afgelopen! Gefelicteerd speler {:?}", spel.get_beurt());
}

fn verkrijg_zet_en_voer_uit(spel: &mut Spel) {
    let zet = verkrijg_zet();

    match spel.zet(&zet) {
        Err(_error) => {
            println!("{}", _error);
            verkrijg_zet_en_voer_uit(spel);
        },
        _ => ()
    }
}

fn verkrijg_zet() -> Zet {
    Zet {
        begin_positie: verkrijg_begin_positie(),
        doel_positie: verkrijg_doel_positie()
    }
}

fn verkrijg_begin_positie() -> String {
    println!("Welke steen moet verplaatst worden?: ");
    let mut begin_positie = String::new();

    match io::stdin().read_line(&mut begin_positie) {
        Ok(_) => begin_positie.trim().to_owned(),
        Err(_) => verkrijg_begin_positie()
    }
}

fn verkrijg_doel_positie() -> String {
    println!("Waar moet deze steen heen?: ");
    let mut doel_positie = String::new();

    match io::stdin().read_line(&mut doel_positie) {
        Ok(_) => doel_positie.trim().to_owned(),
        Err(_) => verkrijg_doel_positie()
    }
}
