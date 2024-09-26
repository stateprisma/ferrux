use super::pl011::Uart;
use core::fmt;
use spin::lazy::Lazy;
use spin::Mutex;

pub static BACKEND: Lazy<Mutex<Uart>> = Lazy::new(|| Mutex::new(Uart::new(0x9000000)));

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}
