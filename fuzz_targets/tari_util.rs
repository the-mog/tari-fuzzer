extern crate tari_utilities;
use self::tari_utilities::{hex,bit};


//takes a &[T] and returns a String
pub fn util_to_hex(data: &[u8]) {
    hex::to_hex(data);
    
}

//takes in u8 and returns Vec<bool>
pub fn util_bytes_to_bits(data: &[u8]) {
    bit::bytes_to_bits(data);
}

//takes a &str and returns a Vec<u8>
pub fn util_from_hex(data: &[u8]) {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = hex::from_hex(&s); 
    }
}

//takes &[Vec<u8>] and returns a Vec<String>
pub fn util_to_hex_multiple(data: &[u8]) {
    let s = [data.to_vec()];
    hex::to_hex_multiple(&s);
}

