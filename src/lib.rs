use regex::Regex;
use std::{collections::HashSet, iter};

/// Given a tree output and a line number, returns a Vec containing the indices of all ancestors of
/// the specified node in the tree.
pub fn list_ancestors(data: &[String], line: usize) -> Vec<usize> {
    iter::successors(Some(line), |&line| find_parent(data, line)).collect()
}

fn get_depth(line: &str) -> usize {
    line.chars().take_while(|c| !c.is_alphanumeric()).count() / 4
}

/// Given a tree output and a line number, returns the index of the given node's parent (or `None`
/// if this node is a root)
pub fn find_parent(data: &[String], line: usize) -> Option<usize> {
    let depth = get_depth(&data[line]);
    (0..line)
        .rev()
        .filter(|&line| get_depth(&data[line]) < depth)
        .next()
}

fn filter_by_predicate(data: Vec<String>, predicate: impl Fn(&str) -> bool) -> Vec<String> {
    let lines: HashSet<usize> = (0..data.len())
        .filter(|&i| predicate(&data[i]))
        .flat_map(|i| list_ancestors(&data, i))
        .collect();
    data.into_iter()
        .enumerate()
        .filter(|(i, _)| lines.contains(i))
        .map(|(_, s)| s)
        .collect()
}

/// Given a regex, filter to only include lines which match the given regex
pub fn filter_by_regex(data: Vec<String>, regex: Regex) -> Vec<String> {
    filter_by_predicate(data, |s| regex.is_match(s))
}

#[cfg(test)]
mod test {
    use super::*;

    /// Use the dependency tree as a test dataset
    static DEPTREE: &'static str = "treegrep v0.1.0
├── clap v2.33.3
│   ├── ansi_term v0.11.0
│   ├── atty v0.2.14
│   │   └── libc v0.2.98
│   ├── bitflags v1.2.1
│   ├── strsim v0.8.0
│   ├── textwrap v0.11.0
│   │   └── unicode-width v0.1.8
│   ├── unicode-width v0.1.8
│   └── vec_map v0.8.2
└── regex v1.5.4
    ├── aho-corasick v0.7.18
    │   └── memchr v2.4.0
    ├── memchr v2.4.0
    └── regex-syntax v0.6.25";

    fn string_to_line_vec(s: &str) -> Vec<String> {
        s.lines().map(str::to_string).collect()
    }

    #[test]
    fn test_filter_by_regex() {
        let regex = Regex::new("memchr").unwrap();
        let answer = "treegrep v0.1.0
└── regex v1.5.4
    ├── aho-corasick v0.7.18
    │   └── memchr v2.4.0
    ├── memchr v2.4.0";
        assert_eq!(
            string_to_line_vec(answer),
            filter_by_regex(string_to_line_vec(DEPTREE), regex)
        );
    }

    #[test]
    fn test_find_parent() {
        let deptree = string_to_line_vec(DEPTREE);
        for (child, parent) in [
            (0, None),
            (1, Some(0)),
            (2, Some(1)),
            (3, Some(1)),
            (4, Some(3)),
            (11, Some(0)),
            (14, Some(11)),
        ] {
            assert_eq!(parent, find_parent(&deptree, child));
        }
    }

    #[test]
    fn test_list_ancestors() {
        let deptree = string_to_line_vec(DEPTREE);
        for (child, ancestry) in [
            (0, vec![0]),
            (1, vec![1, 0]),
            (2, vec![2, 1, 0]),
            (3, vec![3, 1, 0]),
            (4, vec![4, 3, 1, 0]),
            (11, vec![11, 0]),
            (14, vec![14, 11, 0]),
        ] {
            assert_eq!(
                ancestry.as_slice(),
                list_ancestors(&deptree, child).as_slice(),
            );
        }
    }
}
