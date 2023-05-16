use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

use futures_util::future::LocalBoxFuture;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Private,
    Public,
    Expired,
}
pub struct Request {
    state: State
}

impl Request {
    pub fn new(state: State) -> Self {
        Request {
            state
        }
    }
} 

impl<S, B> Transform<S, ServiceRequest> for Request
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service, state: self.state.clone() }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
    state: State,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        println!("{} ", request.path());
        dbg!(self.state);

        let future = self.service.call(request);

        Box::pin(async move {
            let result = future.await?;
            Ok(result)
        })
    }
}
