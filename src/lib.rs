


pub mod storage {
    use std::collections::HashMap;
    use num_bigint::BigUint; 

    fn verify_alphabet(byte_alphabet: &[u8]) {
        // verify unique alphabet in n^2 time but a hashmap would probably be overkill
        assert!(
            byte_alphabet.iter().all(
                |x| byte_alphabet
                        .iter()
                        .filter(|&other| other == x)
                        .count() == 1
            ),
            "Alphabet must have all unique characters!"
        );
    }

    pub fn encode(byte_alphabet: &[u8], data: &[u8]) -> Vec<u8> {
        verify_alphabet(byte_alphabet);

        // create mapping from alphabet to data number
        let mut alphabet_map: HashMap<u8, u8> = HashMap::new();
        for (index, &item) in byte_alphabet.into_iter().enumerate() {
            alphabet_map.insert(item, (index as u8) + 1);
        }

        let mut new_data: BigUint = BigUint::new( vec![0] );
        let modulus = BigUint::from( byte_alphabet.len() + 1 );

        for (i, datum) in data
            .into_iter()
            .map( |x| alphabet_map[x] )
            .enumerate() 
        {
            let datum: BigUint = BigUint::from(datum) * modulus.pow(i as u32);
            new_data += datum;
        }

        new_data.to_bytes_le() // possibly change
    }

    pub fn decode(byte_alphabet: &[u8], data: &[u8]) -> Vec<u8> {
        verify_alphabet(byte_alphabet);

        // create mapping from data number back to original
        let mut alphabet_map: HashMap<u8, u8> = HashMap::new();
        for (index, &item) in byte_alphabet.into_iter().enumerate() {
            alphabet_map.insert((index as u8) + 1, item);
        }

        let alphabet_len_big: BigUint = BigUint::from(byte_alphabet.len() as u32);

        // get num of chars in data
        let mut approx_data_len: usize = ((data.len() as f32) * 256.0_f32.log(byte_alphabet.len() as f32)).ceil() as usize;

        let mut data: BigUint = BigUint::from_bytes_le(data);
        let mut new_data: Vec<u8> = Vec::new();

        while alphabet_len_big.pow(approx_data_len as u32) >= data {
            approx_data_len -= 1_usize;
        }

        let alphabet_len: usize = byte_alphabet.len();
        loop {
            let key: &u8 = &((&data % (alphabet_len + 1)).to_bytes_le()[0] & 0xFF);
            new_data.push( alphabet_map[ key ] );
            data /= alphabet_len + 1;

            if data.bits() == 0 {
                break;
            }
        }

        new_data
    }

}