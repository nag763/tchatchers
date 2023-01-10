use axum::{Middleware, Service, http::{Request, Response}};
use futures_util::future::BoxFuture;

pub struct MyMiddleware {
    next: Box<dyn Service<Request, Response>>,
}

impl<S> Middleware<S> for MyMiddleware
where
    S: Service<Request, Response> + 'static,
{
    type Service = Self;

    fn wrap(&self, service: S) -> Self::Service {
        MyMiddleware {
            next: Box::new(service),
        }
    }
}

impl Service for MyMiddleware {
    type Request = Request;
    type Response = Response;
    type Future = BoxFuture<Result<Self::Response, ()>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), ()>> {
        // You can check if the inner service is ready to be polled
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let fut = self.next.call(request);
        let fut = fut.then(|result| async move {
            // You can add your middleware functionality here, such as modifying the request or response, logging, or error handling
            result
        });
        Box::pin(fut)
    }
}