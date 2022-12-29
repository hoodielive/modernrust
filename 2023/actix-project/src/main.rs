use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState
{
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String
{
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    HttpServer::new(|| {
        App::new()
            .app_data(
                web::Data::new(
                    AppState {
                        app_name: String::from("Oyeku Meji is in the building!"),
                    }
                )
            )
            .service(index)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/index.html", web::get().to(index3))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}