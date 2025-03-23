use quinn::{ClientConfig, Endpoint};
use rustls::RootCertStore;
use std::{error::Error, net::{IpAddr, Ipv4Addr, SocketAddr}, sync::Arc};
use rustls_pki_types::{CertificateDer, pem::PemObject};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8090);

    // Criando um RootCertStore e adicionando o certificado autoassinado
    let mut root_cert_store = RootCertStore::empty();
    let cert = CertificateDer::from_pem_file("certs/cert.pem")?;
    root_cert_store.add(cert)?;

    let clien_config = ClientConfig::with_root_certificates(Arc::new(root_cert_store))?;

    // Criar o endpoint do cliente
    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(clien_config);

    // Conectar ao servidor QUIC
    let conn = endpoint.connect(server_addr, "localhost")?.await?;
    println!("Conectado ao servidor QUIC em {}", server_addr);

    // Abrir um stream bidirecional
    let (_send, _recv) = conn.open_bi().await?;
    println!("Stream aberto, enviando mensagem...");

    Ok(())
}
