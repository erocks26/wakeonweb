use std::iter::zip;
use rocket::form;

#[derive(Debug)]
pub struct MacAddress {
    pub octets: [u8; 6],
}

/*
impl FromStr for MacAddress {
    type Err = String;
    fn from_str(s: &str) -> Result<MacAddress, String> {
        let octets = s.replace(":", "");
        return match decode(&octets) {
            Ok(v) => {
                let mut octets: [u8; 6] = [0u8; 6];
                for i in zip(v.into_iter(), 0..6) {
                    octets[i.1] = i.0;
                }
                Ok(MacAddress { octets })
            },
            Err(e) => Err(e.to_string()),
        };
    }
}
*/

impl form::FromFormField<'_> for MacAddress {
    fn from_value(field: form::ValueField) -> form::Result<Self> {
        match validate_text(field.value) {
            Ok(_) => (),
            Err(e) => return Err(e.into()),
        }
        
        let octets = field.value.replace(":", "").replace("-", "");
        match hex::decode(&octets) {
            Ok(v) => {
                let mut octets: [u8; 6] = [0u8; 6];
                for i in zip(v.into_iter(), 0..6) {
                    octets[i.1] = i.0;
                }
                Ok(MacAddress { octets })
            },
            Err(e) => Err(form::Error::validation(format!("Failed to parse MAC Address: {})", e)).into())
        }
    }
}

pub fn validate_text(text: &str) -> Result<(), form::Error<'_>> {
    if text.len() != 17 {
        return Err(form::Error::validation("Wrong MAC Address length").into());
    }
    
    for i in zip(text.chars(), 0..17) {
        match i.1 {
            0 | 1 | 3 | 4 | 6 | 7 | 9 | 10 | 12 | 13 | 15 | 16 => {
                if i.0.is_ascii_hexdigit() {
                    ()
                } else {
                    return Err(form::Error::validation(format!("Non-Hexadecimal characters are not allowed. Check character at position {}.", i.1)));
                }
            }
            2 | 5 | 8 | 11 | 14 => {
                if i.0.eq(&':') | i.0.eq(&'-') {
                    ()
                } else {
                    return Err(form::Error::validation(format!("Character that should be ':' or '-' is not. Check character at position {}", i.1)));
                }
            }
            _ => {
                return Err(form::Error::validation("Incorrect string length"));
            }
        }
    }
    Ok(())
}