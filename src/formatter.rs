pub struct Formatter {
    unit: char,
    digits: u8,
}

const CHARACTERS: [char; 4] = ['B', 'K', 'M', 'G'];

impl Formatter {
    pub fn from(input: &str) -> Option<Formatter> {
        let mut it = input.chars();
        if it.next().is_none_or(|c| c != '.') {
            return None;
        }
        let digit = it.next();
        if digit.is_none_or(|c| !c.is_ascii_digit()) {
            return None;
        }
        let unit = it.next();
        if unit.is_none_or(|c| !CHARACTERS.contains(&c)) {
            return None;
        }
        Some(Formatter {
            unit: unit.unwrap(),
            digits: digit.unwrap().to_digit(10).unwrap() as u8,
        })
    }

    pub fn get_byte_display(&self, bytes: u64) -> String {
        let multiplier: f64 = match self.unit {
            'K' => 1f64 / 1_000f64,
            'M' => 1f64 / 1_000_000f64,
            'G' => 1f64 / 1_000_000_000f64,
            _ => 1f64,
        };
        format!(
            "{:.3$} {}{}",
            bytes as f64 * multiplier,
            self.unit,
            if self.unit == 'B' {
                ""
            } else {
                "B"
            },
            self.digits as usize
        )
    }


    pub fn digits(&self) -> u8{
        self.digits 
    }
    pub fn unit(&self) -> char{
        self.unit 
    }
}

impl Default for  Formatter{

    fn default() -> Self {
        Self::from(".2G").unwrap()
    }
    
}

#[cfg(test)]
pub mod tests {
    use crate::formatter::Formatter;


    #[test]
    fn constructing() {
        let formatter = Formatter::from(".3G").unwrap();
        assert_eq!(formatter.unit(), 'G');
        assert_eq!(formatter.digits(), 3);
        let formatter = Formatter::from(".0B").unwrap();
        assert_eq!(formatter.unit(), 'B');
        assert_eq!(formatter.digits(), 0);
    }
    #[test]
    fn formatting() {
        let formatter = Formatter::from(".2G").unwrap();
        assert_eq!(formatter.get_byte_display(1_100_000_000), "1.10 GB")
    }
}
