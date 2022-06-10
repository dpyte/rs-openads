
use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::{append::{
    console::{ConsoleAppender, Target},
    file::FileAppender,
}, config::{Appender, Config, Root}, encode::pattern::PatternEncoder, filter::threshold::ThresholdFilter, Handle};

static LOG_FILE: &str = "/var/system/openads/config/log/log_config.yaml";
