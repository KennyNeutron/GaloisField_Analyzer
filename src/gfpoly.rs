// gfpoly.rs - Corrected version
use std::fmt;
use std::ops::Mul;
use std::str::FromStr;

pub struct GFPoly {
    coefs: Vec<u8>,
    which_modulo: u16,
}

impl GFPoly {
    pub fn with_coefs(coefs: Vec<u8>, which_modulo: u16) -> Self {
        GFPoly { coefs, which_modulo }
    }
}

impl FromStr for GFPoly {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let len = parts.next().ok_or(())?.parse::<usize>().map_err(|_| ())?;
        let modulo = parts.next().ok_or(())?.parse::<u16>().map_err(|_| ())?;
        let coefs: Vec<u8> = parts.take(len).map(|x| x.parse::<u8>().unwrap_or(0)).collect();
        Ok(GFPoly::with_coefs(coefs, modulo))
    }
}

impl fmt::Display for GFPoly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut terms = Vec::new();
        for (i, &c) in self.coefs.iter().enumerate() {
            if c == 0 {
                continue;
            }
            let term = match (c, i) {
                (1, 0) => "1".to_string(),
                (1, 1) => "x".to_string(),
                (1, _) => format!("x^{}", i),
                (_, 0) => {
                    if c == 1 { 
                        "1".to_string() 
                    } else if c == 2 {
                        "a".to_string()
                    } else {
                        format!("a^{}", c - 1)
                    }
                },
                (_, 1) => {
                    if c == 1 { 
                        "x".to_string() 
                    } else if c == 2 {
                        "a x".to_string()
                    } else {
                        format!("a^{} x", c - 1)
                    }
                },
                (_, _) => {
                    if c == 1 { 
                        format!("x^{}", i) 
                    } else if c == 2 {
                        format!("a x^{}", i)
                    } else {
                        format!("a^{} x^{}", c - 1, i)
                    }
                },
            };
            terms.push(term);
        }
        if terms.is_empty() {
            write!(f, "0")
        } else {
            write!(f, "{}", terms.join(" + "))
        }
    }
}

impl Mul for &GFPoly {
    type Output = GFPoly;

    fn mul(self, rhs: Self) -> GFPoly {
        let mut result = vec![0u8; self.coefs.len() + rhs.coefs.len() - 1];
        for (i, &a) in self.coefs.iter().enumerate() {
            for (j, &b) in rhs.coefs.iter().enumerate() {
                let product = gf_mul(a, b, self.which_modulo);
                result[i + j] = gf_add(result[i + j], product);
            }
        }
        GFPoly::with_coefs(result, self.which_modulo)
    }
}

// Precomputed log and antilog tables for GF(2^8) with primitive polynomial 285
const LOG_TABLE: [u8; 256] = [
    0, 0, 1, 25, 2, 50, 26, 198, 3, 223, 51, 238, 27, 104, 199, 75,
    4, 100, 224, 14, 52, 141, 239, 129, 28, 193, 105, 248, 200, 8, 76, 113,
    5, 138, 101, 47, 225, 36, 15, 33, 53, 147, 142, 218, 240, 18, 130, 69,
    29, 181, 194, 125, 106, 39, 249, 185, 201, 154, 9, 120, 77, 228, 114, 166,
    6, 191, 139, 98, 102, 221, 48, 253, 226, 152, 37, 179, 16, 145, 34, 136,
    54, 208, 148, 206, 143, 150, 219, 189, 241, 210, 19, 92, 131, 56, 70, 64,
    30, 66, 182, 163, 195, 72, 126, 110, 107, 58, 40, 84, 250, 133, 186, 61,
    202, 94, 155, 159, 10, 21, 121, 43, 78, 212, 229, 172, 115, 243, 167, 87,
    7, 112, 192, 247, 140, 128, 99, 13, 103, 74, 222, 237, 49, 197, 254, 24,
    227, 165, 153, 119, 38, 184, 180, 124, 17, 68, 146, 217, 35, 32, 137, 46,
    55, 63, 209, 91, 149, 188, 207, 205, 144, 135, 151, 178, 220, 252, 190, 97,
    242, 86, 211, 171, 20, 42, 93, 158, 132, 60, 57, 83, 71, 109, 65, 162,
    31, 45, 67, 216, 183, 123, 164, 118, 196, 23, 73, 236, 127, 12, 111, 246,
    108, 161, 59, 82, 41, 157, 85, 170, 251, 96, 134, 177, 187, 204, 62, 90,
    203, 89, 95, 176, 156, 169, 160, 81, 11, 245, 22, 235, 122, 117, 44, 215,
    79, 174, 213, 233, 230, 231, 173, 232, 116, 214, 244, 234, 168, 80, 88, 175
];

const ANTILOG_TABLE: [u8; 256] = [
    1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38,
    76, 152, 45, 90, 180, 117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192,
    157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181, 119, 238, 193, 159, 35,
    70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222, 161,
    95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240,
    253, 231, 211, 187, 107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226,
    217, 175, 67, 134, 17, 34, 68, 136, 13, 26, 52, 104, 208, 189, 103, 206,
    129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197, 151, 51, 102, 204,
    133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84,
    168, 77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115,
    230, 209, 191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255,
    227, 219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65,
    130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166,
    81, 162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9,
    18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11, 22,
    44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216, 173, 71, 142, 1
];

fn gf_add(a: u8, b: u8) -> u8 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    if a == b { return 0; } // Same discrete log values cancel out
    
    // Convert discrete log representation to actual field elements
    let field_a = coeff_to_field_element(a);
    let field_b = coeff_to_field_element(b);
    let field_result = field_a ^ field_b; // XOR in GF(2^8)
    
    field_element_to_coeff(field_result)
}

fn coeff_to_field_element(coeff: u8) -> u8 {
    if coeff == 0 { return 0; }  // 0 coefficient = 0 field element
    if coeff == 1 { return 1; }  // coeff 1 = field element 1
    // coeff c represents α^(c-1)
    // So we need ANTILOG_TABLE[c-1] but ANTILOG_TABLE[0] = α^0 = 1
    // Actually ANTILOG_TABLE[i] = α^i, so for α^(c-1) we need ANTILOG_TABLE[c-1]
    ANTILOG_TABLE[((coeff - 1) as usize) % 255]
}

fn field_element_to_coeff(elem: u8) -> u8 {
    if elem == 0 { return 0; }
    if elem == 1 { return 1; }
    // LOG_TABLE[elem] gives us the exponent i where elem = α^i
    // We want coeff = i + 1 (since coeff c represents α^(c-1))
    let log_val = LOG_TABLE[elem as usize];
    log_val + 1
}

fn gf_mul(a: u8, b: u8, _modulo: u16) -> u8 {
    if a == 0 || b == 0 {
        return 0;
    }
    if a == 1 {
        return b;
    }
    if b == 1 {
        return a;
    }
    
    // Coefficients represent discrete logs: c means a^(c-1)
    // So we add the exponents and use modular arithmetic
    let exp_a = (a - 1) as usize;
    let exp_b = (b - 1) as usize;
    let sum_exp = (exp_a + exp_b) % 255;
    
    (sum_exp + 1) as u8
}