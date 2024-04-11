use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    http::header::{HeaderValue, AUTHORIZATION},
    error::ErrorUnauthorized,
};
use futures_util::future::LocalBoxFuture;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Return 200 OK for preflight requests
        println!("HEADERS: {:?}", req.headers()); // !REMOVE
        println!("METHOD: {:?}", req.method()); // !REMOVE

        // Check authentication for other paths
        let auth_result = check_authentication(req.headers().get(AUTHORIZATION), &req);

        println!("AUTH RESULT: {:?}", auth_result); // !REMOVE

        let fut = self.service.call(req);

        Box::pin(async move {
            match auth_result {
                Some(true) => {
                    // If authentication is successful, proceed
                    println!("AUTHENTICATION SUCCESSFUL"); // !REMOVE
                    fut.await
                },
                _ => {
                    println!("AUTHENTICATION FAILED"); // !REMOVE
                    // If authentication fails or no header is present, return 401 Unauthorized
                    Err(ErrorUnauthorized("Unauthorized"))
                }
            }
        })
    }
}



fn check_authentication(header_value: Option<&HeaderValue>, req: &ServiceRequest) -> Option<bool> {
    println!("CHECKING AUTHENTICATION"); // !REMOVE

    let header_str = header_value?.to_str().ok()?;

    let whitelisted_routes = vec!["/auth/login", "/auth/register"];

    println!("REQUEST PATH: {}", req.path()); // !REMOVE

    println!("WHITELISTED CONTAINS: {}", whitelisted_routes.contains(&req.path())); // !REMOVE

    if whitelisted_routes.contains(&req.path()) {
        println!("WHITELISTED ROUTE: {}", req.path()); // !REMOVE
        return Some(true);
    }

    let is_valid = validate_token(header_str);

    println!("IS VALID: {}", is_valid); // !REMOVE

    Some(is_valid)
    
}

fn validate_token(header_str: &str) -> bool {

    println!("VALIDATING TOKEN"); // !REMOVE

    let parts: Vec<&str> = header_str.split_whitespace().collect();

    if parts.len() < 2 {
        println!("TOKEN VALIDATION FAILED: TO SHORT"); // !REMOVE

        return false;
    }


    let bearer = parts[0];
    let token = parts[1];


    if bearer.is_empty() || token.is_empty() {
        println!("TOKEN VALIDATION FAILED: EMPTY BEARER OR TOKEN"); // !REMOVE
        return false;
    }

    let is_valid = check_token(token);

    println!("TOKEN VALIDATION RESULT: {}", is_valid); // !REMOVE

    is_valid
}


fn check_token(token: &str) -> bool {
    println!("CHECKING TOKEN {}", token); // !REMOVE
    true
}
