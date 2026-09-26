use std::io::{self, Write};

/// 128 + SIGPIPE, the code a shell reports for a writer whose reader went away.
pub const CLOSED_EXIT: i32 = 141;

pub fn write_fmt(args: std::fmt::Arguments) {
    if let Err(e) = io::stdout().write_fmt(args) {
        exit_on(e);
    }
}

fn exit_on(e: io::Error) -> ! {
    if e.kind() == io::ErrorKind::BrokenPipe {
        std::process::exit(CLOSED_EXIT);
    }
    eprintln!("gir: cannot write stdout: {e}");
    std::process::exit(2)
}

#[macro_export]
macro_rules! out {
    ($($arg:tt)*) => { $crate::stdout::write_fmt(format_args!($($arg)*)) };
}

#[macro_export]
macro_rules! outln {
    ($($arg:tt)*) => { $crate::stdout::write_fmt(format_args!("{}\n", format_args!($($arg)*))) };
}
