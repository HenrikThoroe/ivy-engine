use crate::uci::cmd::{command::Command, error::ParsingError};

use super::single_token::try_single_token_cmd;

/// Allows to parse a [Command] into an `stop` command.
///
/// # Examples
/// ```
/// use ivy_engine::uci::Command;
/// use ivy_engine::uci::try_parse_stop_cmd;
///
/// let cmd = Command::new("stop");
///
/// if try_parse_stop_cmd(&cmd).is_ok() {
///   println!("Command is an stop command!");
/// }
/// ```
pub fn try_parse_stop_cmd(cmd: &Command) -> Result<(), ParsingError> {
  try_single_token_cmd(cmd, "stop")
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::uci::CommandType;

  #[test]
  fn accepts_valid_input() {
    let cmd = Command::new("stop");
    assert!(try_parse_stop_cmd(&cmd).is_ok());
    assert_eq!(cmd.command_type(), Some(CommandType::Stop));
  }

  #[test]
  fn rejects_invalid_input() {
    let inp = ["stop invalid", "unknown", "stop\nstop"];

    for input in inp.iter() {
      let cmd = Command::new(input);
      assert!(try_parse_stop_cmd(&cmd).is_err());
    }
  }
}
