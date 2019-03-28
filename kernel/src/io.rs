//! io.rs - Contains handy functions for dealing with I/O and command-line arguments.

// Import some useful macros from the clap package.
use clap::{crate_authors, crate_description, crate_version};

// Import colored extensions for strings.
use colored::*;

// -------------------- Public Functions --------------------

/// Obtains the input prompt string for the given line number.
pub fn get_in_prompt(line_number: i32, color_enabled: bool) -> String {
    match color_enabled {
        true => format!(" {}{}{} := ", "In[".blue(), line_number, "]".blue()),
        _    => format!(" In[{}] := ", line_number)
    }
}


/// Obtains the output prompt string for the given line number.
pub fn get_out_prompt(line_number: i32, color_enabled: bool) -> String {
    match color_enabled {
        true => format!("{}{}{}  = ", "Out[".blue(), line_number, "]".blue()),
        _    => format!("Out[{}]  = ", line_number)
    }
}


/// Parses the command-line arguments, returning the collection of matches.
pub fn parse_arguments<'a>() -> clap::ArgMatches<'a> {
    let argument_parser = clap::App::new("wl")
        .about(crate_description!())
        .author(crate_authors!())
        .help_message("Displays help and usage information.")
        .version(crate_version!())
        .version_message("Displays version information.")
        .settings(
            &[
                clap::AppSettings::ColoredHelp
            ]
        )
        .arg(clap::Arg::with_name("log_file")
             .env("WL_LOG_FILE")
             .help("Specifies a log file to write expressions to in addition to stdout/stderr.")
             .long("log-file")
             .short("f")
             .value_name("FILE")
        )
        .arg(clap::Arg::with_name("log_level")
             .default_value("info")
             .env("WL_LOG_LEVEL")
             .help("Specifies the logging level of the program.")
             .long("log-level")
             .possible_values(
                 &[
                     "off",
                     "error",
                     "warn",
                     "info",
                     "debug",
                     "trace"
                 ]
             )
             .short("l")
             .value_name("LVL")
        )
        .arg(clap::Arg::with_name("log_mode")
             .default_value("append")
             .env("WL_LOG_MODE")
             .help("Specifies whether to append to, or overwrite, the specified log file.")
             .long("log-mode")
             .possible_values(&["append", "overwrite"])
             .short("M")
             .value_name("MODE")
        )
        .arg(clap::Arg::with_name("no_color")
             .help("Disables color output to stdout/stderr.")
             .long("--no-color")
        );
    return argument_parser.get_matches();
}


/// Sets-up logging for the program.
pub fn setup_logging(log_file: &str, log_level: &str, log_mode: &str) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(
                format_args!(
                    "[{}] [{}] [{}] {}",
                    record.level(),
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.target(),
                    message
                )
            )
        })
        .level(match log_level {
            "off"   => log::LevelFilter::Off,
            "error" => log::LevelFilter::Error,
            "warn"  => log::LevelFilter::Warn,
            "info"  => log::LevelFilter::Info,
            "debug" => log::LevelFilter::Debug,
            _       => log::LevelFilter::Trace,
        })
        .chain(std::fs::OpenOptions::new()
               .write(true)
               .create(true)
               .append(match log_mode {
                   "append" => true,
                   _        => false
               })
               .open(log_file)?
        )
        .apply()?;
    return Ok(());
}

// ----------------------------------------------------------
