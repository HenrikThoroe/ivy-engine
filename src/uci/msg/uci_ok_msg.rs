/// Constructs a UCI OK message.
///
/// The returned string does not contain a trailing newline.
///
/// # Examples
/// ```
/// use ivy_engine::uci::build_uci_ok_msg;
///
/// let msg = build_uci_ok_msg();
/// assert_eq!(msg, "uciok");
/// ```
pub fn build_uci_ok_msg() -> String {
  String::from("uciok")
}
