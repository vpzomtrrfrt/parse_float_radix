use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseFloatError {}

impl fmt::Display for ParseFloatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse float")
    }
}

pub fn parse_float_radix(s: &str, radix: u32) -> Result<f64, ParseFloatError> {
    let s2 = s.replace(".", "");
    let i = i64::from_str_radix(&s2, radix).map_err(|_| ParseFloatError {})?;
    let count = s.split('.').count();
    assert!(count >= 1);
    let fraction_len: usize;
    match count {
        0 => unreachable!(),
        1 => fraction_len = 0,
        2 => fraction_len = s.split('.').last().unwrap().len(),
        _ => return Err(ParseFloatError {}),
    }
    let f = (i as f64) / (radix as f64).powi(fraction_len as i32);
    Ok(f)
}

#[cfg(test)]
mod test;
