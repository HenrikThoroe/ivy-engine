use crate::uci::{Command, ParsingError};

/// The payload associated with a `go` command.
///
/// Contains the time the engine should think about the position
/// and whether it should think infinitely.
///
/// If `infinite` is set to `true`, the engine should think until
/// it receives a `stop` command and ignore the `movetime` field.
pub struct GoCommandPayload {
  /// The time in milliseconds the engine should think about the
  /// position.
  pub movetime: u64,

  /// Whether the engine should think infinitely.
  ///
  /// If this is set to `true`, the engine should think until
  /// it receives a `stop` command and ignore the `movetime` field.
  pub infinite: bool,
}

/// Allows to parse a [Command] into a `go` command.
///
/// Parses the given command and returns a [GoCommandPayload].
/// If the command does not contain a `movetime` field, the
/// default value of `0` is used.
///
/// # Examples
/// ```
/// use ivy_engine::uci::Command;
/// use ivy_engine::uci::try_parse_go_cmd;
///
/// let cmd = Command::new("go movetime 1000");
///
/// if let Ok(data) = try_parse_go_cmd(&cmd) {
///   println!("Movetime: {}, Infinite: {}", data.movetime, data.infinite);
/// }
/// ```
pub fn try_parse_go_cmd(cmd: &Command) -> Result<GoCommandPayload, ParsingError> {
  if cmd.tokens.len() < 2 {
    return Err(ParsingError::InvalidLength {
      min: 2,
      max: usize::MAX,
      got: cmd.tokens.len(),
    });
  }

  if cmd.tokens[0] != "go" {
    return Err(ParsingError::InvalidCommandType {
      expected: "go",
      got: cmd.tokens[0].to_string(),
    });
  }

  let mut movetime = 0u64;
  let mut infinite = false;

  let mut tokens = cmd.tokens[1..].iter();

  while let Some(token) = tokens.next() {
    match *token {
      "movetime" => {
        if let Some(token) = tokens.next() {
          if let Ok(time) = token.parse::<u64>() {
            movetime = time;
          } else {
            return Err(ParsingError::UnknownToken {
              token: token.to_string(),
            });
          }
        } else {
          return Err(ParsingError::UnknownToken {
            token: token.to_string(),
          });
        }
      }
      "infinite" => infinite = true,
      _ => {
        return Err(ParsingError::UnknownToken {
          token: token.to_string(),
        })
      }
    }
  }

  Ok(GoCommandPayload { movetime, infinite })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn accepts_infinite() {
    let cmd = Command::new("go infinite");
    let payload = try_parse_go_cmd(&cmd).unwrap();
    assert_eq!(payload.movetime, 0);
    assert_eq!(payload.infinite, true);
  }

  #[test]
  fn accepts_movetime() {
    let cmd = Command::new("go movetime 1000");
    let payload = try_parse_go_cmd(&cmd).unwrap();
    assert_eq!(payload.movetime, 1000);
    assert_eq!(payload.infinite, false);
  }

  #[test]
  fn can_mix_movetime_and_infinite() {
    let cmd = Command::new("go movetime 1000 infinite");
    let payload = try_parse_go_cmd(&cmd).unwrap();
    assert_eq!(payload.movetime, 1000);
    assert_eq!(payload.infinite, true);
  }

  #[test]
  fn fails_with_negative_movetime() {
    let cmd = Command::new("go movetime -1000");
    let payload = try_parse_go_cmd(&cmd);
    assert!(payload.is_err());
  }

  #[test]
  fn fails_with_invalid_token() {
    let cmd = Command::new("go invalid");
    let payload = try_parse_go_cmd(&cmd);
    assert!(payload.is_err());
  }

  #[test]
  fn fails_with_invalid_command_type() {
    let cmd = Command::new("invalid");
    let payload = try_parse_go_cmd(&cmd);
    assert!(payload.is_err());
  }
}
