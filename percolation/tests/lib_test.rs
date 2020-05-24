#[cfg(test)]
mod percolation_tests {
    use percolation::*;

    #[test]
    fn constructor_test() {
        let mut _p = Percolation::new(0);

        match _p {
            Ok(_) => assert!(false, "this should throw SizeIsTooSmallError"),
            Err(error) => assert_eq!(
                error == percolation::errors::size_is_too_small_error::SizeIsTooSmallError,
                true
            ),
        }
    }
}
