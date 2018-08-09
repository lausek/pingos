macro_rules! vga_print {
    ($($arg:tt)*) => (::graphics::vga::writer::WRITER.lock().write_fmt(format_args!($($arg)*)).unwrap());
}

macro_rules! vga_println {
    () => (vga_print!("\n"));
    ($fmt:expr) => (vga_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (vga_print!(concat!($fmt, "\n"), $($arg)*));
}

