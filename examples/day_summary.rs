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
use chrono::NaiveDate;
use mercadobitcoin::{Coin, MercadoBitcoin, MercadoBitcoinError};

#[tokio::main]
async fn main() -> Result<(), MercadoBitcoinError> {
    env_logger::init();
    let coin = Coin::Btc;
    let mb = MercadoBitcoin::new();
    let date = NaiveDate::from_ymd(2021, 6, 7);
    let day_summary = mb.day_summary(coin, &date).await?;
    println!("{:#?}", day_summary);
    Ok(())
}
