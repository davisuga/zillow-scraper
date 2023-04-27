use core::fmt;

use crate::zillow_scrapper::{get_profile_data, get_profile_data_with_redis_cache};
use futures::TryFutureExt;
use lambda_http::{Body, Error, IntoResponse, Request, RequestExt, Response};
use serde::Deserialize;
use tracing::info;

pub mod zillow_profile;
pub mod zillow_scrapper;
#[derive(Deserialize)]
struct AgentInfo {
    username: String,
}

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

async fn fetch_agent_data(event: Request) -> Result<impl IntoResponse, Error> {
    let body = event.body().to_owned();
    info!("body: {:?}", &body);
    match &body {
        Body::Empty | Body::Binary(_) => {
            Err(Box::new(CustomError("Failed to parse input: {}".to_owned())) as Error)
        }
        Body::Text(body) => {
            let agent_info = serde_json::from_str::<AgentInfo>(&body)?;

            match get_profile_data_with_redis_cache(&agent_info.username).await {
                Err(_) => Ok(Response::builder()
                    .status(500)
                    .body::<String>("Failed to fetch profile".into())
                    .expect("failed to build response")),

                Ok(profile) => {
                    info!("profile: {:?}", &profile);
                    let response_body =
                        serde_json::to_string(&profile).expect("Failed to serialize profile");

                    let resp = Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(response_body.into())
                        .expect("Failed to build response");

                    Ok(resp)
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();
    info!("starting lambda");
    lambda_http::run(lambda_http::service_fn(fetch_agent_data)).await?;

    Ok(())
}
