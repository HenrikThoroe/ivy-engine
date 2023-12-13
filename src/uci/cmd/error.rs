use snafu::prelude::*;

/// Errors that can occur while parsing a command.
#[derive(Debug, Snafu)]
pub enum ParsingError {
  /// The command type is invalid.
  ///
  /// May occur when the trying to parse an position command
  /// from `go` or any other command with invalid prefix.
  #[snafu(display("Invalid command type. Expected {}, got {}", expected, got))]
  InvalidCommandType {
    /// The expected prefix.
    expected: &'static str,

    /// The actual prefix.
    got: String,
  },

  /// The command has an invalid length.
  ///
  /// Occurs when trying to parse a command with an invalid amount of tokens.
  #[snafu(display("Invalid length. Expected between {} and {}, got {}", min, max, got))]
  InvalidLength {
    /// The minimum amount of tokens.
    min: usize,

    /// The maximum amount of tokens.
    max: usize,

    /// The actual amount of tokens.
    got: usize,
  },

  /// The token is unknown.
  ///
  /// Occurs when trying to parse a command with an unknown token.
  /// For example, when expecting `on` or `off` and getting `unknown`.
  #[snafu(display("Unknown token '{}'", token))]
  UnknownToken {
    /// The unknown token.
    token: String,
  },
}
