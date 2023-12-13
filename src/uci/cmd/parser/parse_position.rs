use regex::Regex;

use crate::uci::{Command, ParsingError};

const FEN_REGEX: &str = r"^(?<PiecePlacement>((?<RankItem>[pnbrqkPNBRQK1-8]{1,8})\/?){8})\s+(?<SideToMove>b|w)\s+(?<Castling>-|K?Q?k?q)\s+(?<EnPassant>-|[a-h][3-6])\s+(?<HalfMoveClock>\d+)\s+(?<FullMoveNumber>\d+)\s*$";

const MOVE_REGEX: &str = r"^[a-h][1-8][a-h][1-8][rnbqRNBQ]?$";

/// The payload associated with a `position` command.
///
/// Contains the FEN position and a list of moves performed on
/// this position.
pub struct PositionCommandPayload {
  /// The FEN nontation of the board position, before the
  /// moves are performed.
  pub fen: String,

  /// A list of moves performed on the position.
  pub moves: Vec<String>,
}

/// Allows to parse a [Command] into a `position` command.
///
/// Parses the given command and returns a [PositionCommandPayload].
/// If the command shortens the FEN part to `startpos`, the default
/// FEN for standard chess is used.
///
/// # Examples
/// ```
/// use ivy_engine::uci::Command;
/// use ivy_engine::uci::try_parse_position_cmd;
///
/// let cmd = Command::new("position startpos moves e2e4 e7e5");
///
/// if let Ok(data) = try_parse_position_cmd(&cmd) {
///   println!("FEN: {}, Moves: {:?}", data.fen, data.moves);
/// }
/// ```
pub fn try_parse_position_cmd(cmd: &Command) -> Result<PositionCommandPayload, ParsingError> {
  if cmd.tokens.len() < 2 {
    return Err(ParsingError::InvalidLength {
      min: 2,
      max: usize::MAX,
      got: cmd.tokens.len(),
    });
  }

  //? Extract and validate position part

  let fen_tokens: Vec<&str> = cmd.tokens[1..]
    .iter()
    .take_while(|t| **t != "moves")
    .map(|t| *t)
    .collect();

  let mut fen = match fen_tokens.len() {
    1 => fen_tokens[0].to_string(),
    7 => fen_tokens[1..].join(" "),
    _ => {
      return Err(ParsingError::InvalidLength {
        min: 7,
        max: 7,
        got: fen_tokens.len(),
      })
    }
  };

  if fen == "startpos" {
    fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
  }

  if !is_valid_fen(&fen) {
    return Err(ParsingError::UnknownToken { token: fen });
  }

  //? Extract and validate moves list

  let offset = 1 + fen_tokens.len();
  let moves = if cmd.tokens.len() > offset {
    cmd.tokens[offset..].to_vec()
  } else {
    vec![]
  };

  let fen_moves = if moves.len() > 0 {
    moves[1..].to_vec()
  } else {
    vec![]
  };

  if fen_moves.iter().any(|m| !is_valid_move(m)) {
    return Err(ParsingError::UnknownToken {
      token: fen_moves[0].to_string(),
    });
  }

  Ok(PositionCommandPayload {
    fen,
    moves: fen_moves.iter().map(|m| m.to_string()).collect(),
  })
}

fn is_valid_move(move_str: &str) -> bool {
  let re = Regex::new(MOVE_REGEX).unwrap();
  re.is_match(move_str)
}

fn is_valid_fen(fen: &str) -> bool {
  let re = Regex::new(FEN_REGEX).unwrap();
  re.is_match(fen)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn checks_move_format() {
    let valid = [
      "a1a2", "a1a2q", "a1a2r", "a1a2b", "a1a2n", "a1a2Q", "a1a2R", "a1a2B", "a1a2N",
    ];

    for line in valid {
      assert!(is_valid_move(line));
    }

    let invalid = [
      "a1a2x", "a1a2qq", "a1a2rr", "a1a2bb", "a1a2nn", "", "\n", "o9a2q", "aaaa",
    ];

    for line in invalid {
      assert!(!is_valid_move(line));
    }
  }

  #[test]
  fn checks_fen_format() {
    let valid = [
      "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
      "rnbqkb1r/pppppppp/5n2/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq c3 0 2",
      "8/8/8/8/8/8/8/8 b - - 0 0",
    ];

    for line in valid {
      assert!(is_valid_fen(line));
    }

    let invalid = [
      "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
      "rnbqkb1r/pppppppp/5n2/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq c3 0",
      "8/8/8/8/8/8/8/8 b - - 0",
      "8/8/8/8/8/8/8/8 b - - 0 0 0",
      "- - - - - -",
    ];

    for line in invalid {
      assert!(!is_valid_fen(line));
    }
  }

  #[test]
  fn handles_startpos() {
    let cmd = Command::new("position startpos");
    let payload = try_parse_position_cmd(&cmd).unwrap();

    assert_eq!(
      payload.fen,
      "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
    );
    assert_eq!(payload.moves.len(), 0);
  }

  #[test]
  fn handles_fen() {
    let cmd =
      Command::new("position fen rnbqkb1r/pppppppp/5n2/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq c3 0 2");
    let payload = try_parse_position_cmd(&cmd).unwrap();

    assert_eq!(payload.moves.len(), 0);
    assert_eq!(
      payload.fen,
      "rnbqkb1r/pppppppp/5n2/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq c3 0 2".to_string()
    );
  }

  #[test]
  fn accepts_empty_moves() {
    let cmd = Command::new("position startpos moves");
    let payload = try_parse_position_cmd(&cmd).unwrap();

    assert_eq!(payload.moves.len(), 0);
    assert_eq!(
      payload.fen,
      "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
    );
  }

  #[test]
  fn handles_move_list() {
    let cmd_with_startpos = Command::new("position startpos moves e2e4 e7e5");
    let payload_with_startpos = try_parse_position_cmd(&cmd_with_startpos).unwrap();

    assert_eq!(payload_with_startpos.moves.len(), 2);
    assert!(payload_with_startpos.moves.contains(&"e2e4".to_string()));
    assert!(payload_with_startpos.moves.contains(&"e7e5".to_string()));
    assert_eq!(
      payload_with_startpos.fen,
      "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
    );

    let cmd_with_fen = Command::new(
      "position fen rnbqkb1r/pppppppp/5n2/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq c3 0 2 moves e7e5",
    );
    let payload_with_fen = try_parse_position_cmd(&cmd_with_fen).unwrap();

    assert_eq!(payload_with_fen.moves.len(), 1);
    assert!(payload_with_fen.moves.contains(&"e7e5".to_string()));
    assert_eq!(
      payload_with_fen.fen,
      "rnbqkb1r/pppppppp/5n2/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq c3 0 2".to_string()
    );
  }

  #[test]
  fn handles_invlid_invocation() {
    let inp = [
      "",
      "position",
      "position strtpos",
      "position fen invalid",
      "position startpos mves",
      "position startpos moves invalid",
    ];

    for input in inp.iter() {
      let cmd = Command::new(input);
      assert!(try_parse_position_cmd(&cmd).is_err());
    }
  }
}
