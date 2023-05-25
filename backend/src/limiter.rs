use crate::error::UserError;
use crate::models::Ip;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use chrono::DateTime;
use chrono::{Duration, Utc};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::net::{self, SocketAddr};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

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
}

impl FixedWindowRateLimiter {
    pub fn new(max_requests: u32, duration: Duration) -> Self {
        FixedWindowRateLimiter {
            max_requests: 100,
            duration: Duration::seconds(10),
        }
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
        let user_ip = get_ip(&request);
        let db = get_db(&request);

        //Check if the amount of requests is allowed

        /*Box::pin(async move {
            if rate_limiter.is_allowed(user_ip, &mut db).await {
                rate_limiter.add_request(user_ip, &mut db).await;
                let future = self.service.call(request);
                let result = future.await?;
                Ok(result)
            } else {
                Err(UserError::TooManyRequests)
            }
        })*/

        /*
        //Start time reset
        self.start = Utc::now();
        //Clear total_request for next window
        let query_result = db
            .query("DELETE FROM ip WHERE window_start = $window_start")
            .bind(("window_start", self.start.to_rfc3339()))
            .await;
         */
        /*
        //Add new request to database
        let query_result = db.query(
            "UPDATE ip SET requests = $requests WHERE user_ip = $user_ip")
            .bind(("requests", total_request))
            .bind(("user_ip", user_ip))
            .await;
         */
        let fut = self.service.call(request);
        Box::pin(async move {
            let query_result = db
                .query("SELECT * FROM ip WHERE user_ip = $user_ip LIMIT 1")
                .bind(("user_ip", user_ip))
                .await;
            let is_allowed = match query_result {
                Ok(mut response) => {
                    let ip_result: surrealdb::Result<Option<Ip>> = response.take(0);
                    match ip_result {
                        Ok(optional_ip) => match optional_ip {
                            Some(ip) => {
                                while ip.requests< self.rate_limiter.max_requests{
                                    let add_req = db
                                .query("UPDATE ip SET requests = $requests WHERE user_ip = $user_ip")
                                .bind(("requests",  ip.requests + 1))
                                .bind(("user_ip", user_ip))
                                .await;
                                true;
                                break;
                              } 
                            }
                            None => {
                            let add_ip = db
                            .query("INSERT INTO ip (user_ip, requests) VALUES ($user_ip, 1)")
                            .bind(("user_ip", optional_ip))
                            .await;
                            false;
                        },
                        Err(err) => {
                            println!("error outer: {:#?}", err);
                            false
                        }
                    }
                }
                Err(err) => {
                    println!("error outer: {:#?}", err);
                    false
                }
            };
            if is_allowed {
                let result = fut.await?;
                Ok(result)
            } else {
                Err(UserError::TooManyRequests)
            }
        })
    }
}

fn get_db(req: &ServiceRequest) -> Surreal<Client> {
    req.app_data::<Surreal<Client>>().unwrap().clone()
}
fn get_ip(req: &ServiceRequest) -> SocketAddr {
    req.peer_addr().unwrap()
}

