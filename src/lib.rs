


pub mod storage {
    use std::collections::HashMap;
    use num_bigint::BigUint; 

    pub fn encode(byte_alphabet: &Vec<u8>, data: &Vec<u8>) -> Vec<u8> {
        println!("ENCODING ALPHABET LEN: {}", byte_alphabet.len());

        // todo verify alphabet

        // create mapping from alphabet to data number
        let mut alphabet_map: HashMap<u8, u8> = HashMap::new();
        for (index, &item) in byte_alphabet.into_iter().enumerate() {
            alphabet_map.insert(item, (index as u8) + 1);
        }

        println!("ENCODING ALPHABET MAP: {:?}", alphabet_map);

        let mut new_data: BigUint = BigUint::new( vec![0] );
        let big_alphabet_len = BigUint::from( byte_alphabet.len() );

        for (i, datum) in data
            .into_iter()
            .map( |x| alphabet_map[x] )
            .enumerate() 
        {
            let datum: BigUint = BigUint::from(datum) * big_alphabet_len.pow(i as u32);
            new_data += datum;
        }

        new_data.to_bytes_be() // possibly change
    }

    pub fn decode(byte_alphabet: &Vec<u8>, data: &Vec<u8>) -> Vec<u8> {
        // create mapping from data number back to original
        let mut alphabet_map: HashMap<u8, u8> = HashMap::new();
        for (index, &item) in byte_alphabet.into_iter().enumerate() {
            alphabet_map.insert((index as u8) + 1, item);
        }
        
        println!("ALPHABET MAP: {:?}", alphabet_map);

        let alphabet_len_big: BigUint = BigUint::from(byte_alphabet.len() as u32);

        // get num of chars in data
        let mut approx_data_len: usize = ((data.len() as f32) * 256.0_f32.log(byte_alphabet.len() as f32)).ceil() as usize;
        
        println!("DECODED DATA LEN (FIRST ESTIMATE): {}", approx_data_len);

        let mut data: BigUint = BigUint::from_bytes_be(data);
        let mut new_data: Vec<u8> = Vec::new();

        while alphabet_len_big.pow(approx_data_len as u32) > data {
            println!("Too big! {}  vs {}", alphabet_len_big.pow(approx_data_len as u32), data);
            approx_data_len -= 1_usize;
        }
        // data len SHOULD be correct now
        println!("DECODED DATA LEN: {}", approx_data_len);

        let alphabet_len: usize = byte_alphabet.len();
        for i in 0..approx_data_len {
            println!("DATA: {} {}", i, data);
            let key: &u8 = &(((&data % alphabet_len).to_bytes_le()[0] & 0xFF));
            println!("before [0] &: {:?}", ( &data % alphabet_len ).to_bytes_le() );
            println!("KEY: {}", key);
            println!("ADDING {}", alphabet_map[ key ]);
            println!();
            new_data.push( alphabet_map[ key ] );
            data /= alphabet_len;
        }

        new_data
    }

}