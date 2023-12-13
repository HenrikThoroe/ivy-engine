use crate::uci::{Command, ParsingError};

pub fn try_single_token_cmd(cmd: &Command, val: &'static str) -> Result<(), ParsingError> {
  if cmd.tokens.len() != 1 {
    return Err(ParsingError::InvalidLength {
      min: 1,
      max: 1,
      got: cmd.tokens.len(),
    });
  }

  if cmd.tokens[0] != val {
    return Err(ParsingError::InvalidCommandType {
      expected: val,
      got: cmd.tokens[0].to_string(),
    });
  }

  Ok(())
}
