use super::CommandType;

/// Represents a command sent to the engine.
///
/// A command consists of a set of tokens, parsed from an input line.
/// Leading, trailing or multiple whitespaces are ignored, when tokenizing.
/// Use the decicated provider to interpret the command based on its type.
///
/// # Examples
/// ```
/// use ivy_engine::uci::Command;
/// use ivy_engine::uci::CommandType;
///
/// let cmd = Command::new("  position   startpos moves e2e4 e7e5\n");
///
/// if let Some(CommandType::Position) = cmd.command_type() {
///   //...
/// }
/// ```
pub struct Command<'a> {
  /// The tokens of the command.
  ///
  /// The first token is the type
  /// and the remaining tokens are the payload.
  /// A token includes at least one non-whitespace character.
  pub tokens: Vec<&'a str>,
}

impl Command<'_> {
  /// Creates a new command from an input line.
  ///
  /// The input can include leading, trailing or multiple whitespaces.
  pub fn new(line: &str) -> Command {
    let tokens = line.split_whitespace().filter(|p| p.len() > 0).collect();
    Command { tokens }
  }
}

impl Command<'_> {
  /// Returns the type of the command.
  ///
  /// The type is determined by the first token.
  /// If the command has no tokens or the prefix is unknown, `None` is returned.
  pub fn command_type(&self) -> Option<CommandType> {
    if self.tokens.len() == 0 {
      return None;
    }

    match self.tokens[0] {
      "uci" => Some(CommandType::Uci),
      "debug" => Some(CommandType::Debug),
      "isready" => Some(CommandType::IsReady),
      "setoption" => Some(CommandType::SetOption),
      "ucinewgame" => Some(CommandType::UciNewGame),
      "position" => Some(CommandType::Position),
      "go" => Some(CommandType::Go),
      "stop" => Some(CommandType::Stop),
      "quit" => Some(CommandType::Quit),
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_whitespace() {
    let cmd = Command::new("some command    with whitespace \n\n");

    assert_eq!(cmd.tokens.len(), 4);
    assert_eq!(cmd.tokens[0], "some");
    assert_eq!(cmd.tokens[1], "command");
    assert_eq!(cmd.tokens[2], "with");
    assert_eq!(cmd.tokens[3], "whitespace");
  }

  #[test]
  fn detects_type() {
    let inp = [
      ("uci", Some(CommandType::Uci)),
      ("debug", Some(CommandType::Debug)),
      ("isready", Some(CommandType::IsReady)),
      ("setoption", Some(CommandType::SetOption)),
      ("ucinewgame", Some(CommandType::UciNewGame)),
      ("position", Some(CommandType::Position)),
      ("go", Some(CommandType::Go)),
      ("stop", Some(CommandType::Stop)),
      ("quit", Some(CommandType::Quit)),
      ("", None),
      ("unknown", None),
    ];

    for (input, expected) in inp.iter() {
      let cmd = Command::new(input);
      assert_eq!(cmd.command_type(), *expected);
    }
  }

  #[test]
  fn ignores_payload_for_type() {
    let inp = [
      ("uci some   payload\n\n", Some(CommandType::Uci)),
      ("debug here  some payload", Some(CommandType::Debug)),
      ("isready some payload", Some(CommandType::IsReady)),
      ("setoption some payload", Some(CommandType::SetOption)),
      ("ucinewgame some payload", Some(CommandType::UciNewGame)),
      ("position   ", Some(CommandType::Position)),
      ("go some payload", Some(CommandType::Go)),
      ("stop some payload", Some(CommandType::Stop)),
      ("quit some payload", Some(CommandType::Quit)),
      ("    unknown some", None),
    ];

    for (input, expected) in inp.iter() {
      let cmd = Command::new(input);
      assert_eq!(cmd.command_type(), *expected);
    }
  }
}
