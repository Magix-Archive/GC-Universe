use std::fs;
use std::net::SocketAddr;
use rcgen::*;
use std::path::PathBuf;
use std::str::FromStr;
use hudsucker::certificate_authority::RcgenAuthority;
use rustls_pemfile as pemfile;
use hudsucker::{HttpContext, HttpHandler, ProxyBuilder, RequestOrResponse, rustls};
use hudsucker::async_trait::async_trait;
use hudsucker::hyper::{Body, Request, Response, StatusCode, Uri};
use crate::options::Options;
use crate::structs::State;
use crate::system::run_command;

#[derive(Clone)]
struct TrafficHandler;

/// Checks if the request contains a URL that should be intercepted.
/// request: The HTTP request.
fn contains(request: &Request<Body>) -> bool {
    let intercept = State::options().proxy.urls;
    let uri = request.uri().to_string();

    // Check if any of the intercept URIs match the request URI.
    for url in intercept {
        if uri.contains(url.as_str()) {
            return true;
        }
    }

    false
}

#[async_trait]
impl HttpHandler for TrafficHandler {
    async fn handle_request(
        &mut self, _: &HttpContext,
        mut request: Request<Body>
    ) -> RequestOrResponse {
        // Check if we should intercept the request.
        if !contains(&request) {
            return request.into();
        }

        let game = State::game();
        let server = game.proxy.address();

        if request.method().as_str() == "CONNECT" {
            let builder = Response::builder()
                .header("DecryptEndpoint", "Created")
                .status(StatusCode::OK);
            let res = builder.body(()).unwrap();

            *res.body()
        } else {
            let uri_path_and_query = request.uri().path_and_query().unwrap().as_str();
            // Create new URI.
            let new_uri = Uri::from_str(
                format!("{}{}", server, uri_path_and_query).as_str()
            ).unwrap();
            // Set request URI to the new one.
            *request.uri_mut() = new_uri;
        }

        request.into()
    }

    async fn should_intercept(
        &mut self, _: &HttpContext,
        request: &Request<Body>
    ) -> bool {
        contains(request)
    }
}

/// Basic Ctrl + C signal handler.
async fn shutdown_handler() {
    tokio::signal::ctrl_c()
        .await.expect("Failed to install CTRL+C signal handler")
}

/// Spawns a new proxy server.
/// options: The configuration options.
pub fn create_proxy(options: Options) {
    // Check if the certificate needs to be created.
    let cert_path = PathBuf::from(options.proxy.cert_path);
    if !cert_path.exists() {
        create_certificate(&cert_path);
    }

    // Load the certificate and private key.
    let mut cert: &[u8] = &fs::read(cert_path.join("cert.pem"))
        .expect("Failed to read certificate.");
    let mut key: &[u8] = &fs::read(cert_path.join("key.pem"))
        .expect("Failed to read private key.");

    // Parse the certificate and private key.
    let cert = rustls::Certificate(
        pemfile::certs(&mut cert)
            .expect("Failed to parse certificate.")
            .remove(0)
    );
    let key = rustls::PrivateKey(
        pemfile::pkcs8_private_keys(&mut key)
            .expect("Failed to parse private key.")
            .remove(0)
    );

    // Create the certificate authority.
    let authority = RcgenAuthority::new(key, cert, 1000)
        .expect("Failed to create certificate authority.");

    // Create the proxy server.
    let address = format!("{}:{}", options.proxy.host, options.proxy.port);
    let proxy = ProxyBuilder::new()
        .with_addr(SocketAddr::from_str(address.as_str())
            .expect("Failed to parse bind address."))
        .with_rustls_client()
        .with_ca(authority)
        .with_http_handler(TrafficHandler)
        .build();

    // Start the proxy server.
    tokio::spawn(proxy.start(shutdown_handler()));
}

/// Creates the certificate files for the proxy server.
fn create_certificate(path: &PathBuf) {
    // Write certificate details.
    let mut details = DistinguishedName::new();
    details.push(DnType::CommonName, "Cultivation");
    details.push(DnType::OrganizationName, "Grasscutter");
    details.push(DnType::CountryName, "CN");
    details.push(DnType::LocalityName, "CN");

    // Write certificate properties.
    let mut params = CertificateParams::default();
    params.distinguished_name = details;
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.key_usages = vec![
        KeyUsagePurpose::DigitalSignature,
        KeyUsagePurpose::KeyCertSign,
        KeyUsagePurpose::CrlSign,
    ];

    // Create the certificate.
    let certificate = Certificate::from_params(params)
        .expect("Failed to create certificate.");
    let cert_pem = certificate.serialize_pem()
        .expect("Failed to serialize certificate.");
    let key_pem = certificate.serialize_private_key_pem();

    // Write the certificate to disk.
    if !path.exists() {
        fs::create_dir_all(path)
            .expect("Failed to create certificate directory.");
    }

    let cert_path = path.join("cert.pem");
    fs::write(&cert_path, cert_pem)
        .expect("Failed to write certificate to disk.");
    fs::write(path.join("key.pem"), key_pem)
        .expect("Failed to write key to disk.");
    println!("Created certificate and private key.");

    // Install the certificate.
    install_certificate(&cert_path);
}

#[cfg(target_os = "windows")]
fn install_certificate(path: &PathBuf) {
    run_command(
        "certutil",
        vec!["-user", "-addstore", "Root", path.to_str().unwrap()],
        None,
    );
    println!("Installed certificate.");
}

#[cfg(target_os = "macos")]
pub fn install_ca_files(path: &PathBuf) {
    run_command(
        "security",
        vec![
            "add-trusted-cert",
            "-d",
            "-r",
            "trustRoot",
            "-k",
            "/Library/Keychains/System.keychain",
            path.to_str().unwrap(),
        ],
        None,
    );
    println!("Installed certificate.");
}
