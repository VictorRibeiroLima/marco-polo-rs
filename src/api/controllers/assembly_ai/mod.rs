use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub status: String,
    #[serde(rename = "transcript_id")]
    pub transcript_id: String,
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let scope = web::scope("/assemblyai").service(webhook);
    config.service(scope);
}

#[actix_web::post("/transcriptions/webhook")]
async fn webhook(req_body: web::Json<Root>) -> impl actix_web::Responder {
    let req_body = req_body.into_inner();
    let url = format!(
        "https://api.assemblyai.com/v2/transcript/{}/srt",
        req_body.transcript_id
    );

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("Authorization", "786606773f51460cb284d35f05395acd")
        .send()
        .await
        .unwrap();

    let body = resp.text().await.unwrap();
    println!("{}", body);

    return HttpResponse::Ok();
}
