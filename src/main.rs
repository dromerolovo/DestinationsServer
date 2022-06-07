use actix_web::{web, App, HttpResponse, HttpServer, Responder, Error};
mod geo_data;
mod db;
use serde::{Serialize, Deserialize};

#[allow(unused_imports)]
use geo_data::{Destination, in_boundaries_result,transform_vec_to_state};

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Serialize)]
struct AppResponse {
    name : String,
    obj_path : String
}


async fn handle (req_body : String) -> impl Responder {

    let items = db::db_main().await.unwrap();
    let state = transform_vec_to_state(items);
    let destination_tuple = in_boundaries_result(&state, &req_body);
    let destination = AppResponse {
        name : destination_tuple.0,
        obj_path : destination_tuple.1
    };

    web::Json(destination)
    

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/destinations", web::post().to(handle))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

