use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use actix_web::rt::time::Instant;
use chrono::Duration;
use futures_util::future::LocalBoxFuture;


pub struct RateLimiter {
    max_requests: u32,
    duration: Duration,
}

impl RateLimiter{
    pub fn new(max_requests: u32, duration: Duration) -> Self {
        RateLimiter { max_requests, duration, }
    }
}

struct FixedWindowRateLimiter{
    max_requests: u32,
    duration: Duration,
    requests: u32,
    start: Instant,
}

impl FixedWindowRateLimiter{
    pub fn new(max_requests: u32, duration: Duration) -> Self{
        FixedWindowRateLimiter { max_requests: 100, duration: Duration {secs: 10, nanos: 0 }, requests: 0, start: Instant::now(), }
    }

    pub fn is_allowed(&mut self) -> bool{
        let now = Instant::now();
        let elapsed = now.duration_since(self.start);
        if elapsed > self.duration{
            self.requests = 0;
            self.start = now;
        }

        self.requests +=1;
        self.requests <= self.max_requests
    }
}
impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimiterMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware { service,
        rate_limiter: FixedWindowRateLimiter::new(self.max_requests, self.duration), }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    rate_limiter: FixedWindowRateLimiter,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
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
        let is_allowed = self.rate_limiter.is_allowed();
        if is_allowed{
            let future = self.service.call(request);
            Box::pin(async move {
                let result = future.await?;
                Ok(result)
            })
        } else{
            Box::pin(async{
                let response = actix_web::HttpResponse::TooManyRequests().finish();
                Ok(request.into_response(response.into()))
            })
        }
    }
}
