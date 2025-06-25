use std::collections::HashSet;
use std::hash::Hash;

/// Find all maximal cliques is an undirected graph.
pub fn maximal_cliques_collect<N, FN, IN>(vertices: IN, connected: &mut FN) -> Vec<HashSet<N>>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N, &N) -> bool,
    IN: IntoIterator<Item = N>,
{
    let mut result = Vec::new();
    let mut consumer = |n: &HashSet<N>| result.push(n.to_owned());
    let mut remaining_nodes: HashSet<N> = vertices.into_iter().collect::<HashSet<_>>();
    bron_kerbosch(
        connected,
        &HashSet::new(),
        &mut remaining_nodes,
        &mut HashSet::new(),
        &mut consumer,
    );
    result
}

/// Find all maximal cliques is an undirected graph.
pub fn maximal_cliques<N, FN, IN, CO>(vertices: IN, connected: &mut FN, consumer: &mut CO)
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N, &N) -> bool,
    IN: IntoIterator<Item = N>,
    CO: FnMut(&HashSet<N>),
{
    let mut remaining_nodes: HashSet<N> = vertices.into_iter().collect();
    bron_kerbosch(
        connected,
        &HashSet::new(),
        &mut remaining_nodes,
        &mut HashSet::new(),
        consumer,
    );
}

fn bron_kerbosch<N, FN, CO>(
    connected: &mut FN,
    potential_clique: &HashSet<N>,
    remaining_nodes: &mut HashSet<N>,
    skip_nodes: &mut HashSet<N>,
    consumer: &mut CO,
) where
    N: Eq + Hash + Clone,
    FN: FnMut(&N, &N) -> bool,
    CO: FnMut(&HashSet<N>),
{
    if remaining_nodes.is_empty() && skip_nodes.is_empty() {
        consumer(potential_clique);
        return;
    }
    let nodes_to_check = remaining_nodes.clone();
    for node in &nodes_to_check {
        let mut new_potential_clique = potential_clique.clone();
        new_potential_clique.insert(node.to_owned());

        let mut new_remaining_nodes: HashSet<N> = remaining_nodes
            .iter()
            .filter(|n| *n != node && connected(node, n))
            .cloned()
            .collect();

        let mut new_skip_list: HashSet<N> = skip_nodes
            .iter()
            .filter(|n| *n != node && connected(node, n))
            .cloned()
            .collect();
        bron_kerbosch(
            connected,
            &new_potential_clique,
            &mut new_remaining_nodes,
            &mut new_skip_list,
            consumer,
        );

        // We're done considering this node. If there was a way to form a clique with it, we
        // already discovered its maximal clique is the recursive call above.  So, go ahead
        // and remove it from the list of remaining nodes and add it to the skip list.
        remaining_nodes.remove(node);
        skip_nodes.insert(node.to_owned());
    }
}
