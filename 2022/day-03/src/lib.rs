use std::{collections::HashSet, fs::read};

use itertools::Itertools;

pub fn get_priority(c: char) -> u8 {
    let c: u8 = c as u8;
    if c <= 90 {
        c - 38
    } else {
        c - 96
    }
}

pub struct Compartment(pub HashSet<char>);

impl From<&str> for Compartment {
    fn from(s: &str) -> Self {
        let mut set = HashSet::new();
        for c in s.chars() {
            set.insert(c);
        }
        Compartment(set)
    }
}

pub fn load_file(path: &str) -> String {
    let bytes = read(path).unwrap();
    String::from_utf8(bytes).unwrap()
}

pub struct Rucksack(pub Compartment, pub Compartment);

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        let med = s.len() / 2;
        Rucksack(Compartment::from(&s[..med]), Compartment::from(&s[med..]))
    }
}

impl Rucksack {
    #[inline]
    fn a(&self) -> &Compartment {
        &self.0
    }

    #[inline]
    fn b(&self) -> &Compartment {
        &self.1
    }

    /// Gets the intersection of chars in this Rucksack's Compartments.
    pub fn get_intersection(&self) -> &char {
        let mut intersection = self.a().0.intersection(&self.b().0);
        intersection.next().unwrap()
    }

    /// Gets the union of chars in this Rucksack's Compartments.
    pub fn get_union(&self) -> HashSet<&char> {
        self.a().0.union(&self.b().0).collect()
    }
}

pub fn load_rucksacks(s: &str) -> Vec<Rucksack> {
    s.lines()
        .into_iter()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(Rucksack::from(line))
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod test {
    use std::iter::FromIterator;

    use super::*;

    #[test]
    fn test_get_priority() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('Z'), 52);
    }

    #[test]
    fn test_compartment_from_str() {
        assert_eq!(
            Compartment::from("abc").0,
            HashSet::from_iter(vec!['a', 'b', 'c'])
        );
        assert_eq!(
            Compartment::from("abac").0,
            HashSet::from_iter(vec!['a', 'b', 'c'])
        );
    }
}
