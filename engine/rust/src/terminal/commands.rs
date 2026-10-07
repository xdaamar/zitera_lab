//! Zitera Terminal Built-in Commands Implementation
//!
//! Provides self-contained, memory-safe implementations of Unix utilities
//! operating strictly within the logical sandboxed filesystem.

use super::fs::TerminalFilesystem;
use std::fs;
use std::path::Path;

pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl CommandOutput {
    pub fn ok(stdout: impl Into<String>) -> Self {
        Self {
            stdout: stdout.into(),
            stderr: String::new(),
            exit_code: 0,
        }
    }

    pub fn err(stderr: impl Into<String>, code: i32) -> Self {
        Self {
            stdout: String::new(),
            stderr: stderr.into(),
            exit_code: code,
        }
    }
}

pub fn execute_builtin(name: &str, args: &[String], vfs: &mut TerminalFilesystem) -> CommandOutput {
    match name {
        "pwd" => cmd_pwd(vfs),
        "ls" => cmd_ls(args, vfs),
        "cd" => cmd_cd(args, vfs),
        "cat" => cmd_cat(args, vfs),
        "head" => cmd_head(args, vfs),
        "tail" => cmd_tail(args, vfs),
        "grep" => cmd_grep(args, vfs),
        "find" => cmd_find(args, vfs),
        "echo" => cmd_echo(args),
        "mkdir" => cmd_mkdir(args, vfs),
        "touch" => cmd_touch(args, vfs),
        "cp" => cmd_cp(args, vfs),
        "mv" => cmd_mv(args, vfs),
        "rm" => cmd_rm(args, vfs),
        "ps" => cmd_ps(args),
        "clear" => CommandOutput::ok("\x1b[2J\x1b[H"),
        "whoami" => CommandOutput::ok("learner\n"),
        "uname" => cmd_uname(args),
        "help" => cmd_help(args),
        other => CommandOutput::err(format!("{}: command not found\n", other), 127),
    }
}

fn cmd_pwd(vfs: &TerminalFilesystem) -> CommandOutput {
    CommandOutput::ok(format!("{}\n", vfs.current_virtual_dir()))
}

fn cmd_cd(args: &[String], vfs: &mut TerminalFilesystem) -> CommandOutput {
    let target = if args.is_empty() { "/" } else { &args[0] };
    match vfs.change_dir(target) {
        Ok(_) => CommandOutput::ok(""),
        Err(e) => CommandOutput::err(format!("cd: {}\n", e), 1),
    }
}

fn cmd_ls(args: &[String], vfs: &TerminalFilesystem) -> CommandOutput {
    let mut show_all = false;
    let mut long_format = false;
    let mut target_dir = ".";

    for arg in args {
        if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg.chars().skip(1) {
                if ch == 'a' {
                    show_all = true;
                } else if ch == 'l' {
                    long_format = true;
                }
            }
        } else {
            target_dir = arg;
        }
    }

    let physical = match vfs.resolve_path(target_dir) {
        Ok(p) => p,
        Err(e) => return CommandOutput::err(format!("ls: {}\n", e), 1),
    };

    if !physical.exists() {
        return CommandOutput::err(
            format!(
                "ls: cannot access '{}': No such file or directory\n",
                target_dir
            ),
            2,
        );
    }

    if physical.is_file() {
        return CommandOutput::ok(format!("{}\n", target_dir));
    }

    let entries = match fs::read_dir(&physical) {
        Ok(e) => e,
        Err(err) => {
            return CommandOutput::err(format!("ls: error reading directory: {}\n", err), 1)
        }
    };

    let mut names = Vec::new();
    if show_all {
        names.push(".".to_string());
        names.push("..".to_string());
    }

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if !show_all && name.starts_with('.') {
            continue;
        }
        names.push(name);
    }
    names.sort();

    if long_format {
        let mut out = String::new();
        for name in &names {
            if name == "." || name == ".." {
                out.push_str(&format!("drwxr-xr-x 2 learner zitera 4096 {}\n", name));
                continue;
            }
            let child = physical.join(name);
            let is_dir = child.is_dir();
            let size = fs::metadata(&child).map(|m| m.len()).unwrap_or(0);
            let perm = if is_dir { "drwxr-xr-x" } else { "-rw-r--r--" };
            out.push_str(&format!("{} 1 learner zitera {:>6} {}\n", perm, size, name));
        }
        CommandOutput::ok(out)
    } else {
        CommandOutput::ok(format!("{}\n", names.join("  ")))
    }
}

fn cmd_cat(args: &[String], vfs: &TerminalFilesystem) -> CommandOutput {
    if args.is_empty() {
        return CommandOutput::err("cat: missing operand\n", 1);
    }

    let mut combined = String::new();
    for path_arg in args {
        match vfs.read_file(path_arg) {
            Ok(content) => combined.push_str(&content),
            Err(e) => return CommandOutput::err(format!("{}\n", e), 1),
        }
    }

    if !combined.ends_with('\n') && !combined.is_empty() {
        combined.push('\n');
    }
    CommandOutput::ok(combined)
}

fn cmd_head(args: &[String], vfs: &TerminalFilesystem) -> CommandOutput {
    let mut num_lines = 10;
    let mut file_idx = 0;

    if args.len() >= 2 && args[0] == "-n" {
        if let Ok(n) = args[1].parse::<usize>() {
            num_lines = n;
            file_idx = 2;
        }
    }

    if file_idx >= args.len() {
        return CommandOutput::err("head: missing file operand\n", 1);
    }

    let content = match vfs.read_file(&args[file_idx]) {
        Ok(c) => c,
        Err(e) => return CommandOutput::err(format!("head: {}\n", e), 1),
    };

    let lines: Vec<&str> = content.lines().take(num_lines).collect();
    let mut out = lines.join("\n");
    if !out.is_empty() {
        out.push('\n');
    }
    CommandOutput::ok(out)
}

fn cmd_tail(args: &[String], vfs: &TerminalFilesystem) -> CommandOutput {
    let mut num_lines = 10;
    let mut file_idx = 0;

    if args.len() >= 2 && args[0] == "-n" {
        if let Ok(n) = args[1].parse::<usize>() {
            num_lines = n;
            file_idx = 2;
        }
    }

    if file_idx >= args.len() {
        return CommandOutput::err("tail: missing file operand\n", 1);
    }

    let content = match vfs.read_file(&args[file_idx]) {
        Ok(c) => c,
        Err(e) => return CommandOutput::err(format!("tail: {}\n", e), 1),
    };

    let all_lines: Vec<&str> = content.lines().collect();
    let start_idx = all_lines.len().saturating_sub(num_lines);
    let selected = &all_lines[start_idx..];
    let mut out = selected.join("\n");
    if !out.is_empty() {
        out.push('\n');
    }
    CommandOutput::ok(out)
}

fn cmd_grep(args: &[String], vfs: &TerminalFilesystem) -> CommandOutput {
    let mut ignore_case = false;
    let mut line_numbers = false;
    let mut invert = false;
    let mut pattern_opt = None;
    let mut file_opt = None;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg.starts_with('-') && arg.len() > 1 && !arg.starts_with("--") {
            for ch in arg.chars().skip(1) {
                if ch == 'i' {
                    ignore_case = true;
                } else if ch == 'n' {
                    line_numbers = true;
                } else if ch == 'v' {
                    invert = true;
                }
            }
        } else if pattern_opt.is_none() {
            pattern_opt = Some(arg.as_str());
        } else if file_opt.is_none() {
            file_opt = Some(arg.as_str());
        }
        i += 1;
    }

    let pattern = match pattern_opt {
        Some(p) => p,
        None => return CommandOutput::err("grep: missing pattern\n", 2),
    };

    let file_path = match file_opt {
        Some(f) => f,
        None => return CommandOutput::err("grep: missing file operand\n", 2),
    };

    let content = match vfs.read_file(file_path) {
        Ok(c) => c,
        Err(e) => return CommandOutput::err(format!("grep: {}\n", e), 2),
    };

    let lower_pattern = pattern.to_lowercase();
    let mut matches = Vec::new();

    for (idx, line) in content.lines().enumerate() {
        let matched = if ignore_case {
            line.to_lowercase().contains(&lower_pattern)
        } else {
            line.contains(pattern)
        };

        if matched ^ invert {
            if line_numbers {
                matches.push(format!("{}:{}", idx + 1, line));
            } else {
                matches.push(line.to_string());
            }
        }
    }

    let code = if matches.is_empty() { 1 } else { 0 };
    let mut out = matches.join("\n");
    if !out.is_empty() {
        out.push('\n');
    }
    CommandOutput {
        stdout: out,
        stderr: String::new(),
        exit_code: code,
    }
}

fn cmd_find(args: &[String], vfs: &TerminalFilesystem) -> CommandOutput {
    let mut search_dir = ".";
    let mut name_pattern: Option<&str> = None;

    let mut i = 0;
    while i < args.len() {
        if args[i] == "-name" && i + 1 < args.len() {
            name_pattern = Some(&args[i + 1]);
            i += 2;
        } else {
            search_dir = &args[i];
            i += 1;
        }
    }

    let physical = match vfs.resolve_path(search_dir) {
        Ok(p) => p,
        Err(e) => return CommandOutput::err(format!("find: {}\n", e), 1),
    };

    if !physical.exists() {
        return CommandOutput::err(
            format!("find: '{}': No such file or directory\n", search_dir),
            1,
        );
    }

    let mut found_list = Vec::new();
    collect_find_paths(&physical, search_dir, name_pattern, &mut found_list);
    found_list.sort();

    let mut out = found_list.join("\n");
    if !out.is_empty() {
        out.push('\n');
    }
    CommandOutput::ok(out)
}

fn collect_find_paths(
    current: &Path,
    prefix: &str,
    name_pattern: Option<&str>,
    results: &mut Vec<String>,
) {
    let clean_pattern = name_pattern.map(|p| p.trim_matches('*'));

    if let Ok(entries) = fs::read_dir(current) {
        for entry in entries.flatten() {
            let child_path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();
            let display_path = if prefix == "." {
                format!("./{}", file_name)
            } else if prefix.ends_with('/') {
                format!("{}{}", prefix, file_name)
            } else {
                format!("{}/{}", prefix, file_name)
            };

            let matches = match clean_pattern {
                Some(pat) => file_name.contains(pat),
                None => true,
            };

            if matches {
                results.push(display_path.clone());
            }

            if child_path.is_dir() {
                collect_find_paths(&child_path, &display_path, name_pattern, results);
            }
        }
    }
}

fn cmd_echo(args: &[String]) -> CommandOutput {
    let mut no_newline = false;
    let mut start_idx = 0;

    if !args.is_empty() && args[0] == "-n" {
        no_newline = true;
        start_idx = 1;
    }

    let text = args[start_idx..].join(" ");
    let stdout = if no_newline {
        text
    } else {
        format!("{}\n", text)
    };
    CommandOutput::ok(stdout)
}

fn cmd_mkdir(args: &[String], vfs: &mut TerminalFilesystem) -> CommandOutput {
    if args.is_empty() {
        return CommandOutput::err("mkdir: missing operand\n", 1);
    }

    let target = if args[0] == "-p" && args.len() > 1 {
        &args[1]
    } else {
        &args[0]
    };

    match vfs.make_dir(target) {
        Ok(_) => CommandOutput::ok(""),
        Err(e) => CommandOutput::err(format!("mkdir: {}\n", e), 1),
    }
}

fn cmd_touch(args: &[String], vfs: &mut TerminalFilesystem) -> CommandOutput {
    if args.is_empty() {
        return CommandOutput::err("touch: missing file operand\n", 1);
    }

    for arg in args {
        match vfs.write_file(arg, "") {
            Ok(_) => {}
            Err(e) => return CommandOutput::err(format!("touch: {}\n", e), 1),
        }
    }
    CommandOutput::ok("")
}

fn cmd_cp(args: &[String], vfs: &TerminalFilesystem) -> CommandOutput {
    if args.len() < 2 {
        return CommandOutput::err("cp: missing file operand\n", 1);
    }

    let src = match vfs.resolve_path(&args[0]) {
        Ok(p) => p,
        Err(e) => return CommandOutput::err(format!("cp: {}\n", e), 1),
    };
    let dst = match vfs.resolve_path(&args[1]) {
        Ok(p) => p,
        Err(e) => return CommandOutput::err(format!("cp: {}\n", e), 1),
    };

    if !src.exists() {
        return CommandOutput::err(
            format!("cp: cannot stat '{}': No such file or directory\n", args[0]),
            1,
        );
    }

    let target_file = if dst.is_dir() {
        dst.join(src.file_name().unwrap_or_default())
    } else {
        dst
    };

    match fs::copy(&src, &target_file) {
        Ok(_) => CommandOutput::ok(""),
        Err(e) => CommandOutput::err(format!("cp: error copying: {}\n", e), 1),
    }
}

fn cmd_mv(args: &[String], vfs: &TerminalFilesystem) -> CommandOutput {
    if args.len() < 2 {
        return CommandOutput::err("mv: missing file operand\n", 1);
    }

    let src = match vfs.resolve_path(&args[0]) {
        Ok(p) => p,
        Err(e) => return CommandOutput::err(format!("mv: {}\n", e), 1),
    };
    let dst = match vfs.resolve_path(&args[1]) {
        Ok(p) => p,
        Err(e) => return CommandOutput::err(format!("mv: {}\n", e), 1),
    };

    if !src.exists() {
        return CommandOutput::err(
            format!("mv: cannot stat '{}': No such file or directory\n", args[0]),
            1,
        );
    }

    let target_file = if dst.is_dir() {
        dst.join(src.file_name().unwrap_or_default())
    } else {
        dst
    };

    match fs::rename(&src, &target_file) {
        Ok(_) => CommandOutput::ok(""),
        Err(e) => CommandOutput::err(format!("mv: error moving: {}\n", e), 1),
    }
}

fn cmd_rm(args: &[String], vfs: &TerminalFilesystem) -> CommandOutput {
    if args.is_empty() {
        return CommandOutput::err("rm: missing operand\n", 1);
    }

    let mut recursive = false;
    let mut targets = Vec::new();

    for arg in args {
        if arg == "-r" || arg == "-rf" || arg == "-R" {
            recursive = true;
        } else if !arg.starts_with('-') {
            targets.push(arg.as_str());
        }
    }

    if targets.is_empty() {
        return CommandOutput::err("rm: missing operand\n", 1);
    }

    for target in targets {
        if let Err(e) = vfs.remove(target, recursive) {
            return CommandOutput::err(format!("rm: {}\n", e), 1);
        }
    }

    CommandOutput::ok("")
}

fn cmd_uname(args: &[String]) -> CommandOutput {
    if args.iter().any(|a| a == "-a") {
        CommandOutput::ok(
            "Linux zitera-lab 6.1.0-21-amd64 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux\n",
        )
    } else {
        CommandOutput::ok("Linux\n")
    }
}

fn cmd_ps(args: &[String]) -> CommandOutput {
    let show_extended = args
        .iter()
        .any(|a| a == "-ef" || a == "aux" || a == "-aux" || a == "-a" || a == "-l");
    if show_extended {
        let text = "UID        PID  PPID  C STIME TTY          TIME CMD\n\
                    root         1     0  0 00:00 ?        00:00:01 /sbin/init\n\
                    learner     42     1  0 00:00 ?        00:00:00 /opt/zitera/lab-daemon\n\
                    learner     88    42  0 00:00 pts/0    00:00:00 /bin/zitera-term\n";
        CommandOutput::ok(text)
    } else {
        let text = "  PID TTY          TIME CMD\n\
                      1 ?        00:00:01 init\n\
                     42 ?        00:00:00 lab-daemon\n\
                     88 pts/0    00:00:00 zitera-term\n";
        CommandOutput::ok(text)
    }
}

fn cmd_help(args: &[String]) -> CommandOutput {
    if args.is_empty() {
        let help_text = r#"Zitera Educational Terminal v2.0 (OWASP:2025 Edition)
All commands execute strictly in an isolated in-memory Virtual Filesystem (VFS).
Zero host process passthrough, zero raw shell injection risk.

Standard Commands:
  pwd              Print current working directory
  ls [-l] [-a]     List directory contents and attributes
  cd <dir>         Change logical working directory
  cat <file...>    Concatenate and display file content
  head [-n N] file Display first N lines of file
  tail [-n N] file Display last N lines of file
  grep [-i] [-n]   Search for regex or text pattern in files
  find [path]      Recursively search for files in directory tree
  echo [text...]   Display line of text
  mkdir [-p] <dir> Create directory
  touch <file...>  Create or touch file
  cp <src> <dst>   Copy file
  mv <src> <dst>   Move or rename file
  rm [-r] <file>   Remove file or directory
  ps [-ef]         Report active simulated processes
  curl [options]   Simulated / safe HTTP client for testing lab endpoints
  clear            Clear terminal screen
  whoami           Display current user identity
  uname [-a]       Print system and kernel architecture
  help [cmd]       Display detailed educational usage for a command

Tip: Run 'help <command>' (e.g., 'help grep', 'help ls', 'help curl') for cybersecurity examples.
"#;
        return CommandOutput::ok(help_text);
    }

    let target = args[0].trim().to_lowercase();
    let detail = match target.as_str() {
        "ls" => r#"COMMAND: ls
USAGE: ls [-l] [-a] [path]
DESCRIPTION:
  List directory contents. In cybersecurity analysis, 'ls' is fundamental for
  enumerating file permissions, hidden configuration files, and directory layouts.
FLAGS:
  -l   Long listing format (shows file sizes, types, and permissions)
  -a   Show hidden files (files beginning with '.')
SECURITY CONTEXT:
  Always inspect hidden files (-a) like .env, .git, or .bash_history during CTFs.
"#,
        "grep" => r#"COMMAND: grep
USAGE: grep [-i] [-n] <pattern> [file]
DESCRIPTION:
  Search for PATTERN in each FILE or standard input.
FLAGS:
  -i   Case-insensitive match
  -n   Prefix each line of output with its 1-based line number
SECURITY CONTEXT:
  Indispensable for log analysis, discovering exposed hardcoded secrets, API tokens,
  and tracing error stack dumps across application files.
"#,
        "find" => r#"COMMAND: find
USAGE: find [path] [-name <pattern>]
DESCRIPTION:
  Recursively traverses the directory hierarchy to search for files matching patterns.
SECURITY CONTEXT:
  Used to discover vulnerable SUID binaries, misconfigured world-writable files,
  or misplaced backup files (e.g. *.bak, *.sql, *.conf).
"#,
        "cat" => r#"COMMAND: cat
USAGE: cat <file...>
DESCRIPTION:
  Concatenate FILE(s) to standard output.
SECURITY CONTEXT:
  Frequently used in Local File Inclusion (LFI) and Broken Access Control challenges
  to read sensitive files such as /etc/passwd or configuration files.
"#,
        "curl" => r#"COMMAND: curl
USAGE: curl [-i] [-X METHOD] [url]
DESCRIPTION:
  Transfer data from or to a server using supported application protocols (HTTP/HTTPS).
FLAGS:
  -i   Include HTTP response headers in the output
  -X   Specify custom request method (GET, POST, PUT, DELETE)
  -d   HTTP POST data payload
SECURITY CONTEXT:
  The premier CLI tool for API pentesting, verifying CORS headers, testing JWT tokens,
  and executing HTTP requests against local lab servers.
"#,
        "ps" => r#"COMMAND: ps
USAGE: ps [-ef]
DESCRIPTION:
  Report a snapshot of the current processes running within the lab sandbox.
FLAGS:
  -ef, aux   Display full-format listing of all running processes and parent PIDs
SECURITY CONTEXT:
  Key for privilege escalation reconnaissance, detecting background daemon credentials,
  and verifying isolated service states.
"#,
        "head" => r#"COMMAND: head
USAGE: head [-n lines] [file]
DESCRIPTION:
  Output the first N lines (default 10) of specified file.
"#,
        "tail" => r#"COMMAND: tail
USAGE: tail [-n lines] [file]
DESCRIPTION:
  Output the last N lines (default 10) of specified file. Great for live log inspection.
"#,
        "cd" => r#"COMMAND: cd
USAGE: cd [dir]
DESCRIPTION:
  Change the current working directory inside the sandboxed Virtual Filesystem.
"#,
        "pwd" => r#"COMMAND: pwd
USAGE: pwd
DESCRIPTION:
  Print the full pathname of the current working directory.
"#,
        "mkdir" => r#"COMMAND: mkdir
USAGE: mkdir [-p] <dir>
DESCRIPTION:
  Create the DIRECTORY(ies), if they do not already exist.
"#,
        "touch" => r#"COMMAND: touch
USAGE: touch <file...>
DESCRIPTION:
  Update file timestamp or create empty file if it does not exist.
"#,
        "cp" => r#"COMMAND: cp
USAGE: cp <source> <destination>
DESCRIPTION:
  Copy SOURCE to DEST.
"#,
        "mv" => r#"COMMAND: mv
USAGE: mv <source> <destination>
DESCRIPTION:
  Rename SOURCE to DEST, or move SOURCE to DIRECTORY.
"#,
        "rm" => r#"COMMAND: rm
USAGE: rm [-r] <file...>
DESCRIPTION:
  Remove (unlink) the FILE(s). Use -r for recursive directory removal.
"#,
        "whoami" => r#"COMMAND: whoami
USAGE: whoami
DESCRIPTION:
  Print the effective user ID of the current terminal session (defaults to 'learner').
"#,
        "uname" => r#"COMMAND: uname
USAGE: uname [-a]
DESCRIPTION:
  Print system and kernel architecture information.
"#,
        "clear" => r#"COMMAND: clear
USAGE: clear
DESCRIPTION:
  Clear the terminal screen buffer.
"#,
        other => {
            return CommandOutput::err(
                format!(
                    "help: no educational entry found for '{}'. Type 'help' for command list.\n",
                    other
                ),
                1,
            );
        }
    };

    CommandOutput::ok(detail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo() {
        let out = cmd_echo(&["hello".to_string(), "world".to_string()]);
        assert_eq!(out.stdout, "hello world\n");
        assert_eq!(out.exit_code, 0);
    }

    #[test]
    fn test_file_commands() {
        let temp = std::env::temp_dir().join("zitera_test_commands");
        let _ = fs::create_dir_all(&temp);
        let mut vfs = TerminalFilesystem::new(temp.clone());

        // touch & cat
        let touch_out = execute_builtin("touch", &["file.txt".to_string()], &mut vfs);
        assert_eq!(touch_out.exit_code, 0);

        vfs.write_file("file.txt", "line1\nline2\nline3\nline4\nline5\n")
            .unwrap();
        let cat_out = execute_builtin("cat", &["file.txt".to_string()], &mut vfs);
        assert_eq!(cat_out.exit_code, 0);
        assert!(cat_out.stdout.contains("line1"));

        // head
        let head_out = execute_builtin(
            "head",
            &["-n".to_string(), "2".to_string(), "file.txt".to_string()],
            &mut vfs,
        );
        assert_eq!(head_out.exit_code, 0);
        assert_eq!(head_out.stdout, "line1\nline2\n");

        // tail
        let tail_out = execute_builtin(
            "tail",
            &["-n".to_string(), "2".to_string(), "file.txt".to_string()],
            &mut vfs,
        );
        assert_eq!(tail_out.exit_code, 0);
        assert_eq!(tail_out.stdout, "line4\nline5\n");

        // grep
        let grep_out = execute_builtin(
            "grep",
            &["line3".to_string(), "file.txt".to_string()],
            &mut vfs,
        );
        assert_eq!(grep_out.exit_code, 0);
        assert_eq!(grep_out.stdout, "line3\n");

        // cp & ls
        let cp_out = execute_builtin(
            "cp",
            &["file.txt".to_string(), "copy.txt".to_string()],
            &mut vfs,
        );
        assert_eq!(cp_out.exit_code, 0);

        let ls_out = execute_builtin("ls", &[], &mut vfs);
        assert_eq!(ls_out.exit_code, 0);
        assert!(ls_out.stdout.contains("file.txt"));
        assert!(ls_out.stdout.contains("copy.txt"));

        // rm
        let rm_out = execute_builtin("rm", &["copy.txt".to_string()], &mut vfs);
        assert_eq!(rm_out.exit_code, 0);

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_ps_command() {
        let mut temp_vfs = TerminalFilesystem::new(std::env::temp_dir());
        let out_std = execute_builtin("ps", &[], &mut temp_vfs);
        assert_eq!(out_std.exit_code, 0);
        assert!(out_std.stdout.contains("init"));
        assert!(out_std.stdout.contains("zitera-term"));

        let out_ef = execute_builtin("ps", &["-ef".to_string()], &mut temp_vfs);
        assert_eq!(out_ef.exit_code, 0);
        assert!(out_ef.stdout.contains("UID"));
        assert!(out_ef.stdout.contains("root"));
        assert!(out_ef.stdout.contains("learner"));
    }

    #[test]
    fn test_help_command() {
        let mut temp_vfs = TerminalFilesystem::new(std::env::temp_dir());
        // General help
        let out_gen = execute_builtin("help", &[], &mut temp_vfs);
        assert_eq!(out_gen.exit_code, 0);
        assert!(out_gen.stdout.contains("Zitera Educational Terminal"));
        assert!(out_gen.stdout.contains("grep"));

        // Specific command help
        let out_grep = execute_builtin("help", &["grep".to_string()], &mut temp_vfs);
        assert_eq!(out_grep.exit_code, 0);
        assert!(out_grep.stdout.contains("COMMAND: grep"));
        assert!(out_grep.stdout.contains("SECURITY CONTEXT"));

        // Curl command help
        let out_curl = execute_builtin("help", &["curl".to_string()], &mut temp_vfs);
        assert_eq!(out_curl.exit_code, 0);
        assert!(out_curl.stdout.contains("COMMAND: curl"));

        // Unknown command help
        let out_unknown = execute_builtin("help", &["nonexistent".to_string()], &mut temp_vfs);
        assert_eq!(out_unknown.exit_code, 1);
        assert!(out_unknown.stderr.contains("no educational entry found"));
    }
}
