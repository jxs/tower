use futures::Future;
use futures::future::lazy;
use tokio;
use tower::{Service, ServiceBuilder};

pub fn main() {
    tokio::run(lazy(|| {
        let my_service = tower::service_fn(|_request| Ok::<_, &'static str>("omg what"));

        let mut service = ServiceBuilder::new()
            .map_request(|request: &'static str| request.to_string())
            .concurrency_limit(100)
            .buffer(5)
            .service(my_service);

        assert!(service.poll_ready().is_ok());

        service.call("hello?")
            .map(|response| {
                println!("resp = {:?}", response);
            })
            .map_err(|_| ())
    }));
}
