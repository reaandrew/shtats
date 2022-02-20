use std::collections::HashMap;
use itertools::Itertools;

const MAX_POWERSET: usize = 10;

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
            //let key= metro::hash64(set.join(""));
            let key = meowhash::MeowHasher::hash(set.join("").as_ref()).as_u128();
            //let key = city::hash64(set.join(""));
            *self.map.entry(key).or_insert(0) += 1;
            if self.map[&key] >= self.threshold {
                self.values.insert(key, set.clone().join(","));
            }
        }
    }

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
