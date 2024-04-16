use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    http::header::{HeaderValue, AUTHORIZATION},
    error::ErrorUnauthorized,
};
use futures_util::future::LocalBoxFuture;

use berry_lib::auth::jwt::{JwtConfig, JwtAlgorithm};

use colored::*;

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
        let auth_result = check_authentication(req.headers().get(AUTHORIZATION), &req);

        let fut = self.service.call(req);

        Box::pin(async move {
            match auth_result {
                Some(true) => {
                    println!("{} {}", "Auth Middleware".green(), "Authentication Passed".bright_green().bold()); // !REMOVE
                    fut.await
                },
                _ => {
                    println!("{} {}", "Auth Middleware".green(), "Authentication Failed".bright_red().bold()); // !REMOVE
                    Err(ErrorUnauthorized("Unauthorized"))
                },
            }
        })
    }
}

fn check_authentication(header_value: Option<&HeaderValue>, req: &ServiceRequest) -> Option<bool> {
    println!("{} {}", "Auth Middleware".green(), "Checking Authentication FN".cyan().bold()); // !REMOVE


    println!("{}, {}", "Auth Middleware".green(), "Checking Whitelisted Routes".cyan().bold()); // !REMOVE

    println!("{}, {:?}", "Request Path".cyan(), req.path()); // !REMOVE

    let whitelisted_routes = vec!["/auth/login", "/auth/register"];

    if whitelisted_routes.contains(&req.path()) {
        println!("{} {}", "Whitelisted Route".green(), "Skipping Authentication".cyan().bold()); // !REMOVE
        return Some(true);
    }

    let header_str = match header_value {
        Some(value) => match value.to_str() {
            Ok(str) => str,
            Err(err) => {
                println!("{} {}", "Header Value Error".red(), err);
                return Some(false);
            }
        },
        None => {
            println!("{}", "Authorization Header Missing".yellow());
            return Some(false);
        }
    };


    let jwt_algorithm = JwtAlgorithm::HS256; // Specify the desired algorithm here

    let jwt_config = JwtConfig::new(jwt_algorithm.into());

    let validation_result = jwt_config.validate_token(header_str);

    println!("{}, {:?}", "Validation Result".green(), validation_result); // !REMOVE

    match validation_result {
        Ok(_) => Some(true),
        Err(err) => {
            println!("{} {:?}", "JWT Error".red(), err); // !REMOVE
            Some(false)
        }
    }
}