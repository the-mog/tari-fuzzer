use tari_core::transactions::{
            fee::Fee,
                    helpers::{create_random_signature_from_s_key, create_utxo, spend_utxos},
                            tari_amount::uT,
                                    transaction::{KernelBuilder, KernelFeatures, OutputFeatures, UnblindedOutput},
                                            types::{Commitment, CryptoFactories},
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
