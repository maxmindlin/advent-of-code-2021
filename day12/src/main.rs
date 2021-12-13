use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum NodeKind {
    Sml,
    Lrg,
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<Node, Vec<Node>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    val: String,
    kind: NodeKind,
}

fn main() {
    let input = std::fs::read_to_string("input/1.txt").unwrap();
    let g = Graph::from(input.as_str());

    let mut visited = HashMap::new();
    let mut path = Vec::new();
    let mut count = 0;
    let s = Node::from("start");
    let e = Node::from("end");
    g.dfs(s, e, &mut visited, &mut path, &mut count);
    println!("ANSWER: {}", count);
}

impl Graph {
    fn dfs(
        &self,
        start: Node,
        t: Node,
        visited: &mut HashMap<Node, usize>,
        path: &mut Vec<Node>,
        count: &mut usize,
    ) {
        if start.kind == NodeKind::Sml {
            let entry = visited.entry(start.clone()).or_insert(0);
            *entry += 1;
        }
        path.push(start.clone());

        if start == t {
            *count += 1;
        } else {
            if let Some(edges) = self.nodes.get(&start) {
                for node in edges {
                    let any_2 = visited.iter().any(|(_, v)| *v == 2);
                    let v_count = visited.get(node).unwrap_or(&0);
                    if *v_count == 0 || (*v_count == 1 && !any_2) {
                        self.dfs(node.clone(), t.clone(), visited, path, count);
                    }
                }
            }
        }

        if let Some(entry) = visited.get_mut(&start) {
            *entry -= 1;
        }
        path.pop();
    }
}

impl From<&str> for Graph {
    fn from(s: &str) -> Self {
        let mut nodes = HashMap::new();
        for l in s.lines() {
            let v: Vec<&str> = l.split("-").collect();
            let n1 = Node::from(v[0]);
            let n2 = Node::from(v[1]);

            // start and end nodes are not undirected like the rest of the nodes,
            // but can still be presenting in the input like the rest. So we need to
            // make sure to only add an edge one way if the node we are parsing
            // is connected to either the start or end..
            if &n1.val == "start" || &n2.val == "start" {
                let (s, n) = if &n1.val == "start" {
                    (n1, n2)
                } else {
                    (n2, n1)
                };

                let e = nodes.entry(s).or_insert(vec![]);
                e.push(n);
            } else if &n1.val == "end" || &n2.val == "end" {
                let (n, e) = if &n1.val == "end" { (n2, n1) } else { (n1, n2) };

                let en = nodes.entry(n).or_insert(vec![]);
                en.push(e);
            } else {
                let e1 = nodes.entry(n1.clone()).or_insert(vec![]);
                e1.push(n2.clone());
                let e2 = nodes.entry(n2).or_insert(vec![]);
                e2.push(n1);
            }
        }

        Self { nodes }
    }
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        let c: Vec<char> = s.chars().collect();
        let kind = if c[0].is_uppercase() {
            NodeKind::Lrg
        } else {
            NodeKind::Sml
        };

        Self {
            val: c.iter().collect(),
            kind,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let i = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let g = Graph::from(i);
        let mut visited = HashMap::new();
        let mut path: Vec<Node> = Vec::new();
        let mut count = 0;

        let start = Node::from("start");
        let end = Node::from("end");
        g.dfs(start, end, &mut visited, &mut path, &mut count);
        assert_eq!(count, 36);
    }

    #[test]
    fn base2() {
        let i = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let g = Graph::from(i);
        let mut visited = HashMap::new();
        let mut path: Vec<Node> = Vec::new();
        let mut count = 0;

        let start = Node::from("start");
        let end = Node::from("end");
        g.dfs(start, end, &mut visited, &mut path, &mut count);
        assert_eq!(count, 103);
    }
}
