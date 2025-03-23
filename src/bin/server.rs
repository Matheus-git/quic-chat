use quinn::{Endpoint, ServerConfig};
use std::{error::Error, net::SocketAddr};
use rustls_pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "127.0.0.1:8090".parse()?;

    let cert = CertificateDer::from_pem_file("certs/cert.pem")?;
    let private_key = PrivateKeyDer::from_pem_file("certs/cert.key")?;

    let server_config = ServerConfig::with_single_cert(vec![cert], private_key )?;

    let endpoint = Endpoint::server(server_config, addr)?;
    println!("Servidor QUIC rodando em {}", addr);

    while let Some(conn) = endpoint.accept().await {
        tokio::spawn(async move {
            match conn.await {
                Ok(new_conn) => {
                    println!("Nova conexão de {:?}", new_conn.remote_address());
                    // if let Ok((mut send, mut recv)) = new_conn.accept_bi().await {
                    //     let mut buf = vec![1024];
                    // }
                }
                Err(e) => eprintln!("Erro na conexão: {:?}", e),
            }
        });
    }

    Ok(())
}
