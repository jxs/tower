use futures::Future;
use tower::{Service, ServiceBuilder};

pub fn main() {
    let my_service = tower::service_fn(|request: &'static str| Ok::<_, &'static str>("omg what"));

    let mut service = ServiceBuilder::new()
        .concurrency_limit(100)
        .buffer(5)
        .service(my_service);

    assert!(service.poll_ready().is_ok());

    let response = service.call("hello?").wait().unwrap();
    println!("resp = {:?}", response);
}
