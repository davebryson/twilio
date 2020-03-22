use actix_web::{App, HttpServer};

use twilio::index;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = 3000;

    println!(" ~~ server running on port: {:} ~~", port);
    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
