use anyhow::Error;
use serde_derive::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use futures::future::{BoxFuture, FutureExt};
use lambda_runtime::{HandlerFn, Context, run};


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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = HandlerFn::new(my_handler);
    run(handler).await?;
    Ok(())
}

fn my_handler(e: CustomEvent, _c: Context) -> BoxFuture<'static, Result<CustomOutput, Error>> {    
    async move {
        if e.first_name == "" {
            return Err(anyhow::anyhow!("Missing first name"));
        }

        Ok(CustomOutput {
            message: format!("Hello, {}!", e.first_name),
        })
    }.boxed()
}