use chrono::Local;
use log::{LevelFilter, Log, Metadata, Record};
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use tokio::sync::Mutex;



/// customized log using safe multi thread
/// by Arc and Mutex
pub struct Logger {
    log_file: Arc<Mutex<Option<File>>>,
}

/// make sure lazy initialize at once
/// run time environment
static MY_LOGGER: Lazy<Logger> = Lazy::new(|| {
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .truncate(false)
        .open("src/logs/client.log")
        .expect("Failed to open log file");

    Logger {
        log_file: Arc::new(Mutex::new(Some(log_file))),
    }
});


/// implement Log trait
impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= LevelFilter::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            let color_spec = match record.level() {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::Green,
                log::Level::Debug => Color::Blue,
                log::Level::Trace => Color::Magenta,
            };

            stdout.set_color(ColorSpec::new().set_fg(Some(color_spec)))
                .expect("Failed to set color on stdout");

            let log_message = format!(
                "{} - {:6}- [{}][{}] : {}",
                timestamp,
                record.level(),
                record.file().unwrap().replace("/","\\"),
                record.line().unwrap_or(0),
                record.args()
            );

            writeln!(&mut stdout, "{}", log_message)
                .expect("Failed to write to stdout");

            let log_file = self.log_file.clone();
            let log_message = log_message.clone();

            tokio::spawn(async move {
                let mut log_file = log_file.lock().await;
                if let Some(log_file) = &mut *log_file {
                    writeln!(log_file, "{}", log_message).expect("Failed to write to log file");
                }
            });

            stdout.reset().expect("Failed to reset color on stdout");
        }
    }

    fn flush(&self) {
        // Not needed for async logging
    }
}
/// initialization
pub async fn init_logger() {
    let log_level = LevelFilter::Debug;
    log::set_logger(&*MY_LOGGER).expect("Failed to set logger");
    log::set_max_level(log_level);
}
