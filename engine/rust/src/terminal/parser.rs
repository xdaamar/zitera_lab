//! Zitera Terminal Command Lexer and Security Parser
//!
//! Parses user input into typed command structures.
//! Strictly rejects shell injection attempts and unauthorized host commands.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalError {
    EmptyCommand,
    CommandNotAllowed(String),
    ShellInjectionDetected(String),
    UnmatchedQuotes,
    PathTraversalAttempt(String),
    FileNotFound(String),
    IsADirectory(String),
    NotADirectory(String),
    InvalidArguments(String),
    ExecutionError(String),
}

impl std::fmt::Display for TerminalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerminalError::EmptyCommand => write!(f, ""),
            TerminalError::CommandNotAllowed(cmd) => {
                write!(f, "{}: command not found (not in Zitera allowlist)", cmd)
            }
            TerminalError::ShellInjectionDetected(token) => {
                write!(
                    f,
                    "syntax error: shell operators ('{}') are not permitted in Zitera Terminal",
                    token
                )
            }
            TerminalError::UnmatchedQuotes => {
                write!(f, "syntax error: unmatched quotes in command line")
            }
            TerminalError::PathTraversalAttempt(p) => {
                write!(
                    f,
                    "access denied: path '{}' escapes terminal sandbox boundary",
                    p
                )
            }
            TerminalError::FileNotFound(p) => write!(f, "cat: {}: No such file or directory", p),
            TerminalError::IsADirectory(p) => write!(f, "cat: {}: Is a directory", p),
            TerminalError::NotADirectory(p) => write!(f, "cd: {}: Not a directory", p),
            TerminalError::InvalidArguments(msg) => write!(f, "invalid arguments: {}", msg),
            TerminalError::ExecutionError(msg) => write!(f, "error: {}", msg),
        }
    }
}

impl std::error::Error for TerminalError {}

/// Static list of permitted built-in terminal commands.
pub const ALLOWED_COMMANDS: &[&str] = &[
    "pwd", "ls", "cd", "cat", "head", "tail", "grep", "find", "echo", "mkdir", "touch", "cp", "mv",
    "rm", "clear", "help", "whoami", "uname", "curl", "nmap", "ps",
];

/// Prohibited host system shells and tools.
pub const FORBIDDEN_HOST_COMMANDS: &[&str] = &[
    "cmd",
    "powershell",
    "pwsh",
    "bash",
    "sh",
    "wsl",
    "start",
    "runas",
    "wmic",
    "reg",
    "diskpart",
    "net",
    "sc",
    "taskkill",
    "certutil",
    "bitsadmin",
    "vssadmin",
];

/// Prohibited shell meta-characters and injection patterns.
pub const SHELL_INJECTION_TOKENS: &[&str] = &[
    ";", "&&", "||", "|", "&", ">", ">>", "<", "<<", "`", "$(", "${",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedCommand {
    pub name: String,
    pub args: Vec<String>,
}

/// Tokenizes a raw terminal input string into an argument list.
/// Handles single and double quotes without allowing shell escapes.
pub fn tokenize(input: &str) -> Result<Vec<String>, TerminalError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    // Pre-check for shell injection tokens
    for token in SHELL_INJECTION_TOKENS {
        if trimmed.contains(token) {
            return Err(TerminalError::ShellInjectionDetected(token.to_string()));
        }
    }

    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;

    for ch in trimmed.chars() {
        if ch == '\'' && !in_double_quote {
            in_single_quote = !in_single_quote;
            continue;
        }

        if ch == '"' && !in_single_quote {
            in_double_quote = !in_double_quote;
            continue;
        }

        if (ch == ' ' || ch == '\t') && !in_single_quote && !in_double_quote {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
        } else {
            current.push(ch);
        }
    }

    if in_single_quote || in_double_quote {
        return Err(TerminalError::UnmatchedQuotes);
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens)
}

/// Parses and validates a command against security allowlists.
pub fn parse_command(input: &str) -> Result<ParsedCommand, TerminalError> {
    let tokens = tokenize(input)?;
    if tokens.is_empty() {
        return Err(TerminalError::EmptyCommand);
    }

    let raw_name = &tokens[0];
    let cmd_name = raw_name.to_lowercase();

    // Check forbidden host commands explicitly
    if FORBIDDEN_HOST_COMMANDS.contains(&cmd_name.as_str()) {
        return Err(TerminalError::CommandNotAllowed(raw_name.clone()));
    }

    // Check allowlist
    if !ALLOWED_COMMANDS.contains(&cmd_name.as_str()) {
        return Err(TerminalError::CommandNotAllowed(raw_name.clone()));
    }

    Ok(ParsedCommand {
        name: cmd_name,
        args: tokens[1..].to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let tokens = tokenize("ls -la /var/log").unwrap();
        assert_eq!(tokens, vec!["ls", "-la", "/var/log"]);
    }

    #[test]
    fn test_tokenize_quotes() {
        let tokens = tokenize("echo 'hello world' \"zitera lab\"").unwrap();
        assert_eq!(tokens, vec!["echo", "hello world", "zitera lab"]);
    }

    #[test]
    fn test_tokenize_unmatched_quotes() {
        assert!(matches!(
            tokenize("echo 'unclosed"),
            Err(TerminalError::UnmatchedQuotes)
        ));
        assert!(matches!(
            tokenize("echo \"unclosed"),
            Err(TerminalError::UnmatchedQuotes)
        ));
    }

    #[test]
    fn test_shell_injection_rejection() {
        let injections = [
            "ls; whoami",
            "cat flag.txt | grep ZITERA",
            "cat flag.txt && rm -rf /",
            "echo `id`",
            "echo $(whoami)",
            "cat foo > bar",
            "cat foo >> bar",
            "ls & cmd",
        ];

        for inj in injections {
            let res = parse_command(inj);
            assert!(
                matches!(res, Err(TerminalError::ShellInjectionDetected(_))),
                "Failed to reject injection: {}",
                inj
            );
        }
    }

    #[test]
    fn test_forbidden_commands_rejected() {
        let forbidden = [
            "cmd",
            "cmd.exe",
            "powershell",
            "pwsh",
            "wmic",
            "reg",
            "runas",
            "wsl",
        ];
        for cmd in forbidden {
            let res = parse_command(cmd);
            assert!(
                matches!(res, Err(TerminalError::CommandNotAllowed(_))),
                "Failed to reject forbidden command: {}",
                cmd
            );
        }
    }

    #[test]
    fn test_allowed_commands_parsed() {
        for cmd in ALLOWED_COMMANDS {
            let res = parse_command(cmd);
            assert!(res.is_ok(), "Allowed command should parse: {}", cmd);
            assert_eq!(res.unwrap().name, cmd.to_lowercase());
        }
    }
}
