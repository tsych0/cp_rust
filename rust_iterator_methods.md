# Rust Iterator & Slice Methods - Hidden Gems

## Slice Methods (work on `&[T]`, `Vec<T>`, etc.)

### Windowing & Chunking
- **`windows(n)`** - Overlapping windows of size n: `[1,2,3,4].windows(2)` → `[[1,2], [2,3], [3,4]]`
- **`chunks(n)`** - Non-overlapping chunks: `[1,2,3,4,5].chunks(2)` → `[[1,2], [3,4], [5]]`
- **`chunks_exact(n)`** - Only complete chunks: `[1,2,3,4,5].chunks_exact(2)` → `[[1,2], [3,4]]`
- **`rchunks(n)`** - Chunks from the right: `[1,2,3,4,5].rchunks(2)` → `[[4,5], [2,3], [1]]`
- **`split_at(index)`** - Split at index: `[1,2,3,4].split_at(2)` → `([1,2], [3,4])`

### Splitting
- **`split(predicate)`** - Split by predicate: `[1,0,2,0,3].split(|&x| x == 0)` → `[[1], [2], [3]]`
- **`split_inclusive(predicate)`** - Split including delimiter
- **`splitn(n, predicate)`** - Split at most n times
- **`rsplit(predicate)`** - Split from right
- **`split_first()`** - `[1,2,3]` → `Some((1, [2,3]))`
- **`split_last()`** - `[1,2,3]` → `Some((3, [1,2]))`

### Searching & Finding
- **`binary_search(item)`** - Binary search (requires sorted slice)
- **`binary_search_by(comparator)`** - Binary search with custom comparison
- **`binary_search_by_key(key, extractor)`** - Binary search by key
- **`partition_point(predicate)`** - Find partition point in sorted data
- **`starts_with(needle)`** / **`ends_with(needle)`** - Check prefix/suffix

### Rotation & Swapping
- **`rotate_left(n)`** / **`rotate_right(n)`** - Rotate elements
- **`swap(i, j)`** - Swap two elements
- **`reverse()`** - Reverse in place

## Iterator Methods

### Positioning & Indexing  
- **`enumerate()`** - Add indices: `['a','b'].enumerate()` → `[(0,'a'), (1,'b')]`
- **`position(predicate)`** - Find first index matching predicate
- **`rposition(predicate)`** - Find last index matching predicate

### Zipping & Pairing
- **`zip(other)`** - Pair with another iterator
- **`zip_longest(other)`** - Zip but continue until both exhausted (external crate)
- **`intersperse(separator)`** - Insert separator between elements

### Grouping & Batching
- **`chunk_by(predicate)`** - Group consecutive elements by predicate (nightly)
- **`group_by(key_fn)`** - Group consecutive equal elements (itertools crate)
- **`batching(batch_fn)`** - Custom batching logic (itertools crate)

### Taking & Skipping
- **`take_while(predicate)`** - Take while condition is true
- **`skip_while(predicate)`** - Skip while condition is true
- **`step_by(n)`** - Take every nth element

### Comparison & Ordering
- **`cmp(other)`** - Lexicographic comparison
- **`partial_cmp(other)`** - Partial comparison
- **`eq(other)`** - Element-wise equality
- **`lt(other)` / `le(other)` / `gt(other)` / `ge(other)`** - Lexicographic ordering

### Advanced Iteration
- **`cycle()`** - Repeat iterator infinitely
- **`chain(other)`** - Concatenate iterators
- **`flatten()`** - Flatten nested iterables
- **`flat_map(fn)`** - Map then flatten
- **`scan(initial, fn)`** - Like fold but emits intermediate values
- **`map_windows(fn)`** - Map over sliding windows (nightly)

### Inspection & Side Effects
- **`inspect(fn)`** - Peek at values without consuming (great for debugging)
- **`for_each(fn)`** - Execute closure on each element (consuming)

### Collection & Grouping
- **`partition(predicate)`** - Split into two collections: `(true_items, false_items)`
- **`unzip()`** - Split pairs into two collections
- **`collect_into(collection)`** - Collect into existing collection

### Reduction & Aggregation
- **`reduce(fn)`** - Like fold but uses first element as initial value
- **`fold_first(fn)`** - Alias for reduce
- **`try_fold(initial, fn)`** - Fold that can short-circuit on error
- **`try_reduce(fn)`** - Reduce that can short-circuit on error

### Boolean Operations
- **`all(predicate)`** - Check if all elements match
- **`any(predicate)`** - Check if any element matches

### Finding & Searching
- **`find_map(fn)`** - Find first `Some` result from mapping function
- **`max_by(comparator)`** / **`min_by(comparator)`** - Custom comparison
- **`max_by_key(key_fn)`** / **`min_by_key(key_fn)`** - Compare by extracted key

## String-Specific Methods

### Splitting & Parsing
- **`lines()`** - Split by newlines
- **`split_whitespace()`** - Split by any whitespace
- **`split_ascii_whitespace()`** - Split by ASCII whitespace only
- **`split_terminator(pattern)`** - Split including empty trailing substring
- **`rsplit_terminator(pattern)`** - Right split with terminator
- **`splitn(n, pattern)`** / **`rsplitn(n, pattern)`** - Limited splits

### Matching & Finding
- **`matches(pattern)`** - Iterator over all matches
- **`rmatch_indices(pattern)`** - Match indices from right
- **`match_indices(pattern)`** - Iterator over `(index, match)` pairs

## Range Methods
- **`(start..end).step_by(n)`** - Step through range
- **`(start..=end)`** - Inclusive range

## Examples in Action

```rust
// Sliding window average
let averages: Vec<f64> = numbers
    .windows(3)
    .map(|w| w.iter().sum::<f64>() / 3.0)
    .collect();

// Find runs of identical elements
let text = "aaabbbaaccc";
let runs: Vec<_> = text
    .chars()
    .chunk_by(|&c| c)  // nightly feature
    .map(|(char, group)| (char, group.count()))
    .collect();

// Process in batches
data.chunks(1000)
    .enumerate()
    .for_each(|(i, batch)| {
        println!("Processing batch {}", i);
        process_batch(batch);
    });

// Partition into success/failure
let (successes, failures): (Vec<_>, Vec<_>) = results
    .into_iter()
    .partition(Result::is_ok);

// Find first gap in sorted sequence
let first_gap = sorted_nums
    .windows(2)
    .find_map(|w| (w[1] - w[0] > 1).then_some(w[0] + 1));
```

## Pro Tips

1. **`itertools` crate** adds many more powerful methods like `group_by`, `batching`, `cartesian_product`, etc.
2. **Method chaining** is very powerful - combine these methods for complex operations
3. **`collect::<Vec<_>>()`** when you want type inference for the element type
4. **`inspect()`** is invaluable for debugging iterator chains
5. Many methods have `_by` and `_by_key` variants for custom logic