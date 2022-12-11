use std::{
    collections::{hash_map::RandomState, HashSet},
    fs::read,
};

pub fn load_file(path: &str) -> String {
    let bytes = read(path).unwrap();
    String::from_utf8(bytes).unwrap()
}

pub fn find_first_n_unique(s: &[char], n: usize) -> usize {
    let windows = s.windows(n);

    for (i, window) in windows.enumerate() {
        let set: HashSet<&char, RandomState> = HashSet::from_iter(window);

        if set.len() == n {
            return i + n;
        }
    }

    panic!("no signal");
}
