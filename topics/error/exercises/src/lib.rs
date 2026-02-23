#[derive(Debug, PartialEq)]
pub enum MathError {
    DivByZero,
}

pub fn div(x: u32, y: u32) -> Result<u32, MathError> {
    if y != 0 {
        Ok(x / y)
    } else {
        Err(MathError::DivByZero)
    }
}

pub fn get(v: &[u32], i: usize, default_val: u32) -> u32 {
    let z = v.get(i);
    match z {
        Some(val) => *val,
        None => default_val,
    }
}
