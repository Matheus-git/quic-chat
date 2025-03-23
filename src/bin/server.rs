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
                    
                    match new_conn.accept_bi().await {
                        Ok((mut send, mut recv)) => {
                            println!("Nova conexão de {:?}", new_conn.remote_address());
                            let mut buf = vec![0; 1024]; // Buffer de 1024 bytes
                            
                            match recv.read(&mut buf).await {
                                Ok(_size) => {
                                    println!("Recebido: {}", String::from_utf8_lossy(&buf));
                                    send.write("Olá de volta".as_bytes()).await
                                        .expect("erro ao enviar mensagem");
                                }
                                Ok(_) => {
                                    println!("Conexão fechada pelo remetente.");
                                }
                                Err(e) => {
                                    eprintln!("Erro ao ler da conexão: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Erro ao aceitar conexão: {}", e);
                        }
                    }
                }
                Err(e) => eprintln!("Erro na conexão: {:?}", e),
            }
        });
    }

    Ok(())
}
