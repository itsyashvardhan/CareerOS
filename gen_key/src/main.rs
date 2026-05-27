use minisign::KeyPair;
use std::fs;

fn main() {
    let KeyPair { pk, sk } = KeyPair::generate_encrypted_keypair(Some("".to_string()))
        .expect("Failed to generate keypair");

    let pk_str = pk.to_box().unwrap();
    let sk_str = sk.to_box(None).unwrap();

    let pub_key = pk_str.into_string();
    let sec_key = sk_str.into_string();

    fs::create_dir_all(r"C:\Users\me\.tauri").unwrap();
    fs::write(r"C:\Users\me\.tauri\careeros.key", &sec_key).expect("Failed to write secret key");
    fs::write(r"C:\Users\me\.tauri\careeros.pub.txt", &pub_key).expect("Failed to write public key");

    println!("Public key:\n{}", pub_key);
    println!("Secret key written to: C:\\Users\\me\\.tauri\\careeros.key");
}
