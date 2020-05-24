pub mod errors;

use errors::size_is_too_big_error::SizeIsTooBigError;
use errors::size_is_too_small_error::SizeIsTooSmallError;

#[derive(Debug)]
pub struct Percolation {
    sites: Vec<Option<usize>>,
}

const MINIMUM_SIZE_THAT_MAKES_SENSE: usize = 2;

impl Percolation {
    pub fn new(size: usize) -> Result<Percolation, Box<dyn std::error::Error>> {
        let number_of_items: usize = size.pow(2);

        if size < MINIMUM_SIZE_THAT_MAKES_SENSE {
            return Err(Box::new(SizeIsTooSmallError));
        }

        if number_of_items > usize::MAX {
            return Err(Box::new(SizeIsTooBigError));
        }

        let mut result = Percolation {
            sites: Vec::with_capacity(number_of_items),
        };

        for _ in 0..size {
            result.sites.push(Option::None::<usize>);
        }

        Result::Ok(result)
    }
}
