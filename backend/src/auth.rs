use std::future::{ready,Ready};

use actix_web::{dev::{ServiceResponse, ServiceRequest, Transform, Service, forward_ready}, http::Error, rt::net::Ready};

use futures_util::future::LocalBoxFuture;


pub struct Request;

impl <S,B> Transform <Service, ServiceRequest> for Request
where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S:: Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = Middleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self:: InitError>>;

    fn new_transform(&self, service: S) -> Result<Self:: Future, _>{
        Ok(ready(Ok(Middelware{service})))
    }
}


pub struct Middelware<S>{
    service: S,
}

impl <S,B> Service<ServiceRequest> for Middelware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S:: Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future{
        println!("{} ", request.path());

        let future = self.service.call(request);

        Box::pin(async move{
            let result = future.await?;

            println!("");
            Ok(result)
        })
    }
}