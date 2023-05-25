use std::future::{ready, Ready};
use std::net;
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use chrono::{Duration, Utc};
use chrono::DateTime;
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
    db: Surreal<Client>,
    start: DateTime<Utc>,
}

impl FixedWindowRateLimiter {
    pub fn new(max_requests: u32, duration: Duration, db: Surreal<Client>) -> Self {
        FixedWindowRateLimiter {
            max_requests: 100,
            duration: Duration { secs: 10, nanos: 0 },
            db,
            start: Utc::now(),
        }
    }

    pub async fn is_allowed(&mut self, user_ip: Option<net::SocketAddr>) -> bool{
        let now = Utc::now();
        let elapsed = now.signed_duration_since(self.start);
        if elapsed > self.duration{
            self.window_reset().await;
        }
    
        //Check if the amount of requests is allowed
        let total_request = self.db.query(
            "SELECT COUNT(*) FROM ip WHERE user_ip = $user_ip AND window_start = $window_start;")
           .bind(("user_ip", user_ip))
           .bind(("window_start", self.start.to_rfc3339())).await;
    
        if let Ok(rows) = total_request {
            if let Some(row) = rows.take(0){
                if let Some(count) = row.take(0){
                    let requests: u32 = count;
                    return requests <= self.max_requests;
                }
            }
        }
        false
    }

    async fn window_reset(&mut self){
        //Start time reset
        self.start = Utc::now();
        //Clear total_request for next window
        let query_result = self.db.query(
            "DELETE FROM ip WHERE window_start = $window_start")
            .bind(("window_start", self.start.to_rfc3339())).await;
    }

    async fn add_request(&mut self, user_ip: Option<net::SocketAddr>){
        //Add new request to database
      let query_result = self.db.query(
            "UPDATE ip (user_ip, window_start) SET ($user_ip, $window_start)")
            .bind(("user_ip", user_ip))
            .bind(("window_start", self.start.to_rfc3339()))
            .await;
        
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
            rate_limiter: FixedWindowRateLimiter::new(self.max_requests, self.duration, &self.db),
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
        let mut rate_limiter = self.rate_limiter;

        Box::pin(async move{
            if rate_limiter.is_allowed(user_ip).await{
                rate_limiter.add_request(user_ip).await;
                let future = self.service.call(request);
                let result = future.await?;
                Ok(result)
            }else{
                Err(UserError::TooManyRequests)
            }
        })
    }
}
