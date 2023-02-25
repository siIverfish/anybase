
pub mod bytes_manipulation {
    fn split_word(x: u16) -> (u8, u8) {
        // woohoo bits!!
        ((x >> 8) as u8, (x & 0xFF) as u8)
    }
    
    pub fn add_bytes(larger: Vec<u8>, smaller: Vec<u8>) -> Vec<u8> {
        let mut carry: u8 = 0;
        let mut index: usize = larger.len() - 1;
    
        let mut new_bytes: Vec<u8> = vec![0; larger.len() + 1];
    
        loop {
            let (mut new_amount, carry_next): (u8, bool) =
                (larger[index] as u8).overflowing_add(smaller[index] as u8);
            new_amount += carry;
    
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
    
    pub fn multiply_bytes(larger: Vec<u8>, smaller: Vec<u8>) -> Vec<u8> { // TODO
        let mut new_bytes: Vec<u8> = vec![0; larger.len() * 2];
    
        for (index_1, byte_1) in (&larger).iter().enumerate() {
            for (index_2, byte_2) in (&smaller).iter().enumerate()  {
                let mut mult_result_vector: Vec<u8> = vec![0; index_1 + index_2 + 1];
                (mult_result_vector[0], mult_result_vector[1]) = split_word((*byte_1 as u16) * (*byte_2 as u16));
                if mult_result_vector[0] == 0 {
                    mult_result_vector = mult_result_vector[1..].to_vec();
                }
                new_bytes = add_bytes(new_bytes, mult_result_vector);
            }
        }
    
        new_bytes
    }
}