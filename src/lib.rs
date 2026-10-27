//! Count the most frequent words in text, files and folders.
//!
//! This crate powers the `wfreq` command line tool, and its modules can
//! also be used as a library. The usual flow is:
//!
//! 1. [`input::collect_files`] turns paths into a list of files.
//! 2. [`parser::extract_text`] reads each file (.txt, .md, .pdf, .docx) as text.
//! 3. [`core::count_words`] counts words, and [`core::merge_counts`] adds files together.
//! 4. [`core::filter_by_length`] and [`core::build_ignore_set`] remove unwanted words.
//! 5. [`core::top_n`] ranks the words, and [`output::write_top`] prints them.
//!
//! # Example
//!
//! ```
//! use wfreq::core::{count_words, top_n};
//! use wfreq::output::{write_top, Format};
//!
//! let counts = count_words("the cat saw the dog", false);
//! let top = top_n(&counts, 2);
//! assert_eq!(top, vec![("the".to_string(), 2), ("cat".to_string(), 1)]);
//!
//! let mut out = Vec::new();
//! write_top(&mut out, &top, Format::Csv).unwrap();
//! assert_eq!(String::from_utf8(out).unwrap(), "rank,word,count\n1,the,2\n2,cat,1\n");
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod cli;
pub mod core;
pub mod error;
pub mod input;
pub mod output;
pub mod parser;
