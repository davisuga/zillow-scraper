use crate::zillow_scrapper::get_profile_data;
use futures::TryFutureExt;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use tracing::info;

pub mod zillow_profile;
pub mod zillow_scrapper;
#[derive(Deserialize)]
struct Request {
    body: String,
}

#[derive(Deserialize)]
struct AgentInfo {
    username: String,
}

#[derive(Debug, Serialize)]
struct Response {
    req_id: String,
    body: String,
}

impl std::fmt::Display for Response {
    /// Display the response struct as a JSON string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_as_json = serde_json::json!(self).to_string();
        write!(f, "{err_as_json}")
    }
}

impl std::error::Error for Response {}
async fn deserialize_agent_info(
    req_id: String,
    body: String,
) -> Result<AgentInfo, Box<dyn std::error::Error + Send + Sync>> {
    serde_json::from_str(&body).map_err(|err| {
        info!("Failed to parse input: {:?}", err);
        Box::new(Response {
            req_id,
            body: "Failed to parse input".to_owned(),
        }) as Box<dyn std::error::Error + Send + Sync>
    })
}

async fn fetch_profile_data(username: String, req_id: String) -> Result<Response, Error> {
    get_profile_data(&username)
        .await
        .map(|profile| {
            info!("profile: {:?}", &profile);
            Response {
                req_id: req_id.clone(),
                body: serde_json::to_string(&profile).expect("Failed to serialize profile"),
            }
        })
        .map_err(|err| {
            info!("Failed to parse: {:?}", err);
            Box::new(Response {
                req_id,
                body: "Failed to parse input".to_owned(),
            }) as Box<dyn std::error::Error + Send + Sync>
        })
}

#[tracing::instrument(skip(event), fields(req_id = %event.context.request_id))]
async fn fetch_agent_data(event: LambdaEvent<Request>) -> Result<Response, Error> {
    info!("handling a request");

    deserialize_agent_info(event.context.request_id.clone(), event.payload.body)
        .and_then(|agent_info| fetch_profile_data(agent_info.username, event.context.request_id))
        .await
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    // Initialize the client here to be able to reuse it across
    // different invocations.
    //
    // No extra configuration is needed as long as your Lambda has
    // the necessary permissions attached to its role.
    // let config = aws_config::load_from_env().await;

    lambda_runtime::run(service_fn(|event: LambdaEvent<Request>| async {
        fetch_agent_data(event).await
    }))
    .await
}
