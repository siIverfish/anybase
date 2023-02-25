
pub mod bytes_manipulation {
    fn split_word(x: u16) -> (u8, u8) {
        // woohoo bits!!
        ((x >> 8) as u8, (x & 0xFF) as u8)
    }
    
    pub fn add_bytes(larger: Vec<u8>, smaller: Vec<u8>) -> Vec<u8> {
        println!("larger: {:?}\nsmaller: {:?}", larger, smaller);

        let smaller = if smaller.len() < larger.len() {
            let mut new_smaller: Vec<u8> = vec![0; larger.len() - smaller.len()];
            new_smaller.extend_from_slice(&smaller);
            new_smaller
        } else {
            smaller
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
            new_bytes[1..].to_owned()
        }
    }
    
    pub fn multiply_bytes(larger: Vec<u8>, smaller: Vec<u8>) -> Vec<u8> {
        let mut new_bytes: Vec<u8> = vec![0; larger.len() * 2];
    
        for (index_1, byte_1) in (&larger).iter().rev().enumerate() {
            for (index_2, byte_2) in (&smaller).iter().rev().enumerate()  {
                let mut mult_result_vector: Vec<u8> = vec![0; index_1 + index_2 + 2];

                (mult_result_vector[0], mult_result_vector[1]) = split_word((*byte_1 as u16) * (*byte_2 as u16));

                if mult_result_vector[0] == 0 {
                    mult_result_vector = mult_result_vector[1..].to_vec();
                }

                new_bytes = add_bytes(new_bytes, mult_result_vector);
            }
        }

        let mut num_leading_zeroes: usize = 0;
        while new_bytes[num_leading_zeroes] == 0 {
            num_leading_zeroes += 1;
        }

        new_bytes[num_leading_zeroes..].to_vec()
    }
}

pub mod storage {
    use std::collections::HashMap;

    pub fn encode(byte_alphabet: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
        // create mapping from alphabet to data number
        let mut map: HashMap<u8, u8> = HashMap::new();
        for (index, item) in byte_alphabet.into_iter().enumerate() {
            map.insert(item, index as u8);
        }

        todo!();
    }

    pub fn decode(alphabet: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
        todo!();
    }
}