use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SizeIsTooBigError;

impl fmt::Display for SizeIsTooBigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "size is too big")
    }
}

impl Error for SizeIsTooBigError {
    fn cause(&self) -> Option<&(dyn std::error::Error)> {
        None
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
