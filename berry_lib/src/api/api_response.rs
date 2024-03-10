use actix_web::{body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T>
where
    T: Serialize + Debug,
{
    pub data: Option<T>,
    pub error: Option<String>,
    #[serde(skip)]
    pub status_code: Option<StatusCode>,
}

impl<T> ApiResponse<T>
where
    T: Serialize + Debug,
{
    pub fn new(
        data: Option<T>,
        error: Option<String>,
        status_code: Option<StatusCode>,
    ) -> ApiResponse<T> {
        ApiResponse {
            data,
            error,
            status_code,
        }
    }
}

impl<T> Responder for ApiResponse<T>
where
    T: Serialize + Debug,
{
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse {
        let status_code = self.status_code.unwrap_or_else(|| {
            if self.error.is_some() {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::OK
            }
        });

        println!("Direct Status Code Determination: {:?}", status_code); // Diagnostic log

        let body = match to_string(&self) {
            Ok(body) => body,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };

        HttpResponse::build(status_code)
            .content_type("application/json")
            .body(body)
    }
}
