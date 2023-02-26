


// todo later
#[cfg(test)]
mod encode_decode_test {
    use anybase::*;

    macro_rules! encode_and_decode {
        ( $name:ident, $alphabet:expr, $data:expr ) => {
            #[test]
            pub fn $name() {
                assert_eq!($data, decode(&$alphabet, &encode(&$alphabet, &$data)));
            }
        };
    }

    encode_and_decode!(zero_at_end, vec![0, 1], vec![1, 0]);
    encode_and_decode!(zero_at_beginning, vec![0, 1], vec![0, 1]);
    encode_and_decode!(same_as_alphabet, vec![0, 1, 2, 3, 4, 5, 6, 7, 8], vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    encode_and_decode!(two_letter_alphabet, vec![0, 1], vec![0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0]);
    encode_and_decode!(random_y, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![3, 2, 5, 7, 1, 3, 6, 8, 5, 4, 6, 8, 0, 4, 2, 1, 2, 5, 4]);

    encode_and_decode!(much_stuff, vec![1, 2, 3, 4], vec![1, 2, 3, 4, 4, 4, 4, 4, 4, 1, 1, 2, 3, 4, 4, 1, 2, 3, 4, 2, 2, 2, 3, 4, 1, 1, 1, 2, 4, 3, 2, 1, 4, 4, 4, 4, 4, 4, 4, 4, 1, 3, 4, 1, 2, 3, 4, 4, 3, 4, 2, 1, 2]);

    encode_and_decode!(mass_encode, vec![0, 1, 2, 3, 4, 5, 6], vec![1; 1_809]);
}