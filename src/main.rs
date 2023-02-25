

fn add_bytes(larger: Vec<u8>, smaller: Vec<u8>) -> Vec<u8> {
    let mut carry: u8 = 0;
    let mut index: usize = larger.len() - 1;

    let mut new_bytes: Vec<u8> = vec![0; larger.len()+1];

    loop {
        let (mut new_amount, carry_next): (u8, bool) = (larger[index] as u8).overflowing_add(smaller[index] as u8);
        new_amount += carry;

        new_bytes[index+1] = new_amount;

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

fn main() {
    // test add bytes
    let b = vec![1, 2, 3, 4, 5];
    let c = vec![1, 2, 3, 4, 5];
    assert_eq!(add_bytes(b, c), vec![2, 4, 6, 8, 10]);

    // test with max values
    let b = vec![255, 255, 255, 255, 255];
    let c = vec![255, 255, 255, 255, 255];
    assert_eq!(add_bytes(b, c), vec![1, 255, 255, 255, 255, 254]);
}
