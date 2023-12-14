/// Constructs a ready OK message in the UCI format.
///
/// The returned string does not contain a trailing newline.
///
/// # Examples
/// ```
/// use ivy_engine::uci::build_ready_ok_msg;
///
/// let msg = build_ready_ok_msg();
/// assert_eq!(msg, "readyok");
/// ```
pub fn build_ready_ok_msg() -> String {
  String::from("readyok")
}
