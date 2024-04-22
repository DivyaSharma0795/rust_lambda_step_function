use anyhow::{Error, anyhow};
use serde_derive::{Deserialize, Serialize};
use futures::future::{BoxFuture, FutureExt};
use lambda_runtime::{handler_fn, Context, run};

#[derive(Deserialize, Serialize)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Deserialize, Serialize)]
struct CustomOutput {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> { // Updated return type
    let handler = handler_fn(my_handler);
    run(handler).await?;
    Ok(())
}

fn my_handler(e: CustomEvent, _c: Context) -> BoxFuture<'static, Result<CustomOutput, Error>> {    
    async move {
        if e.first_name == "" {
            return Err(anyhow!("Missing first name"));
        }

        Ok(CustomOutput {
            message: format!("Hello, {}!", e.first_name),
        })
    }.boxed()
}