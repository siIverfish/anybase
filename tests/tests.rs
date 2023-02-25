


#[cfg(test)]
mod test_add {
    use anybase::bytes_manipulation::*;

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


    #[test]
    fn test_add_with_empty() {
        assert_eq!(add_bytes(vec![1, 2, 3], vec![]), vec![1, 2, 3]);
    }
}

#[cfg(test)]
mod test_multiply {
    use anybase::bytes_manipulation::*;

    // ----------------------------------------------- multiply_bytes tests -----------------------------------------------

    #[test]
    fn test_multiply_bytes() {
        assert_eq!(multiply_bytes(&vec![1, 2, 3], &vec![4, 5, 6]), vec![4, 13, 28, 27, 18]);
    }

    #[test]
    fn test_max_multiply_bytes() {
        let num1: u64 = (2 as u64).pow(24) - 1; 
        let num2: u64 = (2 as u64).pow(24) - 1;

        let vec1   = (num1).to_be_bytes().to_vec();
        let vec2   = (num2).to_be_bytes().to_vec();

        let result: Vec<u8> = (num1 * num2)
            .to_be_bytes()
            .to_vec()
            .into_iter()
            .skip_while(|&x| x == 0)
            .collect();

        assert_eq!(multiply_bytes(&vec1, &vec2), result);
    }

    #[test]
    fn test_random_multiply_bytes_1() {
        let num1: u64 = 340598;
        let num2: u64 = 2987;

        let vec1   = (num1).to_be_bytes().to_vec();
        let vec2   = (num2).to_be_bytes().to_vec();

        let result: Vec<u8> = (num1 * num2)
            .to_be_bytes()
            .to_vec()
            .into_iter()
            .skip_while(|&x| x == 0)
            .collect();

        assert_eq!(multiply_bytes(&vec1, &vec2), result);
    }

    #[test]
    fn test_random_multiply_bytes_2() {
        let num1: u64 = 23;
        let num2: u64 = 29423487;

        let vec1   = (num1).to_be_bytes().to_vec();
        let vec2   = (num2).to_be_bytes().to_vec();

        let result: Vec<u8> = (num1 * num2)
            .to_be_bytes()
            .to_vec()
            .into_iter()
            .skip_while(|&x| x == 0)
            .collect();

        assert_eq!(multiply_bytes(&vec1, &vec2), result);
    }

    #[test]
    fn test_random_multiply_bytes_3() {
        let num1: u64 = 456546;
        let num2: u64 = 2982317;

        let vec1   = (num1).to_be_bytes().to_vec();
        let vec2   = (num2).to_be_bytes().to_vec();

        let result: Vec<u8> = (num1 * num2)
            .to_be_bytes()
            .to_vec()
            .into_iter()
            .skip_while(|&x| x == 0)
            .collect();

        assert_eq!(multiply_bytes(&vec1, &vec2), result);
    }

    #[test]
    fn test_multiply_with_empty() {
        assert_eq!(multiply_bytes(&vec![1, 2, 3], &vec![]), vec![]);
    }

}

#[cfg(test)]
mod test_pow {
    use anybase::bytes_manipulation::*;

    // ----------------------------------------------- pow_bytes tests -----------------------------------------------

    #[test]
    fn test_pow_bytes_vs_multiply() {
        assert_eq!(
            pow_bytes(&vec![2, 3], 2), 
            multiply_bytes(&vec![2, 3], &vec![2, 3])
        );
    }

    #[test]
    fn test_pow_bytes_1() {
        let num: u64 = 2;
        let pow: u64 = 3;

        let vec   = (num).to_be_bytes().to_vec();
        let result: Vec<u8> = (num.pow(pow as u32))
            .to_be_bytes()
            .to_vec()
            .into_iter()
            .skip_while(|&x| x == 0)
            .collect();

        assert_eq!(pow_bytes(&vec, pow), result);
    }

    #[test]
    fn test_pow_bytes_2() {
        let num: u64 = 2345423;
        let pow: u64 = 3;

        let vec   = (num).to_be_bytes().to_vec();
        let result: Vec<u8> = (num.pow(pow as u32))
            .to_be_bytes()
            .to_vec()
            .into_iter()
            .skip_while(|&x| x == 0)
            .collect();

        assert_eq!(pow_bytes(&vec, pow), result);
    }

    #[test]
    fn test_pow_bytes_3() {
        let num: u64 = 12;
        let pow: u64 = 17;

        let vec   = (num).to_be_bytes().to_vec();
        let result: Vec<u8> = (num.pow(pow as u32))
            .to_be_bytes()
            .to_vec()
            .into_iter()
            .skip_while(|&x| x == 0)
            .collect();

        assert_eq!(pow_bytes(&vec, pow), result);
    }

    #[test]
    fn test_pow_with_1() {
        let num: u64 = 1;
        let pow: u64 = 17;

        let vec   = (num).to_be_bytes().to_vec();
        let result: Vec<u8> = (num.pow(pow as u32))
            .to_be_bytes()
            .to_vec()
            .into_iter()
            .skip_while(|&x| x == 0)
            .collect();

        assert_eq!(pow_bytes(&vec, pow), result);
    }

    #[test]
    fn test_pow_with_0() {
        let num: u64 = 0;
        let pow: u64 = 17;

        let vec   = (num).to_be_bytes().to_vec();
        let result: Vec<u8> = (num.pow(pow as u32))
            .to_be_bytes()
            .to_vec()
            .into_iter()
            .skip_while(|&x| x == 0)
            .collect();

        assert_eq!(pow_bytes(&vec, pow), result);
    }
}


#[cfg(test)]
mod test_encode {
    use anybase::storage::*;

    // ----------------------------------------------- encode tests -----------------------------------------------

    #[test]
    fn test_encode_empty_one_byte_1and0() {
        let alphabet: Vec<u8> = Vec::<u8>::from("01".as_bytes());
        let data: Vec<u8>     = Vec::<u8>::from("00000000".as_bytes());

        assert_eq!(encode(&alphabet, &data), vec![0 as u8]);
    }

    #[test]
    fn test_encode_empty_two_bytes_1and0() {
        let alphabet: Vec<u8> = Vec::<u8>::from("01".as_bytes());
        let data: Vec<u8>     = Vec::<u8>::from("0000000000000000".as_bytes());

        assert_eq!(encode(&alphabet, &data), vec![0 as u8, 0 as u8]);
    }

    #[test]
    fn test_encode_two_bytes_1and0() {
        let alphabet: Vec<u8> = Vec::<u8>::from("01".as_bytes());
        let data: Vec<u8>     = Vec::<u8>::from("0000000000001010".as_bytes());

        assert_eq!(encode(&alphabet, &data), vec![0 as u8, 10 as u8]);
    }

    #[test]
    fn test_encode_two_bytes_1and0_2() {
        let alphabet: Vec<u8> = Vec::<u8>::from("01".as_bytes());
        let data: Vec<u8>     = Vec::<u8>::from("0000000000011010".as_bytes());

        assert_eq!(encode(&alphabet, &data), vec![0 as u8, 26 as u8]);
    }
}










