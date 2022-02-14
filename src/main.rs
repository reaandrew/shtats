use std::path::Path;
use shtats::{BufferedOutput, HtmlReporter, run_shtats};

fn main() {
    //git rev-list --all --count
    //
    //  The above will give you the number of commits first so a progress bar can be displayed.

    let mut output = BufferedOutput::new();
    let reporter = HtmlReporter::new();
    match run_shtats(Path::new("."), &mut output, Box::new(reporter)) {
        Ok(_) => {
            println!("{}", output.to_string())
        }
        Err(_) => {
            println!("something went wrong");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use fasthash::city;
    use itertools::Itertools;

    #[test]
    fn test_something() {
        let data = vec!["blue
            green
            red", "blue
            green
            red"];

        let mut values: HashMap<u64, String> = HashMap::new();
        let mut map: HashMap<u64, i32> = HashMap::new();

        for bunch in data {
            let mut buffer = Vec::<&str>::new();
            let lines = bunch.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
            let sets = lines.into_iter().powerset().collect::<Vec<_>>();
            for set in sets.iter().filter(|x| x.len() > 1) {
                let key = city::hash64(set.join(""));
                *map.entry(key).or_insert(0) += 1;
                if map[&key] == 2 {
                    values.insert(key, set.clone().join(","));
                }
            }
        }

        println!("{}", values.len());
        for (key, value) in values {
            let hash = key;
            let files = value;
            let count = map[&hash];

            println!("{} - {}", count, files);
        }
    }
}
