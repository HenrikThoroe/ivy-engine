use crate::uci::cmd::{command::Command, error::ParsingError};

/// Allows to parse a [Command] into a `debug` command.
///
/// # Examples
/// ```
/// use ivy_engine::uci::Command;
/// use ivy_engine::uci::try_parse_debug_cmd;
///
/// let cmd = Command::new("debug on");
///
/// if let Ok(debug) = try_parse_debug_cmd(&cmd) {
///   if debug {
///     println!("Debug is on!");
///   } else {
///     println!("Debug is off!");
///   }
/// }
pub fn try_parse_debug_cmd(cmd: &Command) -> Result<bool, ParsingError> {
  if cmd.tokens.len() != 2 {
    return Err(ParsingError::InvalidLength {
      min: 2,
      max: 2,
      got: cmd.tokens.len(),
    });
  }

  if cmd.tokens[0] != "debug" {
    return Err(ParsingError::InvalidCommandType {
      expected: "debug",
      got: cmd.tokens[0].to_string(),
    });
  }

  match cmd.tokens[1] {
    "on" => Ok(true),
    "off" => Ok(false),
    _ => Err(ParsingError::UnknownToken {
      token: cmd.tokens[1].to_string(),
    }),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::uci::cmd::{command::Command, CommandType};

  #[test]
  fn accepts_valid_input() {
    let valid = ["debug on", "debug off"];

    for line in valid {
      let cmd = Command::new(line);
      assert!(try_parse_debug_cmd(&cmd).is_ok());
      assert_eq!(cmd.command_type(), Some(CommandType::Debug));
    }
  }

  #[test]
  fn rejects_invalid_input() {
    let inp = ["debug", "debug invalid", "gubed on"];

    for input in inp.iter() {
      let cmd = Command::new(input);
      assert!(try_parse_debug_cmd(&cmd).is_err());
    }
  }
}
