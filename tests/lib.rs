


#[cfg(test)]
mod tests {
    use anybase::bytes_manipulation::multiply_bytes;
    
    #[test]
    fn test_multiply_bytes() {
        assert_eq!(multiply_bytes(vec![1, 2, 3], vec![4, 5, 6]), vec![4, 13, 28, 27, 18]);
    }

    #[test]
    fn test_max_multiply_bytes() {
        assert_eq!(multiply_bytes(vec![255, 255, 255], vec![255, 255, 255]), vec![1, 0, 0, 0, 0, 0, 0, 0, 1]);
    }
}