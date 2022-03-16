use actix_web::{web, App, HttpServer, HttpResponse, Error, error, dev, http};
use tera::Tera;
use actix_web::middleware::{errhandlers::ErrorHandlers, errhandlers::ErrorHandlerResponse};
use actix_files as fs;

// async fn index() -> Result<HttpResponse, Error> {
//     Ok(HttpResponse::Ok().content_type("text/plain").body("Hello World"))
// }

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new( || {
//         App::new()
//             .service(web::resource("/").to(index))
//     })
//         .bind("localhost:3000")
//         .expect("Can not bind to port 3000")
//         .run()
//         .await
// }

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    // hanada という名前を変数としてテンプレートに渡す
    ctx.insert("name", "hanada");
    let view =
        tmpl.render("index.html.tera", &ctx)
            .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

fn not_found<B>(res: dev::ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let new_resp = fs::NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        // error_handlers に, not_found 関数を 404 のときの handler として登録
        let error_handlers = ErrorHandlers::new()
            .handler(http::StatusCode::NOT_FOUND, not_found);

        // templates ディレクトリを指定して、Teraを初期化
        let templates = Tera::new("templates/**/*").unwrap();

        App::new()
            .data(templates) // handlerから参照できるように保持
            .wrap(error_handlers)
            .service(web::resource("/").to(index))
    })
        .bind("localhost:3000")
        .expect("Can not bind to port 3000")
        .run()
        .await
}