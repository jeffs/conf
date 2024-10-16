// See also: [`evcxr/COMMON.md`](https://github.com/evcxr/evcxr/blob/main/COMMON.md)

use std::collections::HashSet;
use std::hash::Hash;

fn set<T: Eq + Hash>(xs: impl IntoIterator<Item = T>) -> HashSet<T> {
    xs.into_iter().collect()
}

fn vec<T>(xs: impl IntoIterator<Item = T>) -> Vec<T> {
    xs.into_iter().collect()
}

fn vec_str<S: ToString>(xs: impl IntoIterator<Item = S>) -> Vec<String> {
    xs.into_iter().map(|s| s.to_string()).collect()
}
