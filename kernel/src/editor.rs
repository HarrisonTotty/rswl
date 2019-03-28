//! editor.rs - Contains editor definitions.

use std::borrow::Cow::{self, Owned};
use colored::*;

// ---------------- Structs & Implementation ----------------

/// A structure utilized in readline completions and highlighting.
pub struct WLHelper {
    /// A filename completer.
    filename_compl: rustyline::completion::FilenameCompleter,
    /// A matching bracket highlighter.
    bracket_hl:     rustyline::highlight::MatchingBracketHighlighter
}


/// Implements `Completer` on `WLHelper`.
impl rustyline::completion::Completer for WLHelper {
    /// Set the candidate type.
    type Candidate = rustyline::completion::Pair;

    /// Performs a completion.
    fn complete(&self, line: &str, pos: usize) -> Result<(usize, Vec<Self::Candidate>), rustyline::error::ReadlineError> {
        self.filename_compl.complete(line, pos)
    }
}


/// Implements `Hinter` on `WLHelper`.
impl rustyline::hint::Hinter for WLHelper {
    /// Performs a hint.
    fn hint(&self, _line: &str, _pos: usize) -> Option<String> {
        None
    }
}


/// Implements `Highlighter` on `WLHelper`.
impl rustyline::highlight::Highlighter for WLHelper {
    /// Highlight hints.
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("{}", hint.blue().bold()))
    }

    /// Highlight lines.
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.bracket_hl.highlight(line, pos)
    }

    /// Returns whether to highlight a specific character.
    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.bracket_hl.highlight_char(line, pos)
    }
}


/// Implements `Helper` for `WLHelper`.
impl rustyline::Helper for WLHelper {}


/// Implements additional methods and functions for `WLHelper`.
impl WLHelper {
    /// Creates a new `WLHelper`.
    fn new() -> WLHelper {
        WLHelper {
            filename_compl: rustyline::completion::FilenameCompleter::new(),
            bracket_hl:     rustyline::highlight::MatchingBracketHighlighter::new()
        }
    }
}

// ----------------------------------------------------------



// -------------------- Public Functions --------------------

/// Initializes a new root editor object.
pub fn init(color_enabled: bool, compl_prmpt_lim: usize, max_hist_len: usize) -> rustyline::Editor<WLHelper> {
    debug!("Initializing editor configuration...");
    let conf = rustyline::Config::builder()
        .auto_add_history(false)
        .color_mode(match color_enabled {
            true => rustyline::config::ColorMode::Enabled,
            _    => rustyline::config::ColorMode::Disabled
        })
        .completion_prompt_limit(compl_prmpt_lim)
        .completion_type(rustyline::config::CompletionType::List)
        .edit_mode(rustyline::config::EditMode::Emacs)
        .history_ignore_dups(true)
        .history_ignore_space(false)
        .max_history_size(max_hist_len)
        .build();

    debug!("Initializing editor completion system...");
    let helper = WLHelper::new();

    debug!("Initializing editor...");
    let mut editor = rustyline::Editor::with_config(conf);
    editor.set_helper(Some(helper));

    return editor;
}

// ----------------------------------------------------------
