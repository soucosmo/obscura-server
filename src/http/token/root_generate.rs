use crate::dao::{AppState, Token, Path};
use chrono::Utc;
use uuid::Uuid;
use actix_web::{
    post, web::Data, HttpResponse, Responder
};


#[post("/token/root-generate")]
pub async fn root_generate(app_state: Data<AppState>) -> impl Responder {
    let uuid_token = Uuid::new_v4();

    let root_exists = app_state.partitions.tokens.is_empty();

    if let Err(e) = root_exists {
        return HttpResponse::InternalServerError().body(
            e.to_string()
        );
    }

    if !root_exists.unwrap() {
        return HttpResponse::Conflict()
            .body("the root token has already been created previously");
    }

    let paths = vec![
        Path {
            prefix: "/".to_string(),
            write: true,
        }
    ];

    let btoken = Token {
        description: "root".to_string(),
        is_root: true,
        paths: paths,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    let res = app_state.partitions.tokens.insert(
        "root",
        serde_json::to_vec(&btoken).unwrap(),
    );

    match res {
        Ok(_) => HttpResponse::Ok().body(uuid_token.to_string()),
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
