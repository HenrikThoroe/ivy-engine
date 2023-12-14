/// The type of an UCI option.
///
/// Depending on the type, the option may have a default value, a range of values, or a list of
/// possible values.
#[derive(Debug, Clone, PartialEq)]
pub enum OptionType {
  /// A boolean option, which accepts true or false.
  ///
  /// Can have a default value.
  Check,

  /// An integer option, which accepts a range of values.
  ///
  /// Can have a default, min and max value
  Spin,

  /// A string option, which accepts any string.
  ///
  /// Can have a default value and a list of possible values.
  Combo,

  /// A button option, which does not accept any value.
  Button,

  /// A string option, which accepts any string.
  ///
  /// Can have a default value.
  String,
}

/// An UCI option message.
///
/// Contains the id, type, default value, min and max values, and possible values of an UCI option.
///
/// # Examples
/// ```
/// use ivy_engine::uci::OptionMsg;
///
/// let spin = OptionMsg::new_spin("UCI_Elo".to_string(), "1350".to_string(), 1350, 2850);
/// let combo = OptionMsg::new_combo(
///   "UCI_Variant".to_string(),
///   "chess".to_string(),
///   vec!["chess".to_string(), "atomic".to_string()],
/// );
/// let button = OptionMsg::new_button("UCI_ShowCurrLine".to_string());
/// let string = OptionMsg::new_string("UCI_EngineAbout".to_string(), "Ivy 0.1.0".to_string());
/// let check = OptionMsg::new_check("UCI_AnalyseMode".to_string(), false);
/// ```
pub struct OptionMsg {
  /// The id of the option.
  pub id: String,

  /// The type of the option.
  pub option_type: OptionType,

  /// The default value of the option.
  pub default: String,

  /// The minimum value of the option.
  pub min: i64,

  /// The maximum value of the option.
  pub max: i64,

  /// The possible values of the option.
  pub var: Vec<String>,
}

impl OptionMsg {
  /// Constructs a new option message of type `Spin`.
  ///
  /// # Examples
  /// ```
  /// use ivy_engine::uci::OptionMsg;
  ///
  /// let option = OptionMsg::new_spin("UCI_Elo".to_string(), "1350".to_string(), 1350, 2850);
  /// ```
  pub fn new_spin(id: String, default: String, min: i64, max: i64) -> OptionMsg {
    OptionMsg {
      id,
      option_type: OptionType::Spin,
      default,
      min,
      max,
      var: Vec::new(),
    }
  }

  /// Constructs a new option message of type `Combo`.
  ///
  /// # Examples
  /// ```
  /// use ivy_engine::uci::OptionMsg;
  ///
  /// let option = OptionMsg::new_combo(
  ///   "UCI_Variant".to_string(),
  ///   "chess".to_string(),
  ///    vec!["chess".to_string(), "atomic".to_string()],
  /// );
  /// ```
  pub fn new_combo(id: String, default: String, var: Vec<String>) -> OptionMsg {
    OptionMsg {
      id,
      option_type: OptionType::Combo,
      default,
      min: 0,
      max: 0,
      var,
    }
  }

  /// Constructs a new option message of type `Button`.
  ///
  /// # Examples
  /// ```
  /// use ivy_engine::uci::OptionMsg;
  ///
  /// let option = OptionMsg::new_button("UCI_ShowCurrLine".to_string());
  /// ```
  pub fn new_button(id: String) -> OptionMsg {
    OptionMsg {
      id,
      option_type: OptionType::Button,
      default: String::new(),
      min: 0,
      max: 0,
      var: Vec::new(),
    }
  }

  /// Constructs a new option message of type `String`.
  ///
  /// # Examples
  /// ```
  /// use ivy_engine::uci::OptionMsg;
  ///
  /// let option = OptionMsg::new_string("UCI_EngineAbout".to_string(), "Ivy 0.1.0".to_string());
  /// ```
  pub fn new_string(id: String, default: String) -> OptionMsg {
    OptionMsg {
      id,
      option_type: OptionType::String,
      default,
      min: 0,
      max: 0,
      var: Vec::new(),
    }
  }

  /// Constructs a new option message of type `Check`.
  ///
  /// # Examples
  /// ```
  /// use ivy_engine::uci::OptionMsg;
  ///
  /// let option = OptionMsg::new_check("UCI_AnalyseMode".to_string(), false);
  /// ```
  pub fn new_check(id: String, default: bool) -> OptionMsg {
    OptionMsg {
      id,
      option_type: OptionType::Check,
      default: default.to_string(),
      min: 0,
      max: 0,
      var: Vec::new(),
    }
  }

  fn has_min_max(&self) -> bool {
    self.option_type == OptionType::Spin
  }

  fn has_var(&self) -> bool {
    self.option_type == OptionType::Combo
  }

  fn has_default(&self) -> bool {
    self.option_type != OptionType::Button
  }
}

/// Constructs an option message in the UCI format.
///
/// The returned string does not contain a trailing newline.
/// Use [OptionMsg] to construct the message data.
///
/// # Examples
/// ```
/// use ivy_engine::uci::{build_option_msg, OptionMsg};
///
/// let option = OptionMsg::new_check("UCI_AnalyseMode".to_string(), false);
/// let msg = build_option_msg(&option);
///
/// assert_eq!(msg, "option name UCI_AnalyseMode type check default false");
/// ```
pub fn build_option_msg(option: &OptionMsg) -> String {
  let mut msg = String::new();

  msg.push_str("option name ");
  msg.push_str(&option.id);
  msg.push_str(" type ");
  msg.push_str(match option.option_type {
    OptionType::Check => "check",
    OptionType::Spin => "spin",
    OptionType::Combo => "combo",
    OptionType::Button => "button",
    OptionType::String => "string",
  });

  if option.has_default() {
    msg.push_str(" default ");
    msg.push_str(&option.default);
  }

  if option.has_min_max() && option.min != option.max {
    msg.push_str(" min ");
    msg.push_str(&option.min.to_string());
    msg.push_str(" max ");
    msg.push_str(&option.max.to_string());
  }

  if option.has_var() && option.var.len() > 0 {
    msg.push_str(" var ");
    msg.push_str(&option.var.join(" "));
  }

  msg
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn build_check_msg() {
    let option = OptionMsg::new_check("UCI_AnalyseMode".to_string(), false);
    let expected = "option name UCI_AnalyseMode type check default false";

    assert_eq!(build_option_msg(&option), expected);
  }

  #[test]
  fn build_spin_msg() {
    let option = OptionMsg::new_spin("UCI_Elo".to_string(), "1350".to_string(), 1350, 2850);
    let expected = "option name UCI_Elo type spin default 1350 min 1350 max 2850";

    assert_eq!(build_option_msg(&option), expected);
  }

  #[test]
  fn build_combo_msg() {
    let option = OptionMsg::new_combo(
      "UCI_Variant".to_string(),
      "chess".to_string(),
      vec!["chess".to_string(), "atomic".to_string()],
    );

    let expected = "option name UCI_Variant type combo default chess var chess atomic";

    assert_eq!(build_option_msg(&option), expected);
  }

  #[test]
  fn build_button_msg() {
    let option = OptionMsg::new_button("UCI_ShowCurrLine".to_string());
    let expected = "option name UCI_ShowCurrLine type button";

    assert_eq!(build_option_msg(&option), expected);
  }

  #[test]
  fn build_string_msg() {
    let option = OptionMsg::new_string("UCI_EngineAbout".to_string(), "Ivy 0.1.0".to_string());
    let expected = "option name UCI_EngineAbout type string default Ivy 0.1.0";

    assert_eq!(build_option_msg(&option), expected);
  }
}
