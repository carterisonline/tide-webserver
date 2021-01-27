use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

pub fn build(workdir: &str) -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("Couldn\'t build the SSL acceptor");

    builder
        .set_private_key_file(format!("{}keys/keyfile.key", workdir), SslFiletype::PEM)
        .expect("Couldn\'t read OpenSSL keyfile");

    builder
        .set_certificate_chain_file(format!("{}keys/chain.pem", workdir))
        .expect("Couldn\'t read OpenSSL certificate chain file");

    builder
}
