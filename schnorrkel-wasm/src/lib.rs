extern crate wasm_bindgen;

use schnorrkel::{signing_context, PublicKey, SecretKey, Signature};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn stateless_sign(context: &[u8], seckey: &[u8], pubkey: &[u8], message: &[u8]) -> Vec<u8> {
    let context = signing_context(context);
    let seckey = SecretKey::from_bytes(seckey).unwrap();
    let pubkey = PublicKey::from_bytes(pubkey).unwrap();
    let signature = seckey.sign(context.bytes(message), &pubkey);
    signature.to_bytes().into()
}

#[wasm_bindgen]
pub fn stateless_verify(context: &[u8], pubkey: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let context = signing_context(context);
    let pubkey = PublicKey::from_bytes(pubkey).unwrap();
    let signature = Signature::from_bytes(signature).unwrap();
    pubkey.verify(context.bytes(&message), &signature).is_ok()
}
