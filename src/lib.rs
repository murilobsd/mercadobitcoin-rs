//
// Copyright (c) 2021 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
use chrono::{Date, DateTime, Utc};
use log::debug;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::convert::AsRef;
use strum_macros::AsRefStr;

const MB_URL: &str = "https://www.mercadobitcoin.net/api/";

static APP_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(AsRefStr, Debug, Clone)]
/// Acrônimo da moeda digital.
pub enum Coin {
    AAVE,
    ACMFT,
    ACORDO01,
    ASRFT,
    ATMFT,
    AXS,
    BAL,
    BARFT,
    BAT,
    BCH,
    BTC,
    CAIFT,
    CHZ,
    COMP,
    CRV,
    DAI,
    DAL,
    ENJ,
    ETH,
    GALFT,
    GRT,
    IMOB01,
    JUVFT,
    KNC,
    LINK,
    LTC,
    MANA,
    MBCONS01,
    MBCONS02,
    MBFP01,
    MBFP02,
    MBFP03,
    MBFP04,
    MBPRK01,
    MBPRK02,
    MBPRK03,
    MBPRK04,
    MBVASCO01,
    MCO2,
    MKR,
    OGFT,
    PAXG,
    PSGFT,
    REI,
    REN,
    SNX,
    UMA,
    UNI,
    USDC,
    WBX,
    XRP,
    YFI,
    ZRX,
}

#[derive(Debug, Clone)]
pub struct TradesParameter {
    pub tid: Option<usize>,
    pub since: Option<usize>,
    pub from: Option<u64>,
    pub to: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Ticker
pub struct Ticker {
    /// Maior preço unitário de negociação das últimas 24 horas.
    pub high: Decimal,
    /// Menor preço unitário de negociação das últimas 24 horas.
    pub low: Decimal,
    /// Quantidade negociada nas últimas 24 horas.
    pub vol: Decimal,
    /// Preço unitário da última negociação.
    pub last: Decimal,
    /// Maior preço de oferta de compra das últimas 24 horas.
    pub buy: Decimal,
    /// Menor preço de oferta de venda das últimas 24 horas.
    pub sell: Decimal,
    /// Data e hora da informação em Era Unix.
    pub date: u64,
}

#[derive(Debug, Clone, Deserialize)]
/// Resposta do método Ticker
struct TickerResp {
    ticker: Ticker,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderBook {
    /// Lista de ofertas de venda, ordenadas do menor para o maior preço.
    ///
    /// Índice 0 preço unitário da oferta de compra.
    /// Índice 1 quantidade da oferta de compra.
    pub asks: Vec<Vec<Decimal>>,
    ///  Lista de ofertas de compras, ordenadas do maior para o menor preço.
    ///
    /// Índice 0 preço unitário da oferta de compra.
    /// Índice 1 quantidade da oferta de compra.
    pub bids: Vec<Vec<Decimal>>,
}

#[derive(AsRefStr, Debug, Clone, Serialize, Deserialize)]
pub enum TradeType {
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "buy")]
    Buy,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trade {
    /// Data e hora da negociação....
    pub date: u64,
    /// Preço unitário da negociação.
    pub price: Decimal,
    /// Quantidade da negociação.
    pub amount: Decimal,
    /// Quantidade da negociação.
    pub tid: usize,
    /// [Indica a ponta executora da negociação.](https://www.mercadobitcoin.com.br/info/execucao-ordem)
    #[serde(rename = "type")]
    pub tp: TradeType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DaySummary {
    /// Data do resumo diário
    pub date: String,
    /// Preço unitário de abertura de negociação no dia.
    pub opening: Decimal,
    /// Preço unitário de fechamento de negociação no dia.
    pub closing: Decimal,
    /// Menor preço unitário de negociação no dia.
    pub lowest: Decimal,
    /// Maior preço unitário de negociação no dia.
    pub highest: Decimal,
    /// Volume de Reais (BRL) negociados no dia.
    pub volume: Decimal,
    /// Quantidade da moeda digital negociada no dia.
    pub quantity: Decimal,
    /// Número de negociações realizadas no dia.
    pub amount: usize,
    /// Preço unitário médio das negociações no dia.
    pub avg_price: Decimal,
}

#[derive(Debug, Clone)]
pub struct MercadoBitcoin {
    client: reqwest::Client,
}

impl MercadoBitcoin {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap();
        Self { client }
    }

    /// Retorna informações com o resumo das últimas 24 horas de negociações.
    pub async fn ticker(&self, coin: Coin) -> Result<Ticker, reqwest::Error> {
        let coin_str = coin.as_ref();
        let method_str = "ticker";
        let url = format!("{}{}/{}/", MB_URL, coin_str, method_str);

        let resp = self.call(&url).await?;
        let ticker_resp: TickerResp = resp.json().await?;

        Ok(ticker_resp.ticker)
    }

    /// Livro de negociações, ordens abertas de compra e venda.
    ///
    /// Livro de ofertas é composto por duas listas: (1) uma lista com as
    /// ofertas de compras ordenadas pelo maior valor; (2) uma lista com as
    /// ofertas de venda ordenadas pelo menor valor. O livro mostra até 1000
    /// ofertas de compra e até 1000 ofertas de venda.
    ///
    /// Uma oferta é constituída por uma ou mais ordens, sendo assim, a
    /// quantidade da oferta é o resultado da soma das quantidades das ordens
    /// de mesmo preço unitário. Caso uma oferta represente mais de uma ordem,
    /// a prioridade de execução se dá com base na data de criação da ordem, da
    /// mais antiga para a mais nova.
    pub async fn order_book(
        &self,
        coin: Coin,
    ) -> Result<OrderBook, reqwest::Error> {
        let coin_str = coin.as_ref();
        let method_str = "orderbook";
        let url = format!("{}{}/{}/", MB_URL, coin_str, method_str);

        let resp = self.call(&url).await?;
        let ob_resp: OrderBook = resp.json().await?;

        Ok(ob_resp)
    }

    /// Histórico de negociações realizadas.
    pub async fn trades(
        &self,
        coin: Coin,
    ) -> Result<Vec<Trade>, reqwest::Error> {
        let coin_str = coin.as_ref();
        let method_str = "trades";
        let url = format!("{}{}/{}/", MB_URL, coin_str, method_str);

        let resp = self.call(&url).await?;
        let trade_resp: Vec<Trade> = resp.json().await?;

        Ok(trade_resp)
    }

    /// Retorna resumo diário de negociações realizadas.
    pub async fn day_summary(
        &self,
        coin: Coin,
    ) -> Result<DaySummary, reqwest::Error> {
        let coin_str = coin.as_ref();
        let method_str = "trades";
        let url = format!("{}{}/{}/", MB_URL, coin_str, method_str);

        let resp = self.call(&url).await?;
        let day_summary_resp: DaySummary = resp.json().await?;

        Ok(day_summary_resp)
    }

    async fn call(
        &self,
        url: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        debug!("Request: {}", url);

        let resp = self.client.get(url).send().await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn btc_coin() {
        let coin = Coin::BTC;
        assert_eq!("BTC", coin.as_ref());
    }
}
