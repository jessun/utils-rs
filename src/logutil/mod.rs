use std::io::Write;

const LOG_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

pub fn default_init() {
    init(|_| {})
}

pub fn init_in_test() {
    init(|log_builder| {
        log_builder.is_test(true);
    });
}

pub fn init<F>(f: F)
where
    F: Fn(&mut env_logger::Builder),
{
    let mut log_builder = env_logger::Builder::from_default_env();
    format(&mut log_builder);
    f(&mut log_builder);
    log_builder.init();
}

fn format(log_builder: &mut env_logger::Builder) {
    log_builder.format(|buf, record| {
        writeln!(
            buf,
            "[{}][{}] {} [{}:{}]",
            chrono::Local::now().format(LOG_TIME_FORMAT),
            buf.default_styled_level(record.level()),
            record.args(),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
        )
    });
}

#[cfg(test)]
mod logutil_tests {
    use crate::logutil;

    #[test]
    fn test_log() {
        logutil::default_init();
        log::error!("error message");
        log::warn!("warn message");
        log::info!("info message");
        log::debug!("debug message");
        log::trace!("trace message");
    }
}
