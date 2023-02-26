
// unused stuff

pub mod bytes_manipulation {
    use crate::uint_splitting::*;

    // this should probably be a class in the future

    pub fn add_bytes(larger: &Vec<u8>, smaller: &Vec<u8>) -> Vec<u8> {
        // println!("larger: {:?}\nsmaller: {:?}", larger, smaller);

        let smaller = if smaller.len() < larger.len() {
            let mut new_smaller: Vec<u8> = vec![0; larger.len() - smaller.len()];
            new_smaller.extend_from_slice(&smaller);
            new_smaller
        } else {
            smaller.clone()
        };

        let mut carry: u8 = 0;
        let mut index: usize = larger.len() - 1;
    
        let mut new_bytes: Vec<u8> = vec![0; larger.len() + 1];
    
        loop {
            let (mut new_amount, mut carry_next): (u8, bool) =
                (larger[index] as u8).overflowing_add(smaller[index] as u8);
            
            if new_amount == 255 && carry == 1 {
                new_amount = 0;
                carry_next = true;
            } else {
                new_amount += carry;
            }
    
            new_bytes[index + 1] = new_amount;
    
            carry = carry_next as u8;
    
            if index == 0 {
                break;
            }
            index -= 1;
        }
    
        if carry == 1 {
            new_bytes[0] = 1;
            new_bytes[..].to_owned()
        } else {
            // let mut num_leading_zeroes: usize = 0;
            // while new_bytes[num_leading_zeroes] == 0 {
            //     num_leading_zeroes += 1;
            // }
            // println!("num leading zeroes: {}", num_leading_zeroes);
            // new_bytes[num_leading_zeroes..].to_owned()
            new_bytes[1..].to_owned()
        }
    }
    
    pub fn sub_bytes(larger: &Vec<u8>, smaller: &Vec<u8>) -> Vec<u8> {

        
        // manually handling edge cases
        if eq_bytes(larger, smaller) {
            return vec![];
        }

        // this is fiiine
        assert!(
            gt_bytes(larger, smaller),
            "the code subtracted weird. try again."
        );

        let mut new_vec: Vec<u8> = vec![0; larger.len()];
        let mut carry_subtractor: u8 = 0;

        // copied from add_bytes, should probably be a function
        let new_smaller = if smaller.len() < larger.len() {
            let mut new_smaller: Vec<u8> = vec![0; larger.len() - smaller.len()];
            new_smaller.extend_from_slice(&smaller);
            new_smaller
        } else {
            smaller.clone()
        };

        let smaller = &new_smaller;

        // note: larger_byte not actually larger
        for (index, (&larger_byte, &smaller_byte)) in larger.into_iter().zip(smaller).enumerate().rev() {
            let (new_amount, first_overflow)  = larger_byte.overflowing_sub(smaller_byte);
            let (new_amount, second_overflow) = new_amount.overflowing_sub(carry_subtractor);
            // index starts at new_value.len() - 1
            new_vec[index] = new_amount;
            carry_subtractor = (first_overflow || second_overflow) as u8;
        }

        strip_leading_zeroes(&new_vec)
    }

    pub fn multiply_bytes(larger: &Vec<u8>, smaller: &Vec<u8>) -> Vec<u8> {
        let mut new_bytes: Vec<u8> = vec![0; larger.len() * 2];

        if larger.iter().all(|&x| x == 0) || smaller.iter().all(|&x| x == 0) {
            return Vec::<u8>::new();
        }
    
        for (index_1, byte_1) in (&larger).iter().rev().enumerate() {
            for (index_2, byte_2) in (&smaller).iter().rev().enumerate()  {
                if (*byte_1 == 0) || (*byte_2 == 0) {
                    continue
                }

                let mut mult_result_vector: Vec<u8> = vec![0; index_1 + index_2 + 2];

                (mult_result_vector[0], mult_result_vector[1]) = split_word((*byte_1 as u16) * (*byte_2 as u16));

                if mult_result_vector[0] == 0 {
                    mult_result_vector = mult_result_vector[1..].to_vec();
                }

                new_bytes = add_bytes(&new_bytes, &mult_result_vector);
            }
        }

        let mut num_leading_zeroes: usize = 0;
        while new_bytes[num_leading_zeroes] == 0 {
            num_leading_zeroes += 1;
        }

        new_bytes[num_leading_zeroes..].to_vec()
    }

    pub fn pow_bytes(bytes: &Vec<u8>, magnitude: u64) -> Vec<u8> { // definitely needs to be optimized... later
        // TODO: add a hash table or something funny

        let mut result = bytes.clone();

        for _ in 1..magnitude {
            result = multiply_bytes(&result, &bytes)
        }

        result
    }

    pub fn strip_leading_zeroes(bytes: &Vec<u8>) -> Vec<u8> {
        let mut num_leading_zeroes: usize = 0;
        while bytes[num_leading_zeroes] == 0 {
            num_leading_zeroes += 1;
        }
        bytes[num_leading_zeroes..].to_owned()
    }

    pub fn gt_bytes(bytes1: &Vec<u8>, bytes2: &Vec<u8>) -> bool { // this could probably be a macro that does all the operations but i dont know how to do that
        let bytes1: Vec<u8> = strip_leading_zeroes(bytes1);
        let bytes2: Vec<u8> = strip_leading_zeroes(bytes2);

        // if one is longer then it's the greater one
        if bytes1.len() != bytes2.len() {
            return bytes1.len() > bytes2.len();
        }

        // loop through them -- big endian
        for (byte1, byte2) in bytes1.into_iter().zip(bytes2) {
            if byte1 != byte2 {
                return byte1 > byte2;
            }
        }

        // defaults to false if they're equal
        false
    }

    pub fn lt_bytes(bytes1: &Vec<u8>, bytes2: &Vec<u8>) -> bool {
        let bytes1: Vec<u8> = strip_leading_zeroes(bytes1);
        let bytes2: Vec<u8> = strip_leading_zeroes(bytes2);

        // if one is longer then it's the greater one
        if bytes1.len() != bytes2.len() {
            return bytes1.len() < bytes2.len();
        }

        // loop through them -- big endian
        for (byte1, byte2) in bytes1.into_iter().zip(bytes2) {
            if byte1 != byte2 {
                return byte1 < byte2;
            }
        }

        // defaults to false if they're equal
        false
    }

    pub fn gte_bytes(bytes1: &Vec<u8>, bytes2: &Vec<u8>) -> bool { // !lt_bytes
        !lt_bytes(bytes1, bytes2)
    }

    pub fn lte_bytes(bytes1: &Vec<u8>, bytes2: &Vec<u8>) -> bool { // !gt_bytes
        !gt_bytes(bytes1, bytes2)
    }

    pub fn eq_bytes(bytes1: &Vec<u8>, bytes2: &Vec<u8>) -> bool {
        let bytes1: Vec<u8> = strip_leading_zeroes(bytes1);
        let bytes2: Vec<u8> = strip_leading_zeroes(bytes2);

        if bytes1.len() != bytes2.len() {
            return false;
        }

        for (b1, b2) in bytes1.into_iter().zip(bytes2) {
            if b1 != b2 {
                return false;
            }
        }

        true
    }

    pub fn mod_bytes(bytes: &Vec<u8>, modulus: &Vec<u8>) -> Vec<u8> { // todo: binary search here like decent person & this function cant have modulus 1
        
        let mut upper_bound: u64 = u64::MAX;
        let mut lower_bound: u64 = u64::MIN;

        loop {
            let middle:      u64 = (upper_bound / 2) + (lower_bound / 2); // cant add before dividing or overflow

            let result = &multiply_bytes(
                modulus, 
                &split_qword(middle).to_vec()
            );

            if result > bytes {
                upper_bound = middle;
            } else if result < bytes {
                lower_bound = middle;
            } else {
                return Vec::<u8>::new();
            }
        }


    }



macro_rules! test_against_function {
    ( $name:ident, $test_function:expr, $against:expr, $arg1:expr, $arg2:expr ) => {
        #[test]
        pub fn $name() {
            let vec1   = ($arg1 as u64).to_be_bytes().to_vec();
            let vec2   = ($arg2 as u64).to_be_bytes().to_vec();

            let result: Vec<u8> = ( $against( $arg1 as u64, $arg2 as u64 ) )
                .to_be_bytes()
                .to_vec()
                .into_iter()
                .skip_while(|&x| x == 0)
                .collect();

            assert_eq!($test_function(&vec1, &vec2), result);
        }
    };
}


#[cfg(test)]
mod test_add {
    use anybase::bytes_manipulation::*;

    // ---------------------------- add_bytes tests ----------------------------

    #[test]
    fn test_add_bytes() {
        assert_eq!(add_bytes(&vec![1, 2, 3], &vec![4, 5, 6]), vec![5, 7, 9]);
    }

    #[test]
    fn test_add_bytes_overflow() {
        assert_eq!(add_bytes(&vec![255, 255, 255], &vec![255, 255, 255]), vec![1, 255, 255, 254]);
    }

    #[test]
    fn test_add_bytes_symmetry() {
        assert_eq!(add_bytes(&vec![1, 2, 3], &vec![4, 5, 6]), add_bytes(&vec![4, 5, 6], &vec![1, 2, 3]))
    }

    #[test]
    fn test_add_bytes_one_small() {
        assert_eq!(add_bytes(&vec![1, 2, 3], &vec![4, 5]), vec![1, 6, 8]);
    }

    #[test]
    fn test_add_bytes_max_small() {
        assert_eq!(add_bytes(&vec![255, 255, 255, 255], &vec![1]), vec![1, 0, 0, 0, 0]);
    }


    #[test]
    fn test_add_with_empty() {
        assert_eq!(add_bytes(&vec![1, 2, 3], &vec![]), vec![1, 2, 3]);
    }
}

#[cfg(test)]
mod test_sub { 
    use anybase::bytes_manipulation::*;

    // ---------------------------- sub_bytes tests ----------------------------

    #[test]
    fn test_simple_sub_bytes() {
        assert_eq!(sub_bytes(&vec![4, 5, 6], &vec![1, 2, 3]), vec![3, 3, 3]);
    }

    #[test]
    fn test_overflow_sub_bytes() {
        assert_eq!(sub_bytes(&vec![1, 0, 0], &vec![0, 0, 1]), vec![255, 255]);
    }

    #[test]
    fn test_overflow_sub_bytes_no_padding() {
        assert_eq!(sub_bytes(&vec![1, 0, 0], &vec![1]), vec![255, 255]);
    }

    #[test]
    fn same() {
        assert_eq!(sub_bytes(&vec![5, 34, 123], &vec![5, 34, 123]), Vec::<u8>::new());
    }

    #[test]
    fn more_padding_zeroes() {
        // todo
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

    
    test_against_function!(test_random_multiply_bytes_1, multiply_bytes, u64::wrapping_mul, 340598, 2987);
    test_against_function!(test_random_multiply_bytes_2, multiply_bytes, u64::wrapping_mul, 23, 29423487);
    test_against_function!(test_random_multiply_bytes_3, multiply_bytes, u64::wrapping_mul, 456546, 2982317);


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

mod test_encode {
    use anybase::storage::*;

    // ----------------------------------------------- encode tests -----------------------------------------------

    #[test]
    fn test_encode_empty_one_byte_1and0() {
        let alphabet: Vec<u8> = Vec::<u8>::from("01".as_bytes());
        let data: Vec<u8>     = Vec::<u8>::from("00000000".as_bytes());

        assert_eq!(encode(&alphabet, &data), vec![1 as u8, 0 as u8]);
    }

    #[test]
    fn test_encode_empty_two_bytes_1and0() {
        let alphabet: Vec<u8> = Vec::<u8>::from("01".as_bytes());
        let data: Vec<u8>     = Vec::<u8>::from("0000000000000000".as_bytes());

        assert_eq!(encode(&alphabet, &data), vec![1, 0 as u8, 0 as u8]);
    }

    #[test]
    fn test_encode_two_bytes_1and0() {
        let alphabet: Vec<u8> = Vec::<u8>::from("01".as_bytes());
        let data: Vec<u8>     = Vec::<u8>::from("0000000000001010".as_bytes());

        assert_eq!(encode(&alphabet, &data), vec![1, 0 as u8, 10 as u8]);
    }

    #[test]
    fn test_encode_two_bytes_1and0_2() {
        let alphabet: Vec<u8> = Vec::<u8>::from("01".as_bytes());
        let data: Vec<u8>     = Vec::<u8>::from("0000000000011010".as_bytes());

        assert_eq!(encode(&alphabet, &data), vec![1, 0 as u8, 26 as u8]);
    }

    #[test]
    fn test_encode_two_bytes_1and0_2_other_alphabet() {
        let alphabet: Vec<u8> = Vec::<u8>::from("ab".as_bytes());
        let data:     Vec<u8> = Vec::<u8>::from("aaaaaaaaaaabbaba".as_bytes());

        assert_eq!(encode(&alphabet, &data), vec![1, 0 as u8, 26 as u8]);
    }

    #[test]
    fn test_encode_two_bytes_1and0_2_other_alphabet_2() {
        let alphabet: Vec<u8> = Vec::<u8>::from("abc".as_bytes());
        let data:     Vec<u8> = Vec::<u8>::from("acaabbaba".as_bytes());

        assert_eq!(encode(&alphabet, &data), vec![55, 248]);

    }

    // #[test]
    // fn test_encode_vs_base32() {
    //     let alphabet: Vec<u8> = Vec::<u8>::from("abcdefghijklmnopqrstuvwxyz ".as_bytes());
    //     let data:     Vec<u8> = Vec::<u8>::from("hello world".as_bytes());
        
    //     println!("data: {:?}", data);
    //     println!("alphabet: {:?}", alphabet);
    //     println!("{:?}", encode(&alphabet, &data));

    //     assert!(false);
    // }
}

mod test_mod_bytes {
    use anybase::bytes_manipulation::*;

    // ----------------------------------------------- mod_bytes tests -----------------------------------------------

    #[test]
    fn test_mod_bytes_1() {
        let num: u64 = 345;
        let mod_: u64 = 3;

        let vec1   = (num).to_be_bytes().to_vec();
        let vec2  = (mod_).to_be_bytes().to_vec();
        let result: Vec<u8> = (num % mod_)
            .to_be_bytes()
            .to_vec()
            .into_iter()
            .skip_while(|&x| x == 0)
            .collect();

        assert_eq!(mod_bytes(&vec1, &vec2), result);
    }

    #[test]
    fn test_mod_bytes_2() {
        let num: u64 = 23464456;
        let mod_: u64 = 345;

        let vec1   = (num).to_be_bytes().to_vec();
        let vec2  = (mod_).to_be_bytes().to_vec();
        let result: Vec<u8> = (num % mod_)
            .to_be_bytes()
            .to_vec()
            .into_iter()
            .skip_while(|&x| x == 0)
            .collect();

        assert_eq!(mod_bytes(&vec1, &vec2), result);
    }

}








