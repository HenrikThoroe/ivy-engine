use crate::uci::cmd::{command::Command, error::ParsingError};

use super::single_token::try_single_token_cmd;

/// Allows to parse a [Command] into an `ucinewgame` command.
///
/// # Examples
/// ```
/// use ivy_engine::uci::Command;
/// use ivy_engine::uci::try_parse_uci_new_game_cmd;
///
/// let cmd = Command::new("ucinewgame");
///
/// if try_parse_uci_new_game_cmd(&cmd).is_ok() {
///   println!("Command is an ucinewgame command!");
/// }
/// ```
pub fn try_parse_uci_new_game_cmd(cmd: &Command) -> Result<(), ParsingError> {
  try_single_token_cmd(cmd, "ucinewgame")
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::uci::CommandType;

  #[test]
  fn accepts_valid_input() {
    let cmd = Command::new("ucinewgame");
    assert!(try_parse_uci_new_game_cmd(&cmd).is_ok());
    assert_eq!(cmd.command_type(), Some(CommandType::UciNewGame));
  }

  #[test]
  fn rejects_invalid_input() {
    let inp = ["ucinewgame invalid", "unknown", "ucinewgame\nucinewgame"];

    for input in inp.iter() {
      let cmd = Command::new(input);
      assert!(try_parse_uci_new_game_cmd(&cmd).is_err());
    }
  }
}
