//bot.rs
use reqwest::Client;
use serde::Deserialize;

// Definir a estrutura para erro da Binance
#[derive(Deserialize, Default)]
struct BinanceError {
    code: i32,
    msg: String,
}

// Função que faz a negociação com a Binance
pub async fn make_trade(prediction: String) -> Result<String, String> {
    // Limpar espaços em branco e garantir que o valor seja tratado corretamente
    let prediction = prediction.trim().to_uppercase(); // Garantir que o valor esteja em maiúsculas

    // Exemplo de mapeamento
    let symbol = match prediction.as_str() {
        "BTC" => "BTCUSDT", // Exemplo de mapeamento, você pode adicionar mais lógica aqui
        "ETH" => "ETHUSDT",
        _ => return Err(format!("Simbolo invalido ou nao reconhecido: {}", prediction)),
    };

    // Cria um cliente HTTP para enviar a requisição
    let client = Client::new();
    let response = client
        .post("https://api.binance.com/api/v3/order")
        .query(&[
            ("symbol", symbol),
            ("side", "BUY"),      // Exemplo de operação de compra
            ("type", "MARKET"),   // Tipo de ordem
            ("quantity", "0.01"), // Exemplo de quantidade
        ])
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                Ok("Ordem executada com sucesso".to_string())
            } else {
                // Caso a API retorne um erro, captura a resposta de erro
                let error: BinanceError = resp.json().await.unwrap_or_default();
                Err(format!("Erro ao executar ordem: {} - Detalhes: {}", error.code, error.msg))
            }
        }
        Err(e) => Err(format!("Erro ao fazer requisicao: {}", e)),
    }
}
