
#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(FromForm)]
struct ProgramForm {
    program_id: String,
}

#[post("/analyze", data = "<form_data>")]
async fn analyze(form_data: Form<ProgramForm>) -> Template {
    let program_id = &form_data.program_id;
    let client = reqwest::Client::new();
    let rpc_url = "https://api.mainnet-beta.solana.com";

    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAccountInfo",
        "params": [program_id, { "encoding": "base64" }]
    });

    let mut context = HashMap::new();

    match client.post(rpc_url)
        .json(&body)
        .send()
        .await {
        Ok(resp) => {
            let json: serde_json::Value = resp.json().await.unwrap_or_default();
            let result = json.get("result").and_then(|r| r.get("value")).and_then(|v| v.get("data")).and_then(|d| d.get(0));

            if let Some(data_base64) = result {
                let decoded = base64::decode(data_base64.as_str().unwrap()).unwrap_or_default();
                context.insert("program_id", program_id.to_string());
                context.insert("decoded_hex", hex::encode(&decoded));
                Template::render("result", &context)
            } else {
                context.insert("error", "Program data not found".to_string());
                Template::render("error", &context)
            }
        }
        Err(_) => {
            context.insert("error", "RPC request failed".to_string());
            Template::render("error", &context)
        }
    }
}

#[get("/")]
fn index() -> Template {
    Template::render("form", &HashMap::<String, String>::new())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, analyze])
        .attach(Template::fairing())
}
