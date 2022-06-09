use std::collections::HashMap;
use itertools::Itertools;

const MAX_POWERSET: usize = 10;

#[allow(dead_code)]
pub struct DuplicateInfo {
    pub duplicate: String,
    pub count: i32,
}

#[derive(Clone, PartialEq)]
pub struct DuplicateDetector {
    values: HashMap<u128, String>,
    map: HashMap<u128, i32>,
    threshold: i32,
}

impl Default for DuplicateDetector{
    fn default() -> Self {
        return DuplicateDetector::new(10);
    }
}

impl DuplicateDetector {
    pub fn new(threshold: i32) -> Self {
        return Self {
            map: HashMap::new(),
            values: HashMap::new(),
            threshold
        };
    }

    pub fn add(&mut self, data: Vec<&str>) {
        let sets = data.into_iter().take(MAX_POWERSET).powerset().collect::<Vec<_>>();
        for set in sets.iter().filter(|x| x.len() > 1) {
            let key = meowhash::MeowHasher::hash(set.join("").as_ref()).as_u128();
            *self.map.entry(key).or_insert(0) += 1;
            if self.map[&key] >= self.threshold {
                self.values.insert(key, set.clone().join(","));
            }
        }
    }

    #[allow(dead_code)]
    pub fn results(&mut self) -> Vec<DuplicateInfo> {
        let mut return_data = Vec::<DuplicateInfo>::new();

        for (key, value) in self.values.clone() {
            let count = self.map[&key];
            return_data.push(DuplicateInfo {
                duplicate: value,
                count: count,
            });
        }

        return return_data;
    }

}


#[cfg(test)]
mod tests {
    use crate::duplicates::DuplicateDetector;

    #[test]
    fn test_something() {
        let data = vec!["blue
            green
            red
            purple", "blue
            green
            red
            purple", "red
            purple"];


        let mut dup_detector = DuplicateDetector::new(3);
        for bunch in data {
            let lines = bunch.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
            dup_detector.add(lines);
        }

        // for item in dup_detector.results() {
        //     println!("BING {}: {}", item.count, item.duplicate)
        // }
    }
}

