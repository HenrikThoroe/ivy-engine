/// A representation of the supported subset of UCI commands.
///
/// Depending of the command type, a different provider can be used
/// to parse the command.
#[derive(Debug, PartialEq)]
pub enum CommandType {
  /// The `uci` command.
  ///
  /// Tells the engine to use the UCI potocol.
  /// Must be the first command sent to the engine.
  Uci,

  /// The `debug` command.
  ///
  /// Enables or disables debug mode
  /// with `debug on` or `debug off`.
  Debug,

  /// The `isready` command.
  ///
  /// Used to synchronize the engine with the driver.
  /// The engine must respond with `readyok` when it is ready to
  /// accept `go` commands.
  IsReady,

  /// The `setoption` command.
  ///
  /// Used to set engine options.
  /// An option is set with `setoption name <id> value <x>`
  /// or `setoption name <id>`.
  ///
  /// The set of supported options is defined by the engine.
  /// Unknown options are to be ignored.
  SetOption,

  /// The `ucinewgame` command.
  ///
  /// Tells the engine that a new game will begin.
  /// Must be sent after `isready` and before `position`.
  UciNewGame,

  /// The `position` command.
  ///
  /// Sets up the position described in the command.
  /// Must be sent after `ucinewgame` and before `go`.
  Position,

  /// The `go` command.
  ///
  /// Used to start calculating on the current position.
  /// The engine must respond with `bestmove` when it is ready to
  /// accept a new command.
  ///
  /// The `go` command can be followed by a number of parameters
  /// to specify how and for how long the engine should search.
  Go,

  /// The `stop` command.
  ///
  /// When the engine receives a `stop` command, it must stop
  /// calculating as soon as possible and return the best move
  /// it has found so far.
  Stop,

  /// The `quit` command.
  ///
  /// Tells the engine to quit as soon as possible.
  Quit,
}
