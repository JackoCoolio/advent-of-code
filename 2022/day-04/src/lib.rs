use std::fs::read;

use tuple::Map;

pub fn load_file(filename: impl Into<String>) -> String {
    let bytes = read(filename.into()).unwrap();
    String::from_utf8(bytes).unwrap()
}

pub struct Pair {
    pub a: (u32, u32),
    pub b: (u32, u32),
}

/// Determines whether a is fully contained within b.
///
/// Note: assumes intervals are ordered correctly
fn is_interval_fully_contained(a: &(u32, u32), b: &(u32, u32)) -> bool {
    a.0 >= b.0 && a.1 <= b.1
}

impl Pair {
    pub fn is_redundant(&self) -> bool {
        is_interval_fully_contained(&self.a, &self.b) || is_interval_fully_contained(&self.b, &self.a)
    }

    pub fn overlaps(&self) -> bool {
        self.a.0 <= self.b.1 && self.b.0 <= self.a.1
    }
}

impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        let (a, b) = s.split_once(',').unwrap().map(|int| {
            int.split_once('-')
                .unwrap()
                .map(|x| x.parse::<u32>().unwrap())
        });

        Pair { a, b }
    }
}

pub fn load_pairs(data: &str) -> Vec<Pair> {
    data.lines().filter(|l| !l.is_empty()).map(Pair::from).collect()
}
