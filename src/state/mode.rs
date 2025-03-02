use edit::insert::InsertOptions;

#[derive(Clone, PartialEq, Eq, Copy)]
/// A mode. Modes determine which set of commands that will be used. Modes comes in two flavors:
pub enum Mode {
    /// A primitive mode. In this mode type, absolutely none preprocessing of the commands are
    /// done. Therefore the instruction will just be a command, without any form of numeral
    /// parameter. This is useful for modes such as insert, where commands don't take numeral
    /// parameters.
    Primitive(PrimitiveMode),
    /// Command mode. In this mode type input is collected into instructions, which are commands
    /// having a numeral parameter. This numeral parameter is useful for a number of things, such
    /// as repeation, line number, etc.
    Command(CommandMode),
}

impl Mode {
    /// Convert the mode to string
    pub fn to_string(self) -> &'static str {
        use self::CommandMode::*;
        use self::Mode::*;
        use self::PrimitiveMode::*;
        match self {
            Command(Normal) => "Normal",
            Primitive(Insert(_)) => "Insert",
            Primitive(Prompt) => "Prompt",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Copy)]
/// A command mode
pub enum CommandMode {
    // Visual(VisualOptions),
    /// Normal mode. The default mode, which can be used for most common commands and switching to
    /// other modes.
    Normal,
}

#[derive(Clone, PartialEq, Eq, Copy)]
/// A primitive mode
pub enum PrimitiveMode {
    /// Insert mode. In this text is inserted
    Insert(InsertOptions),
    /// Prompt. In the prompt the user can give the editor commands, which often are more
    /// "sentence-like", i.e. they're not like commands in normal mode for example. These commands
    /// can be used for a number of things, such as configurating Sodium, or enabling/disabling
    /// options.
    Prompt,
}
