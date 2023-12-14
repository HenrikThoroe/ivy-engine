/// Construct the author message in the UCI format.
///
/// The returned string does not contain a trailing newline.
///
/// # Examples
/// ```
/// use ivy_engine::uci::build_author_msg;
///
/// let msg = build_author_msg("Ivy Team");
/// assert_eq!(msg, "id author Ivy Team");
/// ```
pub fn build_author_msg(author: &str) -> String {
  format!("id author {}", author)
}
