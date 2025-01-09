use rand::Rng;
use tonlib::mnemonic::KeyPair;

fn main() {
    // Generate a random 32-byte secret key
    let mut rng = rand::thread_rng();
    let mut private_key_bytes = [0u8; 32];
    rng.fill(&mut private_key_bytes);

    // Generate key pair
    let key_pair = nacl::sign::generate_keypair(&private_key_bytes.to_vec());

    // Extract keys
    let public_key = key_pair.pkey;
    let secret_key = key_pair.skey;

    // Derive V4 wallet
    let wallet = tonlib::wallet::TonWallet::derive(
        0,
        tonlib::wallet::WalletVersion::V4R2,
        &KeyPair {
            public_key: public_key.to_vec(),
            secret_key: secret_key.to_vec(),
        },
        1,
    ).unwrap();

    // Print out Wallet info.
    println!("Address: {}", wallet.address);
    println!("Public Key: {:?}", wallet.key_pair.public_key);
    println!("Private Key: {:?}", wallet.key_pair.secret_key);
}
