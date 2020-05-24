use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct SizeIsTooSmallError;

impl Display for SizeIsTooSmallError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "size is too small")
    }
}

impl Error for SizeIsTooSmallError {
    fn cause(&self) -> Option<&(dyn std::error::Error)> {
        None
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
