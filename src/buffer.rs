use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;

use regex::Regex;

use block;

fn write_file(contents: &String, dest: &String) {
    let location = Path::new(dest.as_str());
    fs::create_dir_all(location.parent().unwrap()).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    let display = location.display();
    let mut file = match File::create(&location) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Successfully compiled {}", display),
    }
}

/// This function finds and parses block definitions, which take the follwing
/// form:
///
/// ```
/// {{< id="..." src="..." >}}
/// ```
///
/// where `id` is a unique label for the particular block and `src` is the file
/// that contains the source code.
pub fn compile(src: &Path, dst: &String, pat: &Regex, languages: &HashMap<&OsStr, &str>) {
    let display = src.display();
    let mut file = match File::open(src) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut source = String::new();
        file.read_to_string(&mut source).unwrap();
        let mut compiled = source.clone();

        for cap in pat.captures_iter(&source.to_owned()) {
            let definition = &cap[0];
            let path = Path::new(&cap[3]);
            let padding = cap[1].chars().count();
            let content = block::extract(&path, &cap[2], padding, languages);
            let spaced = str::replace(content.as_str(), "\t", "   ");
            compiled = source.replace(definition, spaced.trim_right());
        }

    write_file(&compiled, dst);
}
