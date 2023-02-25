
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
    
    pub fn multiply_bytes(larger: &Vec<u8>, smaller: &Vec<u8>) -> Vec<u8> {
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

    pub fn pow_bytes(bytes: &Vec<u8>, magnitude: u64) -> Vec<u8> { // definitely needs to be optimized... later
        // TODO: add a hash table or something funny

        let mut result = bytes.clone();

        for _ in 1..magnitude {
            result = multiply_bytes(&result, &bytes)
        }

        result
    }
}

pub mod storage {
    use std::collections::HashMap;

    pub fn encode(byte_alphabet: Vec<u8>, data: Vec<u8>) -> Vec<u8> { // optimize later idk
        // create mapping from alphabet to data number
        let alphabet_len: f32 = byte_alphabet.len() as f32;

        let mut alphabet_map: HashMap<u8, u8> = HashMap::new();
        for (index, item) in byte_alphabet.into_iter().enumerate() {
            alphabet_map.insert(item, index as u8);
        }

        // turn the data into vector of map 
        // all of these numbers will be < byte_alphabet.len()
        let data: Vec<u8> = data
            .iter()
            .map(|x| alphabet_map[x])
            .collect();

        // llvm will fix this right?
        let chars_per_byte: f32 = 256.0 / alphabet_len;
        let num_bytes: usize = (chars_per_byte * data.len() as f32).ceil() as usize;
        let mut new_data: Vec<u8> = vec![0; num_bytes];

        for (i, datum) in data.iter().enumerate() {

        }

        todo!();
    }

    pub fn decode(alphabet: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
        todo!();
    }
}