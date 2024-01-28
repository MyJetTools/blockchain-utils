use anyhow::Result;
use secp256k1::rand::{rngs, SeedableRng};
use secp256k1::{PublicKey, SecretKey};

use web3::signing::keccak256;
use web3::transports::Http;
use web3::types::{Address, TransactionParameters, H160, U256};
use web3::Web3;

pub struct KeyPair {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
}

pub fn create_key_pair() -> Result<KeyPair> {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(6);

    let result = secp.generate_keypair(&mut rng);
    Ok(KeyPair {
        secret_key: result.0,
        public_key: result.1,
    })
}

pub fn establish_web3_connection(url: &str) -> Result<Web3<Http>> {
    let transport = web3::transports::Http::new(url)?;
    Ok(web3::Web3::new(transport))
}

pub fn create_txn_object(to: H160, value: usize) -> Result<TransactionParameters> {
    Ok(TransactionParameters {
        to: Some(to),
        //todo: check value
        value: U256::exp10(value), //0.1 eth
        ..Default::default()
    })
}

pub fn address_from_public_key(public_key: &PublicKey) -> Address {
    let public_key = public_key.serialize_uncompressed();
    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]);
    return Address::from_slice(&hash[12..]);
}

/*
pub async fn sign_and_send(
    web3: Web3<Http>,
    tx_object: TransactionParameters,
    secret_key: SecretKey,
) -> Result<H256> {
    let signed = web3
        .accounts()
        .sign_transaction(tx_object, &secret_key)
        .await?;
    Ok(web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?)
}
*/
