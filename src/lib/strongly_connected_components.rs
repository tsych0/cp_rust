use std::collections::{HashMap, HashSet};
use std::hash::Hash;

struct Params<N, FN>
where
    N: Hash + Eq,
{
    preorders: HashMap<N, Option<usize>>,
    c: usize,
    successors: FN,
    p: Vec<N>,
    s: Vec<N>,
    scc: Vec<Vec<N>>,
    scca: HashSet<N>,
}

impl<N, FN, IN> Params<N, FN>
where
    N: Clone + Hash + Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    fn new(nodes: &[N], successors: FN) -> Self {
        Self {
            preorders: nodes
                .iter()
                .map(|n| (n.clone(), None))
                .collect::<HashMap<N, Option<usize>>>(),
            c: 0,
            successors,
            p: Vec::new(),
            s: Vec::new(),
            scc: Vec::new(),
            scca: HashSet::new(),
        }
    }
}

fn recurse_onto<N, FN, IN>(v: &N, params: &mut Params<N, FN>)
where
    N: Clone + Hash + Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    params.preorders.insert(v.clone(), Some(params.c));
    params.c += 1;
    params.s.push(v.clone());
    params.p.push(v.clone());
    for w in (params.successors)(v) {
        if !params.scca.contains(&w) {
            if let Some(pw) = params.preorders.get(&w).and_then(|w| *w) {
                while params.preorders[&params.p[params.p.len() - 1]].unwrap() > pw {
                    params.p.pop();
                }
            } else {
                recurse_onto(&w, params);
            }
        }
    }
    if params.p[params.p.len() - 1] == *v {
        params.p.pop();
        let mut component = Vec::new();
        while let Some(node) = params.s.pop() {
            component.push(node.clone());
            params.scca.insert(node.clone());
            params.preorders.remove(&node);
            if node == *v {
                break;
            }
        }
        params.scc.push(component);
    }
}

/// Partition nodes reachable from a starting point into strongly connected components.
pub fn strongly_connected_components_from<N, FN, IN>(start: &N, successors: FN) -> Vec<Vec<N>>
where
    N: Clone + Hash + Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut params = Params::new(&[], successors);
    recurse_onto(start, &mut params);
    params.scc
}

/// Compute the strongly connected component containing a given node.
pub fn strongly_connected_component<N, FN, IN>(node: &N, successors: FN) -> Vec<N>
where
    N: Clone + Hash + Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    // The unwrap() cannot fail as there will always be at least one group.
    strongly_connected_components_from(node, successors)
        .pop()
        .unwrap()
}

/// Partition all strongly connected components is a graph.
pub fn strongly_connected_components<N, FN, IN>(nodes: &[N], successors: FN) -> Vec<Vec<N>>
where
    N: Clone + Hash + Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut params = Params::new(nodes, successors);
    while let Some(node) = params.preorders.keys().find(|_| true).cloned() {
        recurse_onto(&node, &mut params);
    }
    params.scc
}
