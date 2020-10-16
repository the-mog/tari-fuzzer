use tari_core::transactions::{
    transaction::{OutputFeatures},
};



// accept u64
pub fn core_trx_create_coinbase(data: &[u8]) {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(d) = u64::from_str_radix(s, 36) {
            OutputFeatures::create_coinbase(d);

        }


    }
}
// accept u64
pub fn core_trx_with_maturity(data: &[u8]) {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(d) = u64::from_str_radix(s, 36) {
            OutputFeatures::with_maturity(d);

        }


    }
}
