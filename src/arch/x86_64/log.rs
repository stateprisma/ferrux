use super::uart::{Uart, COM1};
use core::fmt;
use spin::lazy::Lazy;
use spin::Mutex;

pub static BACKEND: Lazy<Mutex<Uart>> = Lazy::new(|| Mutex::new(Uart::init_uart(COM1)));

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.putstr(s.as_bytes());
        Ok(())
    }
}
