/// The score of a move.
///
/// Can be either a centipawn score or a mate score.
/// If the score is a mate score, the value is the amount of moves, not plies.
/// The score is relative to the color of the moving side.
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum Score {
  /// The score in centipawns.
  Cp(i32),

  /// The score in moves to mate.
  Mate(i32),
}

/// Information about a move.
///
/// MoveInfo contains all information about the search supported
/// by the UCI protocol.
#[derive(Hash, Eq, PartialEq)]
pub enum MoveInfo {
  /// The current depth of the search in plies.
  Depth(u32),

  /// The current seldepth of the search in plies.
  SelDepth(u32),

  /// The time spent searching in milliseconds.
  Time(u32),

  /// The amount of nodes searched.
  Nodes(u32),

  /// The principal variation.
  Pv(Vec<String>),

  /// The score of the move.
  Score {
    /// The value of the score in centipawns or moves to mate.
    score: Score,

    /// Whether the score is a lower bound.
    lower_bound: bool,

    /// Whether the score is an upper bound.
    upper_bound: bool,
  },

  /// The currently searched move.
  CurrMove(String),

  /// The 1 based index of the currently searched move at root level.
  CurrMoveNumber(u32),

  /// The used capacity of the hash table in permille.
  /// Can be in the range `[0, 1000]`.
  HashFull(u32),

  /// The amount of nodes searched per second.
  Nps(u32),

  /// The amount of tablebase hits in the current search.
  TbHits(u32),

  /// The current CPU load in permille.
  /// Can be in the range `[0, 1000]`.
  Cpuload(u32),

  /// A custom info message.
  /// Will be placed at the back of the message and
  /// contain even reserved UCI keywords.
  Custom(String),

  /// The refutation to the found move.
  /// The first entry is the explored move,
  /// and the following entries are the refutation
  /// if the move is refuted.
  Refutation(Vec<String>),

  /// The index of the selecetd pv.
  /// When the engine should select the nth pv,
  /// the engine should send `info multipv n`.
  MultiPv(u32),

  /// The current line on which the given task is searching.
  CurrLine {
    /// The index of the task which is evaluating the line.
    task: u32,

    /// The currently searched line.
    line: Vec<String>,
  },
}

/// Constructs an info message in the UCI format.
///
/// The returned string does not contain a trailing newline.
///
/// # Examples
/// ```
/// use ivy_engine::uci::build_info_msg;
/// use ivy_engine::uci::MoveInfo;
/// use ivy_engine::uci::Score;
///
/// let mut info = vec![];
/// info.push(MoveInfo::Depth(1));
/// info.push(MoveInfo::SelDepth(2));
/// info.push(MoveInfo::Time(3));
/// info.push(MoveInfo::Nodes(4));
///
/// let msg = build_info_msg(info.as_slice());
/// assert_eq!(msg, "info depth 1 seldepth 2 time 3 nodes 4");
/// ```
pub fn build_info_msg(info: &[MoveInfo]) -> String {
  let mut msg = String::from("info");
  let mut string_info: Option<&String> = None;

  let fmt_score = |score: &Score, lb: bool, ub: bool| {
    let mut msg = match score {
      Score::Cp(cp) => format!(" score cp {}", cp),
      Score::Mate(mate) => format!(" score mate {}", mate),
    };

    if lb {
      msg.push_str(" lowerbound");
    }

    if ub {
      msg.push_str(" upperbound");
    }

    msg
  };

  for part in info {
    match part {
      MoveInfo::Depth(depth) => msg.push_str(&format!(" depth {}", depth)),
      MoveInfo::SelDepth(sel_depth) => msg.push_str(&format!(" seldepth {}", sel_depth)),
      MoveInfo::Time(time) => msg.push_str(&format!(" time {}", time)),
      MoveInfo::Nodes(nodes) => msg.push_str(&format!(" nodes {}", nodes)),
      MoveInfo::Pv(pv) => msg.push_str(&(" pv ".to_string() + &pv.join(" "))),
      MoveInfo::MultiPv(multi_pv) => msg.push_str(&format!(" multipv {}", multi_pv)),
      MoveInfo::HashFull(hash_full) => msg.push_str(&format!(" hashfull {}", hash_full)),
      MoveInfo::Nps(nps) => msg.push_str(&format!(" nps {}", nps)),
      MoveInfo::TbHits(tb_hits) => msg.push_str(&format!(" tbhits {}", tb_hits)),
      MoveInfo::Cpuload(cpuload) => msg.push_str(&format!(" cpuload {}", cpuload)),
      MoveInfo::Custom(custom) => string_info = Some(custom),
      MoveInfo::CurrMove(curr_move) => msg.push_str(&format!(" currmove {}", curr_move)),

      MoveInfo::Score {
        score,
        lower_bound,
        upper_bound,
      } => msg.push_str(&fmt_score(score, *lower_bound, *upper_bound)),

      MoveInfo::CurrMoveNumber(curr_move_number) => {
        msg.push_str(&format!(" currmovenumber {}", curr_move_number))
      }

      MoveInfo::Refutation(refutation) => {
        msg.push_str(&(" refutation ".to_string() + &refutation.join(" ")))
      }

      MoveInfo::CurrLine { task: cpu, line } => {
        msg.push_str(&format!(" currline {} {}", cpu, line.join(" ")))
      }
    }
  }

  if let Some(custom) = string_info {
    msg.push_str(&format!(" string {}", custom));
  }

  msg
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn accepts_full_input() {
    let mut info = vec![];
    info.push(MoveInfo::Depth(1));
    info.push(MoveInfo::SelDepth(2));
    info.push(MoveInfo::Time(3));
    info.push(MoveInfo::Nodes(4));
    info.push(MoveInfo::Pv(vec!["e2e4".to_string(), "e7e5".to_string()]));
    info.push(MoveInfo::Score {
      score: Score::Cp(100),
      lower_bound: false,
      upper_bound: false,
    });
    info.push(MoveInfo::CurrMove("e2e4".to_string()));
    info.push(MoveInfo::CurrMoveNumber(5));
    info.push(MoveInfo::HashFull(6));
    info.push(MoveInfo::Nps(7));
    info.push(MoveInfo::TbHits(8));
    info.push(MoveInfo::Cpuload(9));
    info.push(MoveInfo::Custom("custom".to_string()));
    info.push(MoveInfo::Refutation(vec![
      "e2e4".to_string(),
      "e7e5".to_string(),
    ]));
    info.push(MoveInfo::MultiPv(10));
    info.push(MoveInfo::CurrLine {
      task: 11,
      line: vec!["e2e4".to_string(), "e7e5".to_string()],
    });

    let msg = build_info_msg(info.as_slice());

    assert_eq!(
            msg,
            "info depth 1 seldepth 2 time 3 nodes 4 pv e2e4 e7e5 score cp 100 currmove e2e4 currmovenumber 5 hashfull 6 nps 7 tbhits 8 cpuload 9 refutation e2e4 e7e5 multipv 10 currline 11 e2e4 e7e5 string custom"
        );
  }

  #[test]
  fn formats_score() {
    let scores = [
      (Score::Cp(100), false, false, "info score cp 100"),
      (Score::Cp(100), true, false, "info score cp 100 lowerbound"),
      (Score::Cp(100), false, true, "info score cp 100 upperbound"),
      (
        Score::Cp(100),
        true,
        true,
        "info score cp 100 lowerbound upperbound",
      ),
      (Score::Mate(100), false, false, "info score mate 100"),
      (
        Score::Mate(100),
        true,
        false,
        "info score mate 100 lowerbound",
      ),
      (
        Score::Mate(100),
        false,
        true,
        "info score mate 100 upperbound",
      ),
      (
        Score::Mate(100),
        true,
        true,
        "info score mate 100 lowerbound upperbound",
      ),
    ];

    for (score, lb, ub, expected) in scores.iter() {
      assert_eq!(
        build_info_msg(&[MoveInfo::Score {
          score: *score,
          lower_bound: *lb,
          upper_bound: *ub,
        }]),
        *expected
      );
    }
  }

  #[test]
  fn reorders_custom_string() {
    let mut info = vec![];
    info.push(MoveInfo::Depth(1));
    info.push(MoveInfo::Custom("custom".to_string()));
    info.push(MoveInfo::SelDepth(2));

    let msg = build_info_msg(info.as_slice());

    assert_eq!(msg, "info depth 1 seldepth 2 string custom");
  }
}
