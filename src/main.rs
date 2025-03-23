use rcgen::{generate_simple_self_signed, CertifiedKey};

fn main() {
    let subject_alt_names = vec!["hello.world.example".to_string(),
	"localhost".to_string()];

    let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names).unwrap();
    let _ = std::fs::write("certs/cert.pem",cert.pem());
    let _ = std::fs::write("certs/cert.key",key_pair.serialize_pem());
}
