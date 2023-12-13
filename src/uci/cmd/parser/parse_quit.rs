use crate::uci::cmd::{command::Command, error::ParsingError};

use super::single_token::try_single_token_cmd;

/// Allows to parse a [Command] into an `quit` command.
///
/// # Examples
/// ```
/// use ivy_engine::uci::Command;
/// use ivy_engine::uci::try_parse_quit_cmd;
///
/// let cmd = Command::new("quit");
///
/// if try_parse_quit_cmd(&cmd).is_ok() {
///   println!("Command is an quit command!");
/// }
/// ```
pub fn try_parse_quit_cmd(cmd: &Command) -> Result<(), ParsingError> {
  try_single_token_cmd(cmd, "quit")
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::uci::CommandType;

  #[test]
  fn accepts_valid_input() {
    let cmd = Command::new("quit");
    assert!(try_parse_quit_cmd(&cmd).is_ok());
    assert_eq!(cmd.command_type(), Some(CommandType::Quit));
  }

  #[test]
  fn rejects_invalid_input() {
    let inp = ["quit invalid", "unknown", "quit\nquit"];

    for input in inp.iter() {
      let cmd = Command::new(input);
      assert!(try_parse_quit_cmd(&cmd).is_err());
    }
  }
}
