use std::future::{ready, Ready};
use std::net;

use actix_web::rt::time::Instant;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    
};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::collections::HashMap;
use chrono::Duration;
use futures_util::future::LocalBoxFuture;
use crate::error::UserError;


pub struct RateLimiter {
    max_requests: u32,
    duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, duration: Duration) -> Self {
        RateLimiter {
            max_requests,
            duration,
        }
    }
}

struct FixedWindowRateLimiter {
    max_requests: u32,
    duration: Duration,
    user_requests: HashMap<Option<net::SocketAddr>, u32>,
    start: Instant,
}

impl FixedWindowRateLimiter {
    pub fn new(max_requests: u32, duration: Duration) -> Self {
        FixedWindowRateLimiter {
            max_requests: 100,
            duration: Duration { secs: 10, nanos: 0 },
            user_requests: HashMap::new(),
            start: Instant::now(),
        }
    }

    pub fn is_allowed(&mut self, user_ip: Option<net::SocketAddr>) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.start);
        if elapsed > self.duration.to_std().unwrap() {
            self.user_requests.clear();
            self.start = now;
        }
        let requests = self.user_requests.entry(user_ip).or_insert(0);
        *requests += 1;
        *requests <= self.max_requests
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = UserError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = UserError;
    type Transform = RateLimiterMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware {
            service,
            rate_limiter: FixedWindowRateLimiter::new(self.max_requests, self.duration),
        }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    rate_limiter: FixedWindowRateLimiter,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = UserError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = UserError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        println!("{} ", request.path());
        let user_ip = request.peer_addr();
        let db = request.app_data::<Surreal<Client>>().unwrap();
        let is_allowed = self.rate_limiter.is_allowed(user_ip);
        if is_allowed {
            let future = self.service.call(request);
            Box::pin(async move {
                let result = future.await?;
                Ok(result)
            })
        } else {
            //let response = UserError::TooManyRequests.respond_to(&request)?;
            //let parts = request.into_parts();
            Box::pin(async {
                //Ok(ServiceResponse::new(parts.0, response.into()).map_err(Error::from))
                Err(UserError::TooManyRequests)
            })
        }
    }
}
