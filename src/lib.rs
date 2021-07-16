use regex::Regex;
use std::{collections::HashSet, iter};

/// Given a tree output and a line number, returns a Vec containing the indices of all ancestors of
/// the specified node in the tree.
pub fn list_ancestors(data: &[String], line: usize) -> Vec<usize> {
    iter::successors(Some(line), |&line| find_parent(data, line)).collect()
}

pub fn get_depth(line: &str) -> usize {
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
