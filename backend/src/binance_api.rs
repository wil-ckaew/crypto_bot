//binancle_api.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use log::info;

#[derive(Deserialize, Debug)]
pub struct BinanceBalance {
    #[serde(rename = "free")]
    pub free: String,
    #[serde(rename = "locked")]
    pub locked: String,
}

#[derive(Serialize, Debug)]
pub struct OrderData {
    symbol: String,
    side: String,
    #[serde(rename = "type")]
    order_type: String,
    timeInForce: String,
    quantity: f64,
    price: f64,
}

pub async fn place_order(api_key: &str, symbol: &str, side: &str, quantity: f64, price: f64) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    // Verificando se o parâmetro 'symbol' não está vazio ou malformado
    if symbol.is_empty() {
        return Err("Parametro 'symbol' não pode estar vazio".into());
    }

    // Verificando o formato correto do 'symbol' (ex: "BTCUSDT")
    if symbol.len() < 6 {
        return Err("Parametro 'symbol' esta malformado. O formato esperado é algo como 'BTCUSDT'.".into());
    }

    // Log para verificar o valor de symbol
    info!("Tentando fazer uma ordem para o simbolo: {}", symbol);

    // Criando os dados para a ordem
    let order_data = OrderData {
        symbol: symbol.to_string(),
        side: side.to_string(),
        order_type: "LIMIT".to_string(),
        timeInForce: "GTC".to_string(),
        quantity,
        price,
    };

    // Log para depuração
    info!("Dados da ordem: {:?}", order_data);

    let response = client
        .post("https://api.binance.com/api/v3/order")
        .header("X-MBX-APIKEY", api_key)
        .json(&order_data)
        .send()
        .await?;

    // Clonando o status da resposta para poder usá-lo depois
    let status = response.status().clone();

    // Verificar se a resposta foi bem-sucedida
    if status.is_success() {
        let order_response: serde_json::Value = response.json().await?;
        if let Some(order_id) = order_response.get("orderId") {
            Ok(order_id.to_string())
        } else {
            Err("Erro ao recuperar orderId".into())
        }
    } else {
        // Obter mensagem de erro adicional para depuração
        let error_message = response.text().await?;
        Err(format!("Erro ao executar pedido: {} - Detalhes: {}", status, error_message).into())
    }
}

#[tokio::main]
async fn main() {
    let symbol = "BTCUSDT";  // Exemplo de símbolo válido
    let side = "BUY";         // Exemplo de lado da ordem (pode ser "BUY" ou "SELL")
    let quantity = 0.1;       // Quantidade de criptomoeda a ser comprada ou vendida
    let price = 30000.0;      // Preço limite

    let api_key = "sua_api_key";  // Substitua pela sua chave API

    // Verifique se o 'symbol' não é vazio ou inválido
    if symbol.is_empty() {
        println!("Erro: 'symbol' nao pode ser vazio.");
    } else {
        match place_order(api_key, symbol, side, quantity, price).await {
            Ok(order_id) => println!("Pedido enviado com sucesso. Order ID: {}", order_id),
            Err(e) => eprintln!("Erro ao executar pedido: {}", e),
        }
    }
}
