use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use pad::{PadStr, Alignment};

fn split(line: &String, comment: &str) -> String {
    let code: Vec<&str> = line.split(comment).collect();
    format!("{}\n", code[0].trim_right())
}

/// This function extracts code snippets from their larger contexts. A snippet
/// is defined by `<id> begin` and `<id> end` comments -- e.g.,
///
/// ```python
/// import spacy
///
/// def my_component(doc):  # my_component begin
///     print("After tokenization, this doc has %s tokens." % len(doc))
///     if len(doc) < 10:
///         print("This is a pretty short document.")
///     return doc
///
/// nlp = spacy.load('en')
/// nlp.add_pipe(my_component, name='print_info', first=True)
/// print(nlp.pipe_names)  # ['print_info', 'tagger', 'parser', 'ner']
/// doc = nlp(u"This is a sentence.")  # my_component end
/// ```
///
/// Where the snippet begins with the `# my_component begin` line and ends on
/// the `# my_component end` line.
pub fn extract(path: &Path, id: &str, padding: usize, languages: &HashMap<&OsStr, &str>) -> String {
    let delimiter = match languages.get(path.extension().unwrap()) {
            Some(delimiter) => delimiter,
            _ => panic!("{} not supported", path.display()),
        };

    let f = File::open(path).unwrap();

    let mut reader = BufReader::new(f);
    let mut line = String::new();

    let begin = format!("{} begin", id);
    let end = format!("{} end", id);

    let mut add_line = false;
    let mut content = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        {
            let padded = line.pad_to_width_with_alignment(
                            line.chars().count() + padding - 1,
                            Alignment::Right);
            if line.trim().ends_with(begin.as_str()) {
                content.push_str(split(&padded, delimiter).as_str());
                add_line = true
            } else if line.trim().ends_with(end.as_str()) {
                content.push_str(split(&padded, delimiter).as_str());
                return content;
            } else if add_line {
                if line.trim().is_empty() {
                    content.push_str(line.as_str());
                } else {
                    content.push_str(padded.as_str());
                }
            }
        }
        line.clear();
    }

    content
}