//! End-to-end tests: run the real `wordfreq` binary and check stdout, stderr
//! and the exit code.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const BIN: &str = env!("CARGO_BIN_EXE_wordfreq");

/// A fresh, empty folder for one test, inside Cargo's target directory.
fn workdir(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write(dir: &Path, name: &str, text: &str) -> PathBuf {
    let path = dir.join(name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&path, text).unwrap();
    path
}

fn run(args: &[&str]) -> Output {
    Command::new(BIN).args(args).output().unwrap()
}

fn run_with_stdin(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(BIN)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

fn s(p: &Path) -> &str {
    p.to_str().unwrap()
}

/// Path to a sample file shipped in the repo's `files/` folder.
fn sample(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("files")
        .join(name)
}

#[test]
fn single_file_prints_path_and_ranked_words() {
    let dir = workdir("single");
    let f = write(&dir, "a.txt", "the cat and the hat. The end");
    let out = run(&[s(&f), "-n", "2"]);
    assert!(out.status.success());
    assert_eq!(
        stdout(&out),
        format!(
            "Path: {}\n 1. the             3\n 2. and             1\n",
            f.display()
        )
    );
    assert_eq!(stderr(&out), "");
}

#[test]
fn several_files_are_counted_together() {
    let dir = workdir("several");
    let a = write(&dir, "a.txt", "apple apple");
    let b = write(&dir, "b.md", "apple banana");
    let out = run(&[s(&a), s(&b), "-f", "csv"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "rank,word,count\n1,apple,3\n2,banana,1\n");
}

#[test]
fn folder_needs_recursive_for_subfolders() {
    let dir = workdir("folder");
    write(&dir, "a.txt", "top");
    write(&dir, "sub/b.txt", "deep");
    write(&dir, "notes.csv", "ignored");

    let flat = stdout(&run(&[s(&dir), "-f", "csv"]));
    assert_eq!(flat, "rank,word,count\n1,top,1\n");

    let deep = stdout(&run(&[s(&dir), "-r", "-f", "csv"]));
    assert_eq!(deep, "rank,word,count\n1,deep,1\n2,top,1\n");
}

#[test]
fn reads_text_from_stdin() {
    let out = run_with_stdin(&["-", "-n", "1"], "to be or not to be");
    assert!(out.status.success());
    assert_eq!(stdout(&out), "Path: -\n 1. be              2\n");
}

#[test]
fn table_format_draws_a_box() {
    let out = run_with_stdin(&["-", "-f", "table"], "a a b");
    let text = stdout(&out);
    assert!(text.contains("┌") && text.contains("└"), "{text}");
    assert!(text.contains("│ 1 │ a    │     2 │"), "{text}");
}

#[test]
fn verbose_stats_go_to_stderr_not_stdout() {
    let out = run_with_stdin(&["-", "-f", "csv", "-v"], "a a b");
    assert_eq!(stdout(&out), "rank,word,count\n1,a,2\n2,b,1\n");
    let err = stderr(&out);
    assert!(err.contains("Total words     : 3"), "{err}");
    assert!(err.contains("Diversity ratio : 0.667"), "{err}");
}

#[test]
fn filters_remove_stop_words_excluded_words_and_lengths() {
    let dir = workdir("filters");
    let text = write(
        &dir,
        "t.txt",
        "the a cat cat jumped jumped jumped over dogs",
    );
    let exclude = write(&dir, "x.txt", "over\n  DOGS  \n");
    let out = run(&[
        s(&text),
        "--ignore-stop-words",
        "-x",
        s(&exclude),
        "--min-len",
        "3",
        "--max-len",
        "4",
        "-f",
        "csv",
    ]);
    assert_eq!(stdout(&out), "rank,word,count\n1,cat,2\n");
}

#[test]
fn case_sensitive_keeps_capitals_apart() {
    let out = run_with_stdin(&["-", "-i", "-f", "csv"], "The the");
    assert_eq!(stdout(&out), "rank,word,count\n1,The,1\n2,the,1\n");
}

#[test]
fn missing_file_is_an_error() {
    let out = run(&["definitely-missing.txt"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).starts_with("error: cannot read definitely-missing.txt"));
    assert_eq!(stdout(&out), "");
}

#[test]
fn unsupported_file_type_is_an_error() {
    let dir = workdir("unsupported");
    let f = write(&dir, "data.csv", "a,b");
    let out = run(&[s(&f)]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("unsupported file format '.csv'"));
}

#[test]
fn empty_and_whitespace_files_are_errors() {
    let dir = workdir("empty");
    for (name, text) in [("zero.txt", ""), ("blank.txt", "  \n\t\n")] {
        let f = write(&dir, name, text);
        let out = run(&[s(&f)]);
        assert_eq!(out.status.code(), Some(1));
        assert!(stderr(&out).contains("is completely empty"), "{name}");
    }
}

#[test]
fn invalid_length_range_is_an_error() {
    let out = run_with_stdin(&["-", "--min-len", "5", "--max-len", "2"], "word");
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        stderr(&out),
        "error: --min-len (5) is greater than --max-len (2)\n"
    );
}

#[test]
fn bad_flag_value_exits_with_2() {
    let out = run(&["x.txt", "-f", "json"]);
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn one_bad_file_among_several_is_skipped_with_a_warning() {
    let dir = workdir("partial");
    let good = write(&dir, "good.txt", "fine fine");
    let broken = write(&dir, "broken.pdf", "not a pdf");
    let out = run(&[s(&broken), s(&good), "-f", "csv"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "rank,word,count\n1,fine,2\n");
    let err = stderr(&out);
    assert!(err.starts_with("warning: cannot parse"), "{err}");
    assert!(
        err.contains("broken.pdf") && err.contains("(skipped)"),
        "{err}"
    );
}

#[test]
fn all_files_failing_is_an_error() {
    let out = run(&["missing-1.txt", "missing-2.txt"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).ends_with("error: none of the given files could be read\n"));
}

#[test]
fn reads_docx_files() {
    let out = run(&[s(&sample("dsa_patterns.docx")), "-n", "1", "-f", "csv"]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(stdout(&out), "rank,word,count\n1,the,59\n");
}

#[test]
fn reads_pdf_files() {
    let out = run(&[
        s(&sample("Competitive-Programmer-Handbook.pdf")),
        "-n",
        "1",
        "-f",
        "csv",
    ]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(stdout(&out), "rank,word,count\n1,the,5800\n");
}
