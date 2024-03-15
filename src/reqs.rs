use std::collections::HashSet;
use std::io::BufReader;
use std::io::{BufRead, Write};
use std::path::Path;
use std::path::PathBuf;

pub(crate) struct Requirements {
    reqs: HashSet<String>,
}

impl Requirements {
    pub(crate) fn new() -> Self {
        Self {
            reqs: HashSet::new(),
        }
    }

    pub(crate) fn add(&mut self, req: impl Into<String>) {
        self.reqs.insert(req.into());
    }

    pub(crate) fn parse_and_append(&mut self, file: impl std::io::Read) -> std::io::Result<()> {
        let reqs = parse(file)?;
        self.reqs.extend(reqs.reqs);

        Ok(())
    }

    pub(crate) fn write_in(&self, path: &Path) -> std::io::Result<Option<PathBuf>> {
        if self.reqs.is_empty() {
            Ok(None)
        } else {
            let path = path.join("requirements.txt");

            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .create_new(true)
                .write(true)
                .open(&path)?;

            for r in self.reqs.iter() {
                writeln!(f, "{}", r)?;
            }

            Ok(Some(path))
        }
    }
}

pub(crate) fn parse(rdr: impl std::io::Read) -> std::io::Result<Requirements> {
    let mut reqs = HashSet::new();
    for line in BufReader::new(rdr).lines() {
        let line = line?;
        for dep in parse_line_req(&line).into_iter().flatten() {
            reqs.insert(dep.to_owned());
        }
    }
    Ok(Requirements { reqs })
}

fn parse_line_req(line: &str) -> Option<HashSet<&str>> {
    let mut modules = HashSet::new();

    let (code, comment) = line.split_once('#')?;
    let comment = comment.trim_start();
    if !comment.starts_with("fades") && !comment.starts_with("spyn") {
        return None;
    }

    if code.starts_with("from") {
        // from import
        let Some(module_name) = code.split_whitespace().nth(1) else {
            return None;
        };
        modules.insert(module_name);
    } else if let Some(import_str) = code.strip_prefix("import") {
        for import_substr in import_str.split(',') {
            modules.insert(import_substr.split_whitespace().next().unwrap());
        }
    }

    Some(modules)
}

#[test]
fn test_parse_line_req() {
    use literally::hset;
    fn parse(line: &str) -> HashSet<&str> {
        parse_line_req(line).unwrap_or_default()
    }

    assert!(parse("import x").is_empty());
    assert_eq!(parse("import x # fades"), hset! {"x"});
    assert_eq!(parse("import x # spyn"), hset! {"x"});
    assert_eq!(parse("import x as y # spyn"), hset! {"x"});
    assert_eq!(parse("import x as y, z as b # spyn"), hset! {"x", "z"});
    assert_eq!(parse("from a import x as y, z as b # spyn"), hset! {"a"});
    assert_eq!(
        parse("from a import x as y, z as b #    fades"),
        hset! {"a"}
    );
}
