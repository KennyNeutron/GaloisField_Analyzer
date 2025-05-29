use std::fmt;

pub struct GFPoly {
    pub which_modulo: u8,
    pub coef_vals: Vec<u8>,
    pub alpha_pows: Vec<u8>,
}

impl GFPoly {
    pub fn with_coefs(coef_vals: Vec<u8>, which_modulo: u16) -> Self {
        let mut poly = GFPoly {
            which_modulo: which_modulo as u8,
            coef_vals,
            alpha_pows: vec![0; 512],
        };
        poly.generate_pows();
        poly
    }

    pub fn generate_pows(&mut self) {
        self.alpha_pows[0] = 1;
        for i in 1..512 {
            let prev = self.alpha_pows[i - 1] as u16;
            let mut next = prev << 1;
            if next >= 256 {
                next ^= self.which_modulo as u16;
            }
            self.alpha_pows[i] = (next & 0xFF) as u8;
        }
    }

    pub fn pow(&self, exp: u8) -> u8 {
        self.alpha_pows[exp as usize]
    }

    pub fn log(&self, val: u8) -> u8 {
        for (i, &v) in self.alpha_pows.iter().enumerate() {
            if v == val {
                return i as u8;
            }
        }
        panic!("Value not found in alpha_pows: {}", val);
    }
}

impl fmt::Display for GFPoly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut terms = Vec::new();

        for (i, &coef) in self.coef_vals.iter().enumerate() {
            if coef == 0 {
                continue;
            }

            let log_val = self.log(coef);
            let coef_str = match log_val {
                0 => "".to_string(),       // a^0 → omit
                1 => "a".to_string(),      // a^1 → a
                _ => format!("a^{}", log_val),
            };

            let term = match i {
                0 => {
                    if log_val == 0 {
                        "1".to_string()
                    } else {
                        coef_str.clone()
                    }
                }
                1 => {
                    if log_val == 0 {
                        "x".to_string()
                    } else {
                        format!("{} x", coef_str)
                    }
                }
                _ => {
                    if log_val == 0 {
                        format!("x^{}", i)
                    } else {
                        format!("{} x^{}", coef_str, i)
                    }
                }
            };

            terms.push(term);
        }

        write!(f, "{}", terms.join(" + "))
    }
}
