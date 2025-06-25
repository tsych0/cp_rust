/// Compute a shortest path using the [iterative deepening depth-first search
/// algorithm](https://en.wikipedia.org/wiki/Iterative_deepening_depth-first_search).
pub fn iddfs<N, FN, IN, FS>(start: N, mut successors: FN, mut success: FS) -> Option<Vec<N>>
where
    N: Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool, {
    let mut path = vec![start];

    let mut current_max_depth: usize = 1;

    loop {
        match step(&mut path, &mut successors, &mut success, current_max_depth) {
            Path::FoundOptimum => return Some(path),
            Path::NoneAtThisDepth => current_max_depth += 1,
            Path::Impossible => return None,
        }
    }
}

#[derive(Debug)]
enum Path {
    FoundOptimum,
    Impossible,
    NoneAtThisDepth,
}

fn step<N, FN, IN, FS>(
    path: &mut Vec<N>,
    successors: &mut FN,
    success: &mut FS,
    depth: usize,
) -> Path
where
    N: Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool, {
    if depth == 0 {
        Path::NoneAtThisDepth
    } else if success(path.last().unwrap()) {
        Path::FoundOptimum
    } else {
        let successors_it = successors(path.last().unwrap());

        let mut best_result = Path::Impossible;

        for n in successors_it {
            if !path.contains(&n) {
                path.push(n);
                match step(path, successors, success, depth - 1) {
                    Path::FoundOptimum => return Path::FoundOptimum,
                    Path::NoneAtThisDepth => best_result = Path::NoneAtThisDepth,
                    Path::Impossible => (),
                }
                path.pop();
            }
        }

        best_result
    }
}
