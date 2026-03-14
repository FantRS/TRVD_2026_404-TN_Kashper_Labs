use tracing::Level;
use tracing_subscriber::{
    EnvFilter, Layer, Registry, filter::filter_fn, layer::SubscriberExt, util::SubscriberInitExt,
};

pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for EnvFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Error => "error".into(),
            LogLevel::Warn => "warning".into(),
            LogLevel::Info => "info".into(),
            LogLevel::Debug => "debug".into(),
            LogLevel::Trace => "trace".into(),
        }
    }
}

pub fn init_logger(level: LogLevel) {
    let stdout_layer = tracing_subscriber::fmt::layer()
        // .json()
        .with_level(true)
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        // .with_span_list(false)
        .with_writer(std::io::stdout)
        .with_filter(filter_fn(|meta| {
            matches!(*meta.level(), Level::TRACE | Level::DEBUG | Level::INFO)
        }));

    let stderr_layer = tracing_subscriber::fmt::layer()
        // .json()
        .with_level(true)
        .with_file(false)
        .with_line_number(true)
        .with_target(true)
        // .with_span_list(true)
        .with_writer(std::io::stderr)
        .with_filter(filter_fn(|meta| {
            matches!(*meta.level(), Level::WARN | Level::ERROR)
        }));

    let env_layer = EnvFilter::try_from_default_env().unwrap_or(level.into());

    Registry::default()
        .with(stdout_layer)
        .with(stderr_layer)
        .with(env_layer)
        .init();
}
