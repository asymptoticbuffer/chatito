use std::io;
use futures::{future, Future};
use tokio_service::{Service};

pub struct Echo;

impl Service for Echo{
    type Request = String;
    type Response = String;

    type Error = io::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        // TODO implement encrypting logic
        Box::new(future::ok(req))
    }
}
