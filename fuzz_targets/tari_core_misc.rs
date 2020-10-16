use rand::rngs::OsRng;
use tari_core::transactions::{
            fee::Fee,
            helpers::{create_random_signature_from_s_key, create_utxo, spend_utxos},
            tari_amount::uT,
            transaction::{KernelBuilder, KernelFeatures, OutputFeatures, UnblindedOutput},
            transaction_protocol::{proto, recipient::RecipientSignedMessage, sender::TransactionSenderMessage}, 
            types::{CryptoFactories, HashDigest, PrivateKey, PublicKey},
 };



// accept string and return &mut Self
pub fn core_trx_with_message(data: &[u8]) {
    let factories = CryptoFactories::default();
    let mut builder = SenderTransactionProtocol::builder(1);
    let amount = MicroTari::from(10_000);
    let input = UnblindedOutput::new(MicroTari::from(100_000), PrivateKey::random(&mut OsRng), None);
    builder.with_lock_height(0).with_fee_per_gram(MicroTari::from(177)).with_offset(PrivateKey::random(&mut OsRng)).with_private_nonce(PrivateKey::random(&mut OsRng)).with_amount(0, amount).with_message("Yo!".to_string()).with_input(input.as_transaction_input(&factories.commitment, OutputFeatures::default()),input.clone(),).with_change_secret(PrivateKey::random(&mut OsRng));

    //if let Ok(s) = std::str::from_utf8(data) {
      //  with_message(s);


    //}
}
