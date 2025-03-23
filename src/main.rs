use rcgen::{generate_simple_self_signed, CertifiedKey};
use std::path::Path;
use std::fs;

fn main() {
    let subject_alt_names = vec!["localhost".to_string()];

    let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names).unwrap();

    if !Path::new("certs").exists() {
        fs::create_dir_all("certs").expect("Failed to create 'certs' directory");
    }

    let _ = fs::write("certs/cert.pem", cert.pem());
    let _ = fs::write("certs/cert.key", key_pair.serialize_pem());
}
