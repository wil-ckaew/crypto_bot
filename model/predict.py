#predict.py
from binance.client import Client
from dotenv import load_dotenv
import os

# Carregar variáveis do arquivo .env
load_dotenv()

class BinanceClient:
    def __init__(self):
        # Pega as chaves API da Binance a partir das variáveis de ambiente
        self.api_key = os.getenv("BINANCE_API_KEY")
        self.api_secret = os.getenv("BINANCE_API_SECRET")
        
        if not self.api_key or not self.api_secret:
            raise ValueError("As chaves da API da Binance não estão configuradas.")
        
        # Crie uma instância do cliente da Binance
        self.client = Client(self.api_key, self.api_secret)

    def get_price(self, symbol="BTCUSDT"):
        """Obtem o preço atual de uma criptomoeda (ex: Bitcoin)."""
        try:
            ticker = self.client.get_symbol_ticker(symbol=symbol)
            return ticker['price']
        except Exception as e:
            print(f"Erro ao obter o preço: {e}")
            return None

    def get_balance(self, symbol="USDT"):
        """Obtem o saldo disponivel de um simbolo (ex: USDT ou BRL)."""
        try:
            account_info = self.client.get_account()
            for balance in account_info['balances']:
                if balance['asset'] == symbol:
                    return float(balance['free'])
            return 0.0
        except Exception as e:
            print(f"Erro ao obter o saldo: {e}")
            return None

    def convert_usdt_to_btc(self, amount_usdt):
        """Converte USDT para BTC."""
        try:
            # Realiza uma ordem de compra de BTC com USDT
            order = self.client.order_market_buy(symbol="USDTBTC", quantity=amount_usdt)
            print(f"Ordem realizada para converter {amount_usdt} USDT em BTC.")
            return order
        except Exception as e:
            print(f"Erro ao realizar a ordem de conversão: {e}")
            return None

    def place_order(self, symbol="BTCUSDT", side="BUY", quantity=1, price=None):
        """Exemplo de função para realizar uma ordem de compra ou venda."""
        try:
            if side == "BUY":
                order = self.client.order_market_buy(symbol=symbol, quantity=quantity)
            elif side == "SELL":
                order = self.client.order_market_sell(symbol=symbol, quantity=quantity)
            else:
                raise ValueError("A ordem deve ser 'BUY' ou 'SELL'")

            return order
        except Exception as e:
            print(f"Erro ao realizar a ordem: {e}")
            return None

# Testando a função de obter o preço
if __name__ == "__main__":
    client = BinanceClient()

    # Verificar o saldo de BRL
    balance_brl = client.get_balance("BRL")
    print(f"Saldo disponivel para troca: {balance_brl} BRL")

    # Verificar o preço atual do Bitcoin
    price = client.get_price("BTCUSDT")
    if price:
        print(f"O preco atual do Bitcoin é: {price}")

    # Verificar o saldo de USDT
    balance_usdt = client.get_balance("USDT")
    print(f"Saldo disponivel para troca: {balance_usdt} USDT")

    # Caso o saldo esteja em USDT, converta para BTC
    if balance_usdt > 0:
        # Converter USDT para BTC
        conversion_order_usdt_btc = client.convert_usdt_to_btc(balance_usdt)
        if conversion_order_usdt_btc:
            # Agora, obter o saldo de BTC
            balance_btc = client.get_balance("BTC")
            print(f"Saldo de BTC disponivel para compra: {balance_btc} BTC")

            # Verificar se há saldo suficiente para comprar BTC
            if balance_btc > 0.0:
                # Realizar uma ordem de compra (exemplo)
                order = client.place_order(symbol="BTCUSDT", side="BUY", quantity=0.001)
                if order:
                    print(f"Ordem realizada: {order}")
                else:
                    print("Erro ao realizar a ordem de compra.")
        else:
            print("Erro ao tentar converter USDT para BTC.")
    else:
        print("Saldo insuficiente de USDT para realizar a conversão.")
