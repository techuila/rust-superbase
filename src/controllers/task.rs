use actix_web::{get, web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    task: String,
}

// Fetch Tasks
#[get("/tasks")]
pub async fn list() -> Result<impl Responder> {
    let obj = Person {
        task: "do nothing".to_string(),
    };
    Ok(web::Json(obj))
}
