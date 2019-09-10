/// Tokens are the individual pieces that make up a shell command.
/// # Examples
/// ## Command
/// Commands consist of all the standard builtin bash programs. Examples of valid commands: `cd`, `pwd`, `echo`, `grep`, etc
pub enum Token {
    Operator(Operator),
    CommandOrArgument(String),
}

// pub enum CommandOrArgument {
//     Command(String),
//     Argument(String)
// }


pub enum Operator {
    Semicolon,
    Pipe,
    RedirectLeft,
    RedirectRight,
    Or,
    And,
    Background,
}