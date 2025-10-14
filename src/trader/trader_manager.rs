use crate::models::trader_stream::TraderStream;
use crate::models::virtual_currency::VirtualCurrency;
use anyhow::Result;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};
use std::collections::HashMap;
use std::io::{Write, stdout};

pub struct TraderManager {
    pub stream_map: HashMap<String, VirtualCurrency>,
}

impl TraderManager {
    pub fn new() -> Self {
        TraderManager {
            stream_map: HashMap::new(),
        }
    }

    pub fn update(&mut self, stream: TraderStream) -> Result<()> {
        if let Some(data) = stream.data {
            self.stream_map.insert(stream.stream, data);
            self.print_current_stream()?;
        }
        Ok(())
    }

    ///打印当前的订阅流
    pub fn print_current_stream(&self) -> Result<()> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        execute!(
            stdout,
            SetForegroundColor(Color::Green),
            Print("📊 Crypto Prices (Binance)\n".to_string()),
            ResetColor
        )?;
        self.stream_map.values().for_each(|currency| {
            execute!(
                stdout,
                SetForegroundColor(Color::Blue),
                Print(format!("{}: {:.2}\n", currency.symbol, currency.price)),
                ResetColor
            )
            .unwrap();
        });
        stdout.flush()?;
        Ok(())
    }
}
