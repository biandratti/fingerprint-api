use warp::reject::Reject;
use warp::Rejection;

use crate::domain::error::ApiError;

pub struct ErrorResponseHandling {}

impl ErrorResponseHandling {
    pub fn map_io_error(e: ApiError) -> Rejection {
        match e.get_error_code() {
            400 => warp::reject::custom(CustomRejection(e.error)),
            _ => warp::reject::custom(CustomRejection(e.error)),
        }
    }
}

#[derive(Debug)]
pub struct CustomRejection(pub Box<dyn std::error::Error + Send + Sync>);

impl Reject for CustomRejection {}
