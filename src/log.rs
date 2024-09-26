use core::fmt;
use core::fmt::Write;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use crate::arch::log::BACKEND;

    BACKEND
        .lock()
        .write_fmt(args)
        .expect("Failed to write to UART");
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::log::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::log::_print(format_args_nl!($($arg)*));
    }};
}
