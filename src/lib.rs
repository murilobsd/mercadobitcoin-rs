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
use serde::{Deserialize, Serialize};
use std::convert::AsRef;
use strum_macros::AsRefStr;
use rust_decimal::Decimal;

const MB_URL: &str = "https://www.mercadobitcoin.net/api/";

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

#[derive(Debug, Clone, Deserialize)]
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


#[derive(Debug, Clone)]
pub struct OrderBookResp {
    pub asks: Vec<Vec<f32>>,
    pub bids: Vec<Vec<f32>>,
}

#[derive(Debug, Clone)]
pub enum TradeType {
    Sell,
    Buy,
}

#[derive(Debug, Clone)]
pub struct TradesResp {
    pub date: DateTime<Utc>,
    pub price: f32,
    pub amount: f32,
    pub tid: usize,
    pub tp: TradeType,
}

#[derive(Debug, Clone)]
pub struct DaySummaryResp {
    pub date: Date<Utc>,
    pub opening: f32,
    pub closing: f32,
    pub lowest: f32,
    pub highest: f32,
    pub volume: f32,
    pub quantity: f32,
    pub amount: usize,
    pub avg_price: f32,
}

#[derive(Debug, Clone)]
pub struct MercadoBitcoin {}

impl MercadoBitcoin {
    pub fn new() -> Self {
        Self {}
    }

    /// Retorna informações com o resumo das últimas 24 horas de negociações.
    pub async fn ticker(
        &self,
        coin: Coin,
    ) -> Result<Ticker, reqwest::Error> {
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
    pub fn order_book(&self, coin: Coin) {}

    /// Histórico de negociações realizadas.
    pub fn trades(&self, coin: Coin) {}

    pub fn day_summary(&self, coin: Coin) {}

    async fn call(
        &self,
        url: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        debug!("Request: {}", url);

        let resp = reqwest::Client::new().get(url).send().await?;
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
