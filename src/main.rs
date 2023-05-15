use axum::{
    extract::{Path, Query},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
mod database;

use database::connection::DB;
use sqlx::types::Uuid;

struct ChampsBd{
    uuid: Uuid,
    champion: String,
    descripcion: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Champion {
    id: String,
    campeon: String,
    descripcion: String,
}

#[derive(Serialize)]

struct AllChamps{
status: String,
champs: Vec<Champion>,
descripcion: String,
}

#[tokio::main]
async fn main(){
    let app = Router::new()
    .route("/", get(campeones));



    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}


async fn campeones() -> Json<AllChamps>{

    let pool = DB::connection().await;
    let campeones = sqlx::query_as!( ChampsBd ,
        r#"SELECT * FROM champ "#)
        .fetch_all(&pool)
        .await;


    let response = match campeones  {

        Ok(res) => AllChamps { 
            status: "200 OK".to_string(), 
            champs: res 
            .into_iter()
            .map(|x| Champion{
                id: x.uuid.to_string(),
                campeon: x.champion,
                descripcion: option_to_string(x.descripcion)
            }) 
            .collect(),
            descripcion: "descripcion obtenida".to_string()
        },
        Err(_err) => AllChamps{
            status: "500 Internal Server Error".to_string(),
            champs: vec![],
            descripcion: "error al obtener registros".to_string(),
        }
        
    };


Json(response)


}

fn option_to_string(descripcion: Option<String>) -> String {
    let resultado = match descripcion {
        Some(desc) => desc,
        None => "".to_string(),
    };

    resultado
}