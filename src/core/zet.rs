extern crate regex;

use self::regex::Regex;
use core::bord::COLUMN_BREEDTE;

/// Weergave van een zet die een speler kan maken. Als er een
/// reeks van stappen gezet moeten worden (bijvoorbeeld als een
/// speler verplicht meerdere keren moet slaan). Dan moet er per
/// stap een nieuwe Zet aangemaakt worden.
pub struct Zet {
    /// Begin positie van de steen in format: A1.
    pub begin_positie: String,
    /// Begin positie van de steen in format: B2.
    pub doel_positie: String
}

impl Zet {

    /// Converteer de zet naar numerice indexes. Retourneerd een Tuple
    /// waarin index 0 de begin_positie is en 1 de doel_positie
    pub fn converteer_naar_indexen(&self) -> Result<(u32, u32), String> {
        Ok((
            self.converteer_naar_index(&self.begin_positie)?,
            self.converteer_naar_index(&self.doel_positie)?
        ))
    }

    fn converteer_naar_index(&self, address: &String) -> Result<u32, String> {
        let regex = Regex::new(r"^([A-Z])([1-9])$").unwrap();

        if !regex.is_match(address) {
            return Err(format!("Positie {} is geen geldige positie!", address))
        }

        let mut chars = address.chars();
        let column = chars.next().unwrap();
        let row = chars.next().unwrap();

        // A = 10, B = 11 etc dus voegen we hier 9 toe zodat we de eerste 9 getallen ('0', '1)
        // enzo overslaan
        let alphanumeric_range = COLUMN_BREEDTE + 9;
        if !column.is_digit(alphanumeric_range) {
            return Err(format!("Column {} is geen geldige column!", column))
        }

        // We willen een index die start op 1 dus we halen van het resultaat (A = 10) 9 af.
        let column = column.to_digit(alphanumeric_range).unwrap() - 9;
        let row = row.to_digit(COLUMN_BREEDTE).unwrap();

        let index = (COLUMN_BREEDTE - row) * COLUMN_BREEDTE + (column - 1);

        Ok(index)
    }
}

#[cfg(test)]
mod tests {
    use core::Zet;

    #[test]
    fn converteer_geeft_fout_terug_bij_invalide_format_positie() {
        let zet = Zet {
            begin_positie: String::from("4Z"),
            doel_positie: String::from("Y4"),
        };

        let result = zet.converteer_naar_indexen();

        assert!(result.is_err());
    }

    #[test]
    fn converteer_geeft_fout_terug_bij_invalide_positie() {
        let zet = Zet {
            begin_positie: String::from("Z4"),
            doel_positie: String::from("Y4"),
        };

        let result = zet.converteer_naar_indexen();

        assert!(result.is_err());
    }

    #[test]
    fn converteer_geeft_juiste_index_terug() {
        let zet = Zet {
            begin_positie: String::from("B4"),
            doel_positie: String::from("I1"),
        };

        let result = zet.converteer_naar_indexen();

        assert_eq!(result, Ok((61, 98)));
    }
}
