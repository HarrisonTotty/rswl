//! wl - A Wolfram Language interpreter written in Rust.

#[macro_use] extern crate log;
#[macro_use] extern crate pest_derive;

// ----- Custom Modules -----
mod editor;
mod io;
mod parser;
// --------------------------

/// The entrypoint of the program.
fn main() {
    // Parse the command-line arguments.
    let args = io::parse_arguments();

    // Setup file logging if a log file was supplied.
    if args.is_present("log_file") {
        let log_file  = args.value_of("log_file").unwrap();
        let log_level = args.value_of("log_level").unwrap();
        let log_mode  = args.value_of("log_mode").unwrap();
        match io::setup_logging(log_file, log_level, log_mode) {
            Ok(_) => debug!("Logging successfully initialized."),
            _     => panic!("Unable to initialize logging system!")
        }
    }

    // Obtain an editor.
    let mut editor = editor::init(!args.is_present("no_color"), 80, 1000);

    // Enter the main event loop.
    let line_number = 0;
    loop {
        let rl = editor.readline(&io::get_in_prompt(line_number, !args.is_present("no_color")));
        match rl {
            Ok(line) => {
                info!(" In[{}] := {}", line_number, line);
                
            },
            _ => {
                info!("Session ended.");
                break;
            }
        }
    }
}
