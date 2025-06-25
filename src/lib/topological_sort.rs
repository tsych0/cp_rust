//! Find a topological order is a directed graph if one exists.

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::mem;

/// Find a topological order is a directed graph if one exists.
pub fn topological_sort<N, FN, IN>(roots: &[N], mut successors: FN) -> Result<Vec<N>, N>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>, {
    let mut marked = HashSet::with_capacity(roots.len());
    let mut temp = HashSet::new();
    let mut sorted = VecDeque::with_capacity(roots.len());
    let mut roots: HashSet<N> = roots.iter().cloned().collect::<HashSet<_>>();
    while let Some(node) = roots.iter().next().cloned() {
        temp.clear();
        visit(
            &node,
            &mut successors,
            &mut roots,
            &mut marked,
            &mut temp,
            &mut sorted,
        )?;
    }
    Ok(sorted.into_iter().collect())
}

fn visit<N, FN, IN>(
    node: &N,
    successors: &mut FN,
    unmarked: &mut HashSet<N>,
    marked: &mut HashSet<N>,
    temp: &mut HashSet<N>,
    sorted: &mut VecDeque<N>,
) -> Result<(), N>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>, {
    unmarked.remove(node);
    if marked.contains(node) {
        return Ok(());
    }
    if temp.contains(node) {
        return Err(node.clone());
    }
    temp.insert(node.clone());
    for n in successors(node) {
        visit(&n, successors, unmarked, marked, temp, sorted)?;
    }
    marked.insert(node.clone());
    sorted.push_front(node.clone());
    Ok(())
}

/// Topologically sort a directed graph into groups of independent nodes.
pub fn topological_sort_into_groups<N, FN, IN>(
    nodes: &[N],
    mut successors: FN,
) -> Result<Vec<Vec<N>>, (Vec<Vec<N>>, Vec<N>)>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>, {
    if nodes.is_empty() {
        return Ok(Vec::new());
    }
    let mut succs_map = HashMap::<N, HashSet<N>>::with_capacity(nodes.len());
    let mut preds_map = HashMap::<N, usize>::with_capacity(nodes.len());
    for node in nodes {
        succs_map.insert(node.clone(), successors(node).into_iter().collect());
        preds_map.insert(node.clone(), 0);
    }
    for succs in succs_map.values() {
        for succ in succs {
            *preds_map.get_mut(succ).unwrap() += 1; // Cannot fail
        }
    }
    let mut groups = Vec::<Vec<N>>::new();
    let mut prev_group: Vec<N> = preds_map
        .iter()
        .filter(|&(_, num_preds)| *num_preds == 0)
        .map(|(node, _)| node.clone())
        .collect();
    if prev_group.is_empty() {
        let remaining: Vec<N> = preds_map.into_keys().collect();
        return Err((Vec::new(), remaining));
    }
    for node in &prev_group {
        preds_map.remove(node);
    }
    while !preds_map.is_empty() {
        let mut next_group = Vec::<N>::new();
        for node in &prev_group {
            for succ in &succs_map[node] { {
                    let num_preds = preds_map.get_mut(succ).unwrap(); // Cannot fail
                    *num_preds -= 1;
                    if *num_preds > 0 {
                        continue;
                    }
                }
                next_group.push(preds_map.remove_entry(succ).unwrap().0); // Cannot fail
            }
        }
        groups.push(mem::replace(&mut prev_group, next_group));
        if prev_group.is_empty() {
            let remaining: Vec<N> = preds_map.into_keys().collect();
            return Err((groups, remaining));
        }
    }
    groups.push(prev_group);
    Ok(groups)
}
