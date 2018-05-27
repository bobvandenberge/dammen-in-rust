pub struct Bord {
    velden: [Veld; 100]
}

impl Bord {
    pub fn get_velden(&self) -> &[Veld; 100] {
        &self.velden
    }

    fn new() -> Bord {
        let mut bord = Bord {
            velden: [Veld::new(); 100]
        };

        let velden_met_steen = [1, 3, 5, 7, 9, 10, 12, 14, 16, 18, 21, 23, 25, 27, 29, 30, 32, 34, 36, 38];

        // Vul met stenen
        for index in 0..40 {
            if velden_met_steen.contains(&index) {
                {
                    let mut veld = &mut bord.velden[index];
                    veld.schijf = Some(Schijf::Enkel(SchijfKleur::Zwart));
                }

                {
                    let mut veld = &mut bord.velden[index + 60];
                    veld.schijf = Some(Schijf::Enkel(SchijfKleur::Wit));
                }
            }
        }

        return bord;
    }
}

#[derive(Copy, Clone)]
pub struct Veld {
    schijf: Option<Schijf>
}

impl Veld {
    pub fn get_schijf(&self) -> &Option<Schijf> {
        &self.schijf
    }

    fn new() -> Veld {
        Veld {
            schijf: None
        }
    }
}

#[derive(Copy, Clone)]
pub enum SchijfKleur {
    Wit,
    Zwart,
}

#[derive(Copy, Clone)]
pub enum Schijf {
    Enkel(SchijfKleur),
    Dam(SchijfKleur),
}

pub struct Spel {
    bord: Bord
}

impl Spel {
    pub fn new() -> Spel {
        Spel {
            bord: Bord::new()
        }
    }

    pub fn get_bord(&self) -> &Bord {
        &self.bord
    }
}