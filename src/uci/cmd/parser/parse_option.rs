use crate::uci::{Command, ParsingError};

/// The payload associated with the `setoption` command.
///
/// Contains the name and value of the option.
/// If the value is ommitted, the field will be an empty string.
pub struct OptionCommandPayload {
  /// The name of the option.
  pub name: String,

  /// The value of the option.
  /// An empty string if ommitted.
  pub value: String,
}

/// Allows to parse a [Command] into a `setoption` command.
///
/// Parses the given command and returns a [OptionCommandPayload].
/// If the command does not contain a `value` field, the
/// default value of `""` is used.
///
/// # Examples
/// ```
/// use ivy_engine::uci::Command;
/// use ivy_engine::uci::try_parse_option_cmd;
///
/// let cmd = Command::new("setoption name Hash value 128");
///
/// if let Ok(data) = try_parse_option_cmd(&cmd) {
///   println!("Name: {}, Value: {}", data.name, data.value);
/// }
/// ```
pub fn try_parse_option_cmd(cmd: &Command) -> Result<OptionCommandPayload, ParsingError> {
  if cmd.tokens.len() < 3 {
    return Err(ParsingError::InvalidLength {
      min: 3,
      max: usize::MAX,
      got: cmd.tokens.len(),
    });
  }

  if cmd.tokens[0] != "setoption" {
    return Err(ParsingError::InvalidCommandType {
      expected: "setoption",
      got: cmd.tokens[0].to_string(),
    });
  }

  let mut tokens = cmd.tokens[1..].iter();

  let mut name = String::new();
  let mut value = String::new();

  while let Some(token) = tokens.next() {
    match *token {
      "name" => {
        if let Some(token) = tokens.next() {
          name = token.to_string();
        } else {
          return Err(ParsingError::UnknownToken {
            token: token.to_string(),
          });
        }
      }
      "value" => {
        if let Some(token) = tokens.next() {
          value = token.to_string();
        } else {
          return Err(ParsingError::UnknownToken {
            token: token.to_string(),
          });
        }
      }
      _ => {
        return Err(ParsingError::UnknownToken {
          token: token.to_string(),
        })
      }
    }
  }

  Ok(OptionCommandPayload { name, value })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn accepts_full_input() {
    let cmd = Command::new("setoption name Hash value 128");
    let payload = try_parse_option_cmd(&cmd).unwrap();

    assert_eq!(payload.name, "Hash");
    assert_eq!(payload.value, "128");
  }

  #[test]
  fn accepts_partial_input() {
    let cmd = Command::new("setoption name Hash");
    let payload = try_parse_option_cmd(&cmd).unwrap();

    assert_eq!(payload.name, "Hash");
    assert_eq!(payload.value, "");
  }

  #[test]
  fn rejects_invalid_input() {
    let inp = ["setoption", "setoption name", "setoption name Hash value"];

    for input in inp.iter() {
      let cmd = Command::new(input);
      assert!(try_parse_option_cmd(&cmd).is_err());
    }
  }

  #[test]
  fn rejects_unknown_command() {
    let cmd = Command::new("unknown name Hash value 128");

    assert!(try_parse_option_cmd(&cmd).is_err());
  }
}
