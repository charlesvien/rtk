use crate::git::compact_diff;
use crate::tracking;
use crate::utils::truncate;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::ffi::OsString;
use std::process::Command;

lazy_static! {
    static ref EMAIL_RE: Regex = Regex::new(r"\S+@\S+\.\S+").unwrap();
}

pub fn run_status(args: &[String], verbose: u8) -> Result<()> {
    let timer = tracking::TimedExecution::start();

    let mut cmd = Command::new("jj");
    cmd.args(["status", "--no-pager"]);
    for arg in args {
        cmd.arg(arg);
    }

    if verbose > 0 {
        eprintln!("Running: jj status --no-pager {}", args.join(" "));
    }

    let output = cmd
        .output()
        .context("Failed to run jj status. Is jj installed?")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let raw = format!("{}\n{}", stdout, stderr);

    let exit_code = output
        .status
        .code()
        .unwrap_or(if output.status.success() { 0 } else { 1 });

    let filtered = if verbose > 0 {
        stdout.trim().to_string()
    } else {
        filter_jj_status(&stdout)
    };

    if let Some(hint) = crate::tee::tee_and_hint(&raw, "jj_status", exit_code) {
        println!("{}\n{}", filtered, hint);
    } else {
        println!("{}", filtered);
    }

    if !stderr.trim().is_empty() {
        eprintln!("{}", stderr.trim());
    }

    timer.track(
        &format!("jj status {}", args.join(" ")),
        &format!("rtk jj status {}", args.join(" ")),
        &raw,
        &filtered,
    );

    if !output.status.success() {
        std::process::exit(exit_code);
    }

    Ok(())
}

pub fn run_diff(args: &[String], verbose: u8) -> Result<()> {
    let timer = tracking::TimedExecution::start();

    let mut cmd = Command::new("jj");
    cmd.args(["diff", "--no-pager", "--git"]);
    for arg in args {
        cmd.arg(arg);
    }

    if verbose > 0 {
        eprintln!("Running: jj diff --no-pager --git {}", args.join(" "));
    }

    let output = cmd
        .output()
        .context("Failed to run jj diff. Is jj installed?")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let raw = format!("{}\n{}", stdout, stderr);

    let exit_code = output
        .status
        .code()
        .unwrap_or(if output.status.success() { 0 } else { 1 });

    let filtered = if verbose > 0 {
        stdout.trim().to_string()
    } else if stdout.trim().is_empty() {
        NO_CHANGES.to_string()
    } else {
        compact_diff(&stdout, 100)
    };

    if let Some(hint) = crate::tee::tee_and_hint(&raw, "jj_diff", exit_code) {
        println!("{}\n{}", filtered, hint);
    } else {
        println!("{}", filtered);
    }

    if !stderr.trim().is_empty() {
        eprintln!("{}", stderr.trim());
    }

    timer.track(
        &format!("jj diff {}", args.join(" ")),
        &format!("rtk jj diff {}", args.join(" ")),
        &raw,
        &filtered,
    );

    if !output.status.success() {
        std::process::exit(exit_code);
    }

    Ok(())
}

pub fn run_log(args: &[String], verbose: u8) -> Result<()> {
    let timer = tracking::TimedExecution::start();

    let mut cmd = Command::new("jj");
    cmd.args(["log", "--no-pager"]);

    let has_limit = args
        .iter()
        .any(|a| a == "-n" || a == "--limit" || a.starts_with("--limit="));

    if !has_limit {
        cmd.args(["--limit", "10"]);
    }

    for arg in args {
        cmd.arg(arg);
    }

    if verbose > 0 {
        eprintln!("Running: jj log --no-pager {}", args.join(" "));
    }

    let output = cmd
        .output()
        .context("Failed to run jj log. Is jj installed?")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let raw = format!("{}\n{}", stdout, stderr);

    let exit_code = output
        .status
        .code()
        .unwrap_or(if output.status.success() { 0 } else { 1 });

    let filtered = if verbose > 0 {
        stdout.trim().to_string()
    } else {
        filter_jj_log(&stdout)
    };

    if let Some(hint) = crate::tee::tee_and_hint(&raw, "jj_log", exit_code) {
        println!("{}\n{}", filtered, hint);
    } else {
        println!("{}", filtered);
    }

    if !stderr.trim().is_empty() {
        eprintln!("{}", stderr.trim());
    }

    timer.track(
        &format!("jj log {}", args.join(" ")),
        &format!("rtk jj log {}", args.join(" ")),
        &raw,
        &filtered,
    );

    if !output.status.success() {
        std::process::exit(exit_code);
    }

    Ok(())
}

pub fn run_show(args: &[String], verbose: u8) -> Result<()> {
    let timer = tracking::TimedExecution::start();

    let mut cmd = Command::new("jj");
    cmd.args(["show", "--no-pager", "--git"]);
    for arg in args {
        cmd.arg(arg);
    }

    if verbose > 0 {
        eprintln!("Running: jj show --no-pager --git {}", args.join(" "));
    }

    let output = cmd
        .output()
        .context("Failed to run jj show. Is jj installed?")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let raw = format!("{}\n{}", stdout, stderr);

    let exit_code = output
        .status
        .code()
        .unwrap_or(if output.status.success() { 0 } else { 1 });

    let filtered = if verbose > 0 {
        stdout.trim().to_string()
    } else {
        filter_jj_show(&stdout)
    };

    if let Some(hint) = crate::tee::tee_and_hint(&raw, "jj_show", exit_code) {
        println!("{}\n{}", filtered, hint);
    } else {
        println!("{}", filtered);
    }

    if !stderr.trim().is_empty() {
        eprintln!("{}", stderr.trim());
    }

    timer.track(
        &format!("jj show {}", args.join(" ")),
        &format!("rtk jj show {}", args.join(" ")),
        &raw,
        &filtered,
    );

    if !output.status.success() {
        std::process::exit(exit_code);
    }

    Ok(())
}

pub fn run_other(args: &[OsString], verbose: u8) -> Result<()> {
    if args.is_empty() {
        anyhow::bail!("jj: no subcommand specified");
    }

    let timer = tracking::TimedExecution::start();

    let subcommand = args[0].to_string_lossy();
    let mut cmd = Command::new("jj");
    cmd.arg("--no-pager");
    cmd.arg(&*subcommand);

    for arg in &args[1..] {
        cmd.arg(arg);
    }

    if verbose > 0 {
        eprintln!("Running: jj --no-pager {} ...", subcommand);
    }

    let output = cmd
        .output()
        .with_context(|| format!("Failed to run jj {}", subcommand))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let raw = format!("{}\n{}", stdout, stderr);

    print!("{}", stdout);
    eprint!("{}", stderr);

    timer.track(
        &format!("jj {}", subcommand),
        &format!("rtk jj {}", subcommand),
        &raw,
        &raw,
    );

    if !output.status.success() {
        std::process::exit(output.status.code().unwrap_or(1));
    }

    Ok(())
}

const NO_CHANGES: &str = "No changes in working copy";

fn filter_jj_status(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return NO_CHANGES.to_string();
    }

    if trimmed.contains("nothing changed") || trimmed.contains("no changes") {
        return NO_CHANGES.to_string();
    }

    let mut result = Vec::new();

    for line in trimmed.lines() {
        let stripped = line.trim();
        if stripped.is_empty() {
            continue;
        }
        if stripped == "Working copy changes:" {
            continue;
        }
        result.push(line.to_string());
    }

    if result.is_empty() {
        return NO_CHANGES.to_string();
    }

    result.join("\n")
}

fn filter_jj_log(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let total_entries = trimmed
        .lines()
        .filter(|line| {
            let s = line.trim_start_matches('│').trim_start_matches(' ');
            s.starts_with('@')
                || s.starts_with('○')
                || s.starts_with('◆')
                || s.starts_with('◉')
                || s.starts_with('●')
        })
        .count();

    let mut result = Vec::new();
    let mut entry_count = 0;
    let max_entries = 15;

    for line in trimmed.lines() {
        if entry_count >= max_entries {
            let remaining = total_entries - max_entries;
            if remaining > 0 {
                result.push(format!("... +{} more entries", remaining));
            }
            break;
        }

        // Count graph node lines as entries (@ or ○ or ◆ at start after optional │ chars)
        let stripped = line.trim_start_matches('│').trim_start_matches(' ');
        if stripped.starts_with('@')
            || stripped.starts_with('○')
            || stripped.starts_with('◆')
            || stripped.starts_with('◉')
            || stripped.starts_with('●')
        {
            entry_count += 1;
        }

        // Strip email addresses to save tokens (Cow avoids clone when no match)
        let replaced = EMAIL_RE.replace_all(line, "");
        let processed = truncate(&replaced, 120);

        result.push(processed);
    }

    result.join("\n")
}

fn filter_jj_show(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    // Split at first "diff --git" line
    if let Some(diff_start) = trimmed.find("\ndiff --git") {
        let header = &trimmed[..diff_start];
        let diff = &trimmed[diff_start + 1..];
        let compacted = compact_diff(diff, 100);
        format!("{}\n{}", header.trim(), compacted)
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_tokens(text: &str) -> usize {
        text.split_whitespace().count()
    }

    #[test]
    fn test_filter_jj_status_with_changes() {
        let input = r#"Working copy changes:
M src/main.rs
A src/new_file.rs
D src/old_file.rs
Working copy : ksrmwuqz abc12345 (no description set)
Parent commit: zzzzzzzz 00000000 (empty) (no description set)
"#;

        let output = filter_jj_status(input);
        assert!(output.contains("M src/main.rs"));
        assert!(output.contains("A src/new_file.rs"));
        assert!(output.contains("D src/old_file.rs"));
        assert!(output.contains("Working copy"));
        assert!(output.contains("Parent commit"));
        assert!(!output.contains("Working copy changes:"));
    }

    #[test]
    fn test_filter_jj_status_clean() {
        let input = "The working copy is clean (nothing changed)\n";
        let output = filter_jj_status(input);
        assert_eq!(output, NO_CHANGES);
    }

    #[test]
    fn test_filter_jj_status_empty() {
        let output = filter_jj_status("");
        assert_eq!(output, NO_CHANGES);
    }

    #[test]
    fn test_filter_jj_log() {
        let input = r#"@  ksrmwuqz user@email.com 2025-06-15 10:30:00 abc12345
│  fix: resolve argument parsing issue
○  tpstlust other@email.com 2025-06-15 09:00:00 bookmark def67890
│  feat: add new command support
○  rrrnwwmq user@email.com 2025-06-14 18:00:00 ghi12345
│  refactor: clean up filter logic
~
"#;

        let output = filter_jj_log(input);
        assert!(output.contains("ksrmwuqz"));
        assert!(output.contains("fix: resolve argument parsing issue"));
        assert!(output.contains("tpstlust"));
        // Email addresses should be stripped
        assert!(!output.contains("user@email.com"));
        assert!(!output.contains("other@email.com"));
    }

    #[test]
    fn test_filter_jj_log_truncation() {
        let mut input = String::new();
        for i in 0..20 {
            input.push_str(&format!(
                "○  change{:02} user@email.com 2025-01-01 00:00:00 hash{:02}\n│  commit message {}\n",
                i, i, i
            ));
        }
        input.push_str("~\n");

        let output = filter_jj_log(&input);
        assert!(output.contains("... +"));
    }

    #[test]
    fn test_filter_jj_log_token_savings() {
        let input = r#"@  ksrmwuqz user@email.com 2025-06-15 10:30:00 abc12345
│  fix: resolve argument parsing issue with a somewhat long description that goes on
○  tpstlust another.user@example.org 2025-06-15 09:00:00 bookmark def67890
│  feat: add new command support for the Jujutsu version control system
○  rrrnwwmq developer@company.com 2025-06-14 18:00:00 ghi12345
│  refactor: clean up filter logic and remove unnecessary allocations
○  ppqqrrss admin@organization.net 2025-06-14 17:00:00 jkl12345
│  docs: update README with new installation instructions
○  aabbccdd contributor@project.io 2025-06-14 16:00:00 mno12345
│  test: add snapshot tests for all filter modules
~
"#;

        let output = filter_jj_log(input);
        let input_tokens = count_tokens(input);
        let output_tokens = count_tokens(&output);
        // Email stripping saves ~6% on small inputs; main savings come from
        // --limit 10 auto-injection at runtime (not tested here)
        assert!(
            output_tokens < input_tokens,
            "jj log filter should reduce tokens: {} -> {}",
            input_tokens,
            output_tokens
        );
    }

    #[test]
    fn test_filter_jj_diff_reuses_compact_diff() {
        let input = r#"diff --git a/src/main.rs b/src/main.rs
index abc1234..def5678 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,5 +1,6 @@
 use std::io;
+use std::fs;

 fn main() {
-    println!("hello");
+    println!("world");
 }
"#;

        let output = compact_diff(input, 100);
        assert!(output.contains("src/main.rs"));
        assert!(output.contains("+use std::fs;"));
    }

    #[test]
    fn test_filter_jj_show_with_diff() {
        let input = r#"Commit ID: abc1234567890
Change ID: def0987654321
Author: Test User <test@example.com>
Date: 2025-06-15 10:30:00

fix: resolve argument parsing issue

diff --git a/src/main.rs b/src/main.rs
index abc1234..def5678 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,5 +1,6 @@
 use std::io;
+use std::fs;

 fn main() {
-    println!("hello");
+    println!("world");
 }
"#;

        let output = filter_jj_show(input);
        assert!(output.contains("Commit ID: abc1234567890"));
        assert!(output.contains("fix: resolve argument parsing issue"));
        // Diff should be compacted
        assert!(output.contains("src/main.rs"));
    }

    #[test]
    fn test_filter_jj_show_no_diff() {
        let input = r#"Commit ID: abc1234567890
Change ID: def0987654321
Author: Test User <test@example.com>
Date: 2025-06-15 10:30:00

(empty) initial commit
"#;

        let output = filter_jj_show(input);
        assert!(output.contains("Commit ID: abc1234567890"));
        assert!(output.contains("initial commit"));
    }

    #[test]
    fn test_filter_jj_show_token_savings() {
        let input = r#"Commit ID: abc1234567890abcdef1234567890abcdef12345678
Change ID: def0987654321fedcba0987654321fedcba09876543
Author: Test User <test@example.com>
Date: 2025-06-15 10:30:00

fix: resolve argument parsing issue

diff --git a/src/main.rs b/src/main.rs
index abc1234..def5678 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,15 +1,16 @@
 use std::io;
+use std::fs;
 use std::env;
 use std::path::Path;

 fn main() {
-    println!("hello");
+    println!("world");
     let x = 1;
     let y = 2;
     let z = x + y;
 }

-fn old_function() {
-    // removed
-}
+fn new_function() {
+    // added
+}
diff --git a/src/lib.rs b/src/lib.rs
index 1111111..2222222 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,10 +1,12 @@
 pub mod utils;
+pub mod helpers;

 pub fn run() {
-    old_impl();
+    new_impl();
 }
"#;

        let output = filter_jj_show(input);
        let input_tokens = count_tokens(input);
        let output_tokens = count_tokens(&output);
        let savings = 100.0 - (output_tokens as f64 / input_tokens as f64 * 100.0);
        assert!(
            savings >= 5.0,
            "jj show filter: expected token savings, got {:.1}%",
            savings
        );
    }

    #[test]
    fn test_filter_jj_status_token_savings() {
        let input = r#"Working copy changes:
M src/main.rs
M src/lib.rs
A src/new_file.rs
A src/another_new.rs
D src/old_file.rs
Working copy : ksrmwuqz abc12345 (no description set)
Parent commit: zzzzzzzz 00000000 (empty) (no description set)
"#;

        let output = filter_jj_status(input);
        let input_tokens = count_tokens(input);
        let output_tokens = count_tokens(&output);
        // Removing the header line saves some tokens
        assert!(
            output_tokens <= input_tokens,
            "jj status filter should not increase tokens: {} -> {}",
            input_tokens,
            output_tokens
        );
    }
}
