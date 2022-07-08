// #[allow(unused_imports)]
use super::*;

use futures::{executor::block_on, prelude::*};
// use http_service::{HttpService, Request, Response};
use http_service::{HttpService, Response};

use futures::future::BoxFuture;
use http_service_mock::make_server;
use http_types::headers::{HeaderName, HeaderValue};
use http_types::{Method, Request, Url};
use std::str::FromStr;
use tide::{Middleware, Next};

#[derive(Debug)]
pub struct TestBackend<T: HttpService> {
    service: T,
    connection: T::Connection,
}

impl<T: HttpService> TestBackend<T> {
    fn wrap(service: T) -> Result<Self, <T::ConnectionFuture as TryFuture>::Error> {
        let connection = block_on(service.connect().into_future())?;
        Ok(Self {
            service,
            connection,
        })
    }

    /// Send a request to a simulated server.
    pub fn simulate(
        &mut self,
        req: Request,
    ) -> Result<Response, <T::ResponseFuture as TryFuture>::Error> {
        block_on(
            self.service
                .respond(self.connection.clone(), req)
                .into_future(),
        )
    }
}

pub fn make_server<T: HttpService>(
    service: T,
) -> Result<TestBackend<T>, <T::ConnectionFuture as TryFuture>::Error> {
    TestBackend::wrap(service)
}

#[async_std::test]
async fn nested() {
    let mut inner = tide::new();
    inner.at("/foo").get(|_| async { Ok("foo") });
    inner.at("/bar").get(|_| async { Ok("bar") });

    let mut outer = tide::new();
    // Nest the inner app on /foo
    outer.at("/foo").nest(inner);

    let mut server = make_server(my_server().await).unwrap();

    let req = Request::new(
        Method::Get,
        Url::parse("http://example.com/foo/foo").unwrap(),
    );
    let res = server.simulate(req).unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.body_string().await.unwrap(), "foo");

    // let req = Request::new(
    //     Method::Get,
    //     Url::parse("http://example.com/foo/bar").unwrap(),
    // );
    // let res = server.simulate(req).unwrap();
    // assert_eq!(res.status(), 200);
    // assert_eq!(res.body_string().await.unwrap(), "bar");
}
