use hex_literal::hex;
use arc4::Arc4;

/*
const KEY2 : [u8;32] = hex!("1ada31d5cf688221c109163908ebe51debb46227c6cc8b37641910833222772a");
fn key2(key: &mut [u8]) {
    for i in 0 .. std::cmp::min(key.len(), KEY2.len()) {
        key[i] = KEY2[i];
    }
}
*/

fn key1(key: &mut [u8]) {
    for i in 0 .. key.len()  {
        key[i] = (i + 1) as u8;
    }
}

#[test]
fn t_40bit_key() {
    let mut key = [0; 5];
    key1(&mut key);

    let mut a = Arc4::with_key(&key);
    let mut b = [0;16];
    a.prga(&mut b);
    assert_eq!(b, hex!("b2 39 63 05  f0 3d c0 27   cc c3 52 4a  0a 11 18 a8"));
    a.prga(&mut b);
    assert_eq!(b, hex!("69 82 94 4f  18 fc 82 d5   89 c4 03 a4  7a 0d 09 19"));
}
