
use anybase::bytes_manipulation::multiply_bytes;

fn main() {
    assert_eq!(multiply_bytes(vec![1, 2, 3], vec![4, 5, 6]), vec![4, 13, 28, 27, 18]);
}