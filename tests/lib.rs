


#[cfg(test)]
mod tests {
    use anybase::bytes_manipulation::multiply_bytes;
    use anybase::bytes_manipulation::add_bytes;


    // ---------------------------- add_bytes tests ----------------------------


    #[test]
    fn test_add_bytes() {
        assert_eq!(add_bytes(vec![1, 2, 3], vec![4, 5, 6]), vec![5, 7, 9]);
    }

    #[test]
    fn test_add_bytes_overflow() {
        assert_eq!(add_bytes(vec![255, 255, 255], vec![255, 255, 255]), vec![1, 255, 255, 254]);
    }

    #[test]
    fn test_add_bytes_symmetry() {
        assert_eq!(add_bytes(vec![1, 2, 3], vec![4, 5, 6]), add_bytes(vec![4, 5, 6], vec![1, 2, 3]))
    }

    #[test]
    fn test_add_bytes_one_small() {
        assert_eq!(add_bytes(vec![1, 2, 3], vec![4, 5]), vec![1, 6, 8]);
    }

    #[test]
    fn test_add_bytes_max_small() {
        assert_eq!(add_bytes(vec![255, 255, 255, 255], vec![1]), vec![1, 0, 0, 0, 0]);
    }


    // ----------------------------------------------- multiply_bytes tests -----------------------------------------------


    #[test]
    fn test_multiply_bytes() {
        assert_eq!(multiply_bytes(vec![1, 2, 3], vec![4, 5, 6]), vec![4, 13, 28, 27, 18]);
    }

    #[test]
    fn test_max_multiply_bytes() {
        assert_eq!(multiply_bytes(vec![255, 255, 255], vec![255, 255, 255]), vec![1, 0, 0, 0, 0, 0, 0, 0, 1]);
    }
}