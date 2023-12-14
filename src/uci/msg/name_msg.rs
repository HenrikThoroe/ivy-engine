/// Constructs a name message in the UCI format.
///
/// The returned string does not contain a trailing newline.
///
/// # Examples
/// ```
/// use ivy_engine::uci::build_name_msg;
///
/// let msg = build_name_msg("Ivy 0.1.0");
/// assert_eq!(msg, "id name Ivy 0.1.0");
/// ```
pub fn build_name_msg(name: &str) -> String {
  format!("id name {}", name)
}
