use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Deserialize, Serialize)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: CustomEvent, _c: Context) -> Result<CustomOutput, HandlerError> {
    if e.first_name == "" {
        return Err(HandlerError::from("Missing first name"));
    }

    Ok(CustomOutput {
        message: format!("Hello, {}!", e.first_name),
    })
}