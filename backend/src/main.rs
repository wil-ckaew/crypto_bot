//main.rs
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::process::Command;
use log::info;
use actix_cors::Cors; // Adicionado para corrigir o erro
use serde_json::json; // Importa a macro `json`

mod bot;  // Agora o módulo bot está no arquivo src/bot.rs

// Função que executa a negociação no servidor web
pub async fn trade() -> impl Responder {
    let output = Command::new("python3")
        .arg("../model/predict.py")
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let prediction_output = String::from_utf8_lossy(&output.stdout).trim().to_uppercase();

            match prediction_output.as_str() {
                "BTC" | "USDT" => {
                    match bot::make_trade(prediction_output.clone()).await {
                        Ok(message) => HttpResponse::Ok().json(json!({
                            "status": "success",
                            "message": message,
                            "prediction": prediction_output
                        })),
                        Err(e) => HttpResponse::InternalServerError().json(json!({
                            "status": "error",
                            "message": format!("Erro ao negociar: {}", e),
                            "prediction": prediction_output
                        })),
                    }
                }
                msg if msg.contains("Saldo insuficiente") => HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": msg
                })),
                msg if msg.starts_with("Erro") => HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": msg
                })),
                _ => HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": format!("Saída inesperada do script: {}", prediction_output)
                })),
            }
        }
        Ok(output) => {
            let error_message = String::from_utf8_lossy(&output.stderr);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Erro no script Python: {}", error_message)
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Erro ao executar o script Python: {}", e)
        })),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("Iniciando o servidor do bot...");

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(), // Permite requisições de qualquer origem
            )
            .route("/trade", web::get().to(trade)) // Define a rota para /trade
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
