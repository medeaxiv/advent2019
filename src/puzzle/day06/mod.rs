use ahash::AHashMap as HashMap;

use crate::util::graph::search::breadth_first_search;

pub const INPUT_FILE: &str = "inputs/day06/input.txt";

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> i64 {
    let root = "COM";
    let system = parse(input, root);

    fn count(idx: usize, depth: i64, system: &Tree) -> i64 {
        if system.children[idx].is_empty() {
            depth
        } else {
            system.children[idx]
                .iter()
                .map(|&child_idx| count(child_idx, depth + 1, system))
                .sum::<i64>()
                + depth
        }
    }

    let root_idx = system.index[root];
    count(root_idx, 0, &system)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> i64 {
    let root = "COM";
    let system = parse(input, root);
    let you_idx = system.index["YOU"];
    let san_idx = system.index["SAN"];
    let start = system.parents[you_idx].expect("YOU node should have a parent");
    let end = system.parents[san_idx].expect("SAN node should have a parent");

    let distance = breadth_first_search(
        |&node, _| {
            system.children[node]
                .iter()
                .cloned()
                .chain(system.parents[node])
        },
        |&node, depth| if node == end { Some(depth) } else { None },
        [start],
    )
    .expect("Tree should be fully linked");

    distance as i64
}

#[derive(Debug, Default, Clone)]
struct Tree<'a> {
    nodes: Vec<&'a str>,
    index: HashMap<&'a str, usize>,
    children: Vec<Vec<usize>>,
    parents: Vec<Option<usize>>,
}

impl<'a> Tree<'a> {
    fn insert_node(&mut self, node: &'a str) -> usize {
        if let Some(idx) = self.index.get(node) {
            *idx
        } else {
            let idx = self.nodes.len();
            self.nodes.push(node);
            self.index.insert(node, idx);
            self.children.push(Vec::new());
            self.parents.push(None);
            idx
        }
    }

    pub fn add_node(&mut self, node: &'a str) {
        self.insert_node(node);
    }

    pub fn add_branch(&mut self, parent: &'a str, child: &'a str) {
        let parent_idx = self.insert_node(parent);
        let child_idx = self.insert_node(child);

        if let Some(idx) = self.parents[child_idx] {
            tracing::error!(
                node = child,
                parent = self.nodes[idx],
                new_parent = parent,
                "Node already has a parent"
            );
            panic!("Node already has a parent");
        } else {
            self.parents[child_idx] = Some(parent_idx);
            self.children[parent_idx].push(child_idx);
        }
    }
}

fn parse<'a>(input: &'a str, root: &'a str) -> Tree<'a> {
    let mut tree = Tree::default();
    tree.add_node(root);

    for line in input.lines() {
        let (parent, child) = line.split_once(')').expect("Invalid input line");
        tree.add_branch(parent, child);
    }

    tree
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn input(which: usize) -> String {
        let file = format!("inputs/day06/test.{}.txt", which);
        std::fs::read_to_string(file).expect("Missing test input file")
    }

    #[rstest]
    #[case(0, 42)]
    fn test_part1(#[case] which: usize, #[case] expected: i64) {
        crate::util::test::setup_tracing();
        let input = input(which);
        let result = solve_part1(&input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(1, 4)]
    fn test_part2(#[case] which: usize, #[case] expected: i64) {
        crate::util::test::setup_tracing();
        let input = input(which);
        let result = solve_part2(&input);
        assert_eq!(result, expected);
    }
}
