use crate::uci::cmd::{command::Command, error::ParsingError};

use super::single_token::try_single_token_cmd;

/// Allows to parse a [Command] into an `uci` command.
///
/// # Examples
/// ```
/// use ivy_engine::uci::Command;
/// use ivy_engine::uci::try_parse_uci_cmd;
///
/// let cmd = Command::new("uci");
///
/// if try_parse_uci_cmd(&cmd).is_ok() {
///   println!("Command is a uci command!");
/// }
/// ```
pub fn try_parse_uci_cmd(cmd: &Command) -> Result<(), ParsingError> {
  try_single_token_cmd(cmd, "uci")
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::uci::CommandType;

  #[test]
  fn accepts_valid_input() {
    let cmd = Command::new("uci");
    assert!(try_parse_uci_cmd(&cmd).is_ok());
    assert_eq!(cmd.command_type(), Some(CommandType::Uci));
  }

  #[test]
  fn rejects_invalid_input() {
    let inp = ["uci invalid", "icu", "uci\nuci"];

    for input in inp.iter() {
      let cmd = Command::new(input);
      assert!(try_parse_uci_cmd(&cmd).is_err());
    }
  }
}
