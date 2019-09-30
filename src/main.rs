use futures::future::Future;
use futures::stream::Stream;
use hyper::client::HttpConnector;
use hyper::Client;

fn main() {
    env_logger::init();

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let url =
        ("https://taskcluster-artifacts.net/Ii0zYIfITc6MrVWWiUw0Aw/0/public/build/target.tar.bz2")
            .parse()
            .unwrap();
    let mut connector = HttpConnector::new(4);
    connector.enforce_http(false);

    let mut tls = ::rustls::ClientConfig::new();
    tls.set_protocols(&["h2".into(), "http/1.1".into()]);
    tls.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

    let https: hyper_rustls::HttpsConnector<HttpConnector> = (connector, tls).into();

    let client: Client<_, hyper::Body> = Client::builder().build(https);

    let fut = client
        .get(url)
        .and_then(|response| response.into_body().for_each(|_| Ok(())));
    rt.block_on(fut).unwrap();
}
