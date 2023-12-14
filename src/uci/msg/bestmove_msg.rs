/// Constructs a bestmove message in the UCI format.
///
/// The returned string does not contain a trailing newline.
///
/// # Examples
/// ```
/// use ivy_engine::uci::build_bestmove_msg;
///     
/// let msg = build_bestmove_msg("e2e4");
/// assert_eq!(msg, "bestmove e2e4");
/// ```
pub fn build_bestmove_msg(bestmove: &str) -> String {
  format!("bestmove {}", bestmove)
}
