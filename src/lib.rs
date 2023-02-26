
mod uint_splitting {
    // TODO make `add_bytes` accept an array of u8s so this can stop using tuples

    pub fn split_word(x: u16) -> (u8, u8) {
        // woohoo bits!!
        ((x >> 8) as u8, (x & 0xFF) as u8)
    }
    
    pub fn split_dword(x: u32) -> [u8; 4] {
        [
            (x >> 24) as u8 & 0xFF,
            (x >> 16) as u8 & 0xFF,
            (x >> 8)  as u8 & 0xFF,
            (x >> 0)  as u8 & 0xFF
        ]
    }
}

pub mod bytes_manipulation {
    use std::num;

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

    pub fn gt_bytes(bytes1: &Vec<u8>, bytes2: &Vec<u8>) -> bool { // this could probably be a macro that does all the operations
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

    pub fn mod_bytes(bytes: &Vec<u8>, modulus: &Vec<u8>) -> Vec<u8> { // todo: binary search here like decent person
        let mut multiplier: u32 = 1;

        while gt_bytes(
            bytes, 
            &multiply_bytes(
                modulus, 
                &split_dword(multiplier).to_vec()
            )
        ) {
            multiplier += 1;
        }

        println!("{:?}", bytes);
        println!("{:?}", 
            &multiply_bytes(
                modulus,
                &split_dword(multiplier).to_vec()
            )
        );

        sub_bytes(
            bytes,
            &multiply_bytes(
                modulus,
                &split_dword(multiplier).to_vec()
            )
        )
    }

}

pub mod storage {
    use std::collections::HashMap;
    use crate::bytes_manipulation::*;
    use crate::uint_splitting::split_dword;

    pub fn encode(byte_alphabet: &Vec<u8>, data: &Vec<u8>) -> Vec<u8> { // optimize later idk
        // create mapping from alphabet to data number
        let alphabet_len: f32 = byte_alphabet.len() as f32;
        let alphabet_len_as_bytes: Vec<u8> = split_dword(byte_alphabet.len() as u32).to_vec();

        let mut alphabet_map: HashMap<u8, u8> = HashMap::new();
        for (index, &item) in byte_alphabet.into_iter().enumerate() {
            // println!("{}", item);
            alphabet_map.insert(item, (index as u8) + 1);
        }

        // turn the data into vector of map 
        // all of these numbers will be < byte_alphabet.len()
        let data: Vec<u8> = data
            .iter()
            .map(|x| {
                // println!("{}", x);
                alphabet_map[x]
            })
            .collect();

        // llvm will fix this right?
        let chars_per_byte: f32 = (256 as f32).log(alphabet_len);
        let num_bytes: usize = (data.len() as f32 / chars_per_byte).ceil() as usize;
        let mut new_data: Vec<u8> = vec![0; num_bytes];

        // uncomment for debugging
        println!("chars_per_byte: {}", chars_per_byte);
        println!("(data.len() as f32 / chars_per_byte): {}", (data.len() as f32 / chars_per_byte));
        println!("num_bytes: {}", num_bytes);


        for (i, datum) in data.iter().rev().enumerate() {
            let datum: Vec<u8> = multiply_bytes(
                &vec![*datum], 
                &pow_bytes(
                    &alphabet_len_as_bytes, 
                    i as u64
                )
            );
            println!("DATUM: {:?}", datum);
            new_data = add_bytes(&new_data, &datum);
            // uncomment for debugging
            println!("NEW: {:?}", new_data);
            println!("byte power: {:?}", 
                &pow_bytes(
                    &alphabet_len_as_bytes, 
                    (i as u64) + 1
                )
            );
            print!("\n");
        }

        new_data
    }

    pub fn decode(_alphabet: Vec<u8>, _data: Vec<u8>) -> Vec<u8> {
        todo!();
    }

}