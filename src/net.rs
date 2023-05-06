use std::{error::Error, net::SocketAddr, sync::Arc};

use anyhow::{self, Result};
use quinn::{ClientConfig, Endpoint, ServerConfig};
pub struct Net {
    pub endpoint: Endpoint,
    pub client_endpoint: Endpoint,
}

impl Net {
    pub fn new(bind_addr: SocketAddr) -> Self {
        Self {
            endpoint: Self::make_server_endpoint(bind_addr).unwrap(),
            client_endpoint: Self::make_client_endpoint().unwrap(),
        }
    }

    fn client_addr() -> SocketAddr {
        "127.0.0.1:4358".parse::<SocketAddr>().unwrap()
    }

    fn server_addr() -> SocketAddr {
        "127.0.0.1:4438".parse::<SocketAddr>().unwrap()
    }

    fn configure_client(server_certs: Vec<u8>) -> Result<ClientConfig, Box<dyn Error>> {
        let mut certs = rustls::RootCertStore::empty();
        for cert in server_certs {
            certs.add(&rustls::Certificate(vec![cert]))?;
        }

        let mut client_config = ClientConfig::with_root_certificates(certs);

        Ok(client_config)
    }

    fn make_client_endpoint() -> Result<(Endpoint)> {
        let (_, der) = Self::configure_server().unwrap();
        let client_cfg = Self::configure_client(der).unwrap();
        let mut endpoint = Endpoint::client(Self::server_addr()).unwrap();
        endpoint.set_default_client_config(client_cfg);
        Ok(endpoint)
    }

    fn make_server_endpoint(bind_addr: SocketAddr) -> Result<Endpoint> {
        let (server_config, server_cert) = Self::configure_server()?;
        let mut endpoint = Endpoint::server(server_config, bind_addr)?;
        let client_cfg = Self::configure_client(server_cert.into()).unwrap();
        endpoint.set_default_client_config(client_cfg);
        Ok(endpoint)
    }

    fn configure_server() -> Result<(ServerConfig, Vec<u8>)> {
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let cert_der = cert.serialize_der().unwrap();
        let priv_key = cert.serialize_private_key_der();
        let priv_key = rustls::PrivateKey(priv_key);
        let cert_chain = vec![rustls::Certificate(cert_der.clone())];

        let mut server_config = ServerConfig::with_single_cert(cert_chain, priv_key).unwrap();

        let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
        transport_config.max_concurrent_uni_streams(0_u8.into());
        #[cfg(any(windows, os = "linux"))]
        transport_config.mtu_discovery_config(Some(quinn::MtuDiscoveryConfig::default()));

        Ok((server_config, cert_der))
    }
}
