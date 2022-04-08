/// Represents a redirect
#[derive(Debug)]
pub enum Redirect {
    /// Redirect output (stdout) to a filename
    ///
    /// `> filename or >> filename`
    Output { words: WordsPtr, append: bool },
    /// Redirect error (stderr) to a filename
    ///
    /// `2> filename or 2>> filename`
    Error { words: WordsPtr, append: bool },
    /// Redirect input from (stdin) a filename
    ///
    /// `< filename`
    Input { words: WordsPtr },
    /// Redirect output (stdout) and error (stderr) to a filename
    ///
    /// `&> filename or &>> filename`
    OutputAndError { words: WordsPtr, append: bool },
}

/// Represents a word (part) of a command or a parameter.
///
/// Commands the formed out of one or more words. For example
/// command `ls` is composed out of a single word. The command
/// `ls${VARIABLE}` is composed out of two words:
///   - the word `ls`
///   - the variable VARIABLE
/// The resulting command is the concatentation of the `ls` word with the value
/// of the variable VARIABLE
///
/// ```rust
/// format!("ls{}", std::env::var("VARIABLE").unwrap())
/// ```
#[derive(Debug)]
pub enum Word {
    /// A word that is the actual String that the user typed
    ///
    /// ```bash
    /// $ top
    /// ```
    /// ```
    /// Word("top"),
    /// ```
    Word(String),
    /// A text that is formed out of several words. This is useful
    /// for commands and parameters that have spaces in between.
    ///
    /// ```bash
    /// $ "command  with a few    spaces"
    /// ```
    /// ```
    // Quotes(
    //     [
    //         Word(
    //             "command",
    //         ),
    //         Word(
    //             "  ",
    //         ),
    //         Word(
    //             "with",
    //         ),
    //         Word(
    //             " ",
    //         ),
    //         Word(
    //             "a",
    //         ),
    //         Word(
    //             " ",
    //         ),
    //         Word(
    //             "few",
    //         ),
    //         Word(
    //             "    ",
    //         ),
    //         Word(
    //             "spaces",
    //         ),
    //     ],
    // ),
    /// ```
    Quotes(WordsPtr),
    /// A word that has to be replaced with the value of an environment variable.
    /// Its value is computed by using the `std::env::var("...")`
    ///
    /// ```bash
    /// $ $CMD
    ///
    /// $ ${CMD}
    /// ```
    /// ```
    /// Expand("CMD"),
    /// ```
    Expand(String),
    /// A word that has to be replaced with the output (what was written to the
    /// screen - stdout) of a command.
    ///
    /// This usually mean executing the command and redirecting the output to
    /// a pipe, reading the pipe and then replacing the text.
    ///
    /// ```bash
    /// $ $(echo top)
    /// ```
    /// ```
    /// Execute(
    ///     SimpleCommand {
    ///         assignments: [],
    ///         parameters: [
    ///             [
    ///                 Word(
    ///                     "echo",
    ///                 ),
    ///             ],
    ///             [
    ///                 Word(
    ///                     "top",
    ///                 ),
    ///             ],
    ///         ],
    ///         redirects: [],
    ///     },
    /// ),
    /// ```
    Execute(CommandPtr),
    /// A word that is a redirect.
    ///
    /// Should not be used directly.
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

/// A heap allocated list of words
pub type WordsPtr = Box<Words>;

/// A list of assignments
pub type Assignments = Vec<AssignmentPtr>;
/// A heap allocated list of assignments
pub type AssignmentsPtr = Box<Assignments>;

/// A list of parameters
pub type Parameters = Vec<WordsPtr>;
/// A heap allocated list of parameters
pub type ParametersPtr = Box<Parameters>;

/// A heap allocated Redirect
pub type RedirectPtr = Box<Redirect>;

/// A list of heap allocated redirects
pub type Redirects = Vec<RedirectPtr>;
// A heap allocated list of heap allocated redirects
pub type RedirectsPtr = Box<Redirects>;

#[derive(Debug)]

/// Represents an enivronemnt variable assignment
pub struct Assignment {
    /// Variable name
    pub variable: String,
    /// A list of words that should be assigned to the variable
    pub values: WordsPtr,
}

/// A heap allocated Assignment
pub type AssignmentPtr = Box<Assignment>;

/// Represents a command to be run
#[derive(Debug)]
pub enum Command {
    /// A simple command that has variable assignments, parameters and
    /// redirects. parameters\[0\] is the actual command.
    SimpleCommand {
        /// Environment variables that have to be assigned to the command
        /// (to be used with `execvpe`)
        assignments: AssignmentsPtr,
        /// The command (first parameter) and the parameters
        parameters: ParametersPtr,
        /// The redirects
        redirects: RedirectsPtr,
    },
    /// Two commands connected with a pipe
    /// ```
    /// execute1 || execute2
    /// ```
    PipeCommand(CommandPtr, CommandPtr),
    /// Two commands that a run sequentially (one after the other one)
    /// ```
    /// execute1 ; execute2
    /// ```
    SequentialCommand(CommandPtr, CommandPtr),
    /// Two commands, where the second one is run only if the
    /// first one returned success (0)
    /// ```
    /// execute1 && execute2
    /// ```
    AndCommand(CommandPtr, CommandPtr),
    /// Two commands, where the second one is run only if the
    /// first one returned failure (not 0)
    /// ```
    /// execute1 || execute2
    /// ```
    OrCommand(CommandPtr, CommandPtr),
}

pub type CommandPtr = Box<Command>;
