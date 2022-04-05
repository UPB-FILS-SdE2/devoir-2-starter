/// Represents a redirect
#[derive(Debug)]
pub enum Redirect {
    /// > filename or >> filename
    Output { words: WordsPtr, append: bool },
    /// 2> filename or 2>> filename
    Error { words: WordsPtr, append: bool },
    /// < filename
    Input { words: WordsPtr },
    /// &> filename or &>> filename
    OutputAndError { words: WordsPtr, append: bool },
}

/// A word (part) of a command or a parameter
#[derive(Debug)]
pub enum Word {
    /// a string
    Word(String),
    /// a list of parameters that have to be concatenated to
    /// a single string
    Quotes(ParametersPtr),
    /// an env variable that has to be expended (replaced with its value)
    Expand(String),
    /// a command that has to be executed and replaced with its output
    /// (with the text that it wrote to stdout)
    Execute(CommandPtr),
    /// a redirect
    Redirect(RedirectPtr),
}

pub type WordPtr = Box<Word>;

/// Represents a command or a parameter
/// It is formed out of several words
/// that are concatenated before run
///
/// Ex:
/// ```
/// FILE=user
/// cat ${FILE}name
/// ```
/// this will become
/// ```
/// cat username
/// ```
pub type Words = Vec<WordPtr>;

/// A heap allocated command or parameter
pub type WordsPtr = Box<Words>;

pub type Assignments = Vec<AssignmentPtr>;
pub type AssignmentsPtr = Box<Assignments>;

pub type Parameters = Vec<WordsPtr>;
pub type ParametersPtr = Box<Parameters>;

pub type RedirectPtr = Box<Redirect>;

pub type Redirects = Vec<RedirectPtr>;
pub type RedirectsPtr = Box<Redirects>;

#[derive(Debug)]

pub struct Assignment {
    pub variable: String,
    pub values: WordsPtr,
}

pub type AssignmentPtr = Box<Assignment>;

/// Represents a command to be run
#[derive(Debug)]
pub enum Command {
    /// a simple command that has variable assignments, parameters and
    /// redirects. parameters\[0\] is the actual command.
    SimpleCommand {
        assignments: AssignmentsPtr,
        parameters: ParametersPtr,
        redirects: RedirectsPtr,
    },
    /// two commands connected with a pipe
    /// ```
    /// execute1 || execute2
    /// ```
    PipeCommand(CommandPtr, CommandPtr),
    /// two commands that a run sequentially (one after the other one)
    /// ```
    /// execute1 ; execute2
    /// ```
    SequentialCommand(CommandPtr, CommandPtr),
    /// two commands, where the second one is run only if the
    /// first one returned success (0)
    /// ```
    /// execute1 && execute2
    /// ```
    AndCommand(CommandPtr, CommandPtr),
    /// two commands, where the second one is run only if the
    /// first one returned failure (not 0)
    /// ```
    /// execute1 || execute2
    /// ```
    OrCommand(CommandPtr, CommandPtr),
}

pub type CommandPtr = Box<Command>;
