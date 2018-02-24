extern crate clap;
extern crate regex;
extern crate walkdir;

use std::collections::HashMap;
use std::ffi::OsStr;

use clap::{App};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

mod buffer;
mod block;

fn is_match(entry: &DirEntry, ext: &str) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.ends_with(ext))
         .unwrap_or(false)
}

fn main() {
    let matches = App::new("blocktest")
                          .version("1.0")
                          .author("Joseph Kato. <joseph@jdkato.io>")
                          .about("A preprocessor for markup code blocks.")
                          .args_from_usage(
                              "<src>              'Source directory'
                               <ext>              'Target file extension (e.g., '.md')'
                               <out>              'Output directory'")
                          .get_matches();

    let mut languages: HashMap<&OsStr, &str> = HashMap::new();
    // The languages we support:
    //
    // TODO: Use syntect to support languages file OCaml:
    //    (* This is a single-line comment. *)
    languages.insert(OsStr::new("py"), "#"); // Python
    languages.insert(OsStr::new("rb"), "#"); // Ruby
    languages.insert(OsStr::new("pl"), "#"); // Perl
    languages.insert(OsStr::new("r"), "#"); // R

    languages.insert(OsStr::new("hs"), "--"); // Haskell
    languages.insert(OsStr::new("lua"), "--"); // Lua

    languages.insert(OsStr::new("c"), "//"); // C
    languages.insert(OsStr::new("d"), "//"); // D
    languages.insert(OsStr::new("cs"), "//"); // C#
    languages.insert(OsStr::new("cpp"), "//"); // C++
    languages.insert(OsStr::new("go"), "//"); // Go
    languages.insert(OsStr::new("java"), "//"); // Java
    languages.insert(OsStr::new("js"), "//"); // JavaScript
    languages.insert(OsStr::new("jl"), "//"); // Julia
    languages.insert(OsStr::new("m"), "//"); // Objective-C
    languages.insert(OsStr::new("php"), "//"); // PHP
    languages.insert(OsStr::new("rs"), "//"); // Rust
    languages.insert(OsStr::new("scala"), "//"); // Scala
    languages.insert(OsStr::new("swift"), "//"); // Swift

    let pat = Regex::new(r#"\{\{< id="(.+)" src="(.+)" >\}\}"#).unwrap();
    let src = matches.value_of("src").unwrap();
    let ext = matches.value_of("ext").unwrap();

    let walker = WalkDir::new(src).into_iter();
    let dest = matches.value_of("out").unwrap();
    for entry in walker {
        let entry = entry.unwrap();
        if is_match(&entry, ext) {
            let dest = entry.path().to_str().unwrap().replace(src, dest);
            buffer::compile(&entry.path(), &dest, &pat, &languages);
        }
    }
}