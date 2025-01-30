use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;

/// フォームデータ用の構造体
#[derive(Deserialize)]
struct GcdForm {
    n: u64,
    m: u64,
}

/// GCD 計算関数
fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        let temp = m;
        m = n % m;
        n = temp;
    }
    n
}

/// ルート (/) のハンドラー
async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
            </form>
            "#,
        )
}

/// GCD 計算のハンドラー
async fn compute_gcd(form: web::Form<GcdForm>) -> impl Responder {
    let result = gcd(form.n, form.m);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!(
            "<h1>The greatest common divisor of {} and {} is {}</h1>",
            form.n, form.m, result
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(compute_gcd))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
