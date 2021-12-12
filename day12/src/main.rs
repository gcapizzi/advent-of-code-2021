use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    let graph = parse_graph(&input)?;
    dbg!(paths(&graph)?.len());
    Ok(())
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Node<'a> {
    Start,
    End,
    Big(&'a str),
    Small(&'a str),
}

type Edge<'a> = (Node<'a>, Node<'a>);
type Graph<'a> = HashSet<Edge<'a>>;
type Path<'a> = Vec<Node<'a>>;

fn paths<'a>(graph: &'a Graph<'a>) -> Result<HashSet<Path<'a>>> {
    let mut paths = HashSet::new();
    paths.insert(vec![Node::Start]);

    loop {
        paths = paths
            .iter()
            .flat_map(|path| {
                let start = path.last().unwrap();
                if *start == Node::End {
                    return vec![path.clone()];
                }
                graph
                    .iter()
                    .filter(|(from, to)| from == start && can_visit_node(path, to))
                    .map(|edge| append_to_path(path, edge.1.clone()))
                    .collect()
            })
            .collect::<HashSet<Path>>();

        if paths.iter().all(|p| p.last().unwrap() == &Node::End) {
            break;
        }
    }

    Ok(paths)
}

fn can_visit_node(path: &Path, node: &Node) -> bool {
    match node {
        Node::Start => false,
        Node::End => true,
        // Node::Small(_) => !path.contains(node),
        Node::Small(_) => !path.contains(node) || all_small_visited_once(path),
        Node::Big(_) => true,
    }
}

fn all_small_visited_once(path: &Path) -> bool {
    let smalls = path
        .into_iter()
        .filter(|n| match n {
            Node::Small(_) => true,
            _ => false,
        })
        .collect::<Vec<&Node>>();
    smalls.iter().cloned().collect::<HashSet<&Node>>().len() == smalls.len()
}

fn append_to_path<'a>(path: &Path<'a>, node: Node<'a>) -> Path<'a> {
    let mut new_path = path.clone();
    new_path.push(node);
    new_path
}

fn parse_graph<'a, I: IntoIterator<Item = &'a String>>(lines: I) -> Result<Graph<'a>> {
    let edges = lines
        .into_iter()
        .map(|s| parse_edge(s).ok_or(anyhow!("invalid edge: {}", s)))
        .collect::<Result<Graph>>()?;
    Ok(edges
        .into_iter()
        .flat_map(|(x, y)| vec![(x.clone(), y.clone()), (y.clone(), x.clone())])
        .collect())
}

fn parse_edge<'a>(s: &'a str) -> Option<Edge<'a>> {
    let (from, to) = s.trim().split_once('-')?;
    Some((parse_node(from)?, parse_node(to)?))
}

fn parse_node<'a>(s: &'a str) -> Option<Node<'a>> {
    if s == "start" {
        return Some(Node::Start);
    }

    if s == "end" {
        return Some(Node::End);
    }

    if s.chars().all(|c| c.is_uppercase()) {
        return Some(Node::Big(s));
    }

    if s.chars().all(|c| c.is_lowercase()) {
        return Some(Node::Small(s));
    }

    None
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_graph, paths};
    use anyhow::Result;

    #[test]
    fn test_paths() -> Result<()> {
        let lines = vec![
            "dc-end".to_string(),
            "HN-start".to_string(),
            "start-kj".to_string(),
            "dc-start".to_string(),
            "dc-HN".to_string(),
            "LN-dc".to_string(),
            "HN-end".to_string(),
            "kj-sa".to_string(),
            "kj-HN".to_string(),
            "kj-dc".to_string(),
        ];
        let graph = parse_graph(&lines)?;

        assert_eq!(paths(&graph)?.len(), 103);

        Ok(())
    }
}
