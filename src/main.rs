use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn chomp_line(line: String){
	match line.trim_start().chars().collect::<Vec<char>>().as_slice(){
		['c','o','m','m','i','t',..] => {
			println!("We have a commit");
		},
		['A','u','t','h','o','r',..] => {
			println!("We have an Author")
		},
		['D','a','t','e',..] => {
			println!("We have a Date")
		},
		['S','u','b','j','e','c','t',..] => {
			println!("We have a Date")
		},
		[':',..] => {
			println!("We have a file stat")
		},
		[a,..] if a.is_numeric() => {
			println!("We have a line count file stat")
		}
		&[] => {},
		&[_, ..] => {}
	}
}

fn main() {
    
    let args = vec![
        "--no-pager",
		"log",
		"--pretty=format:commit %w(0,5,5) %H%d%nAuthor: %an <%ae> %nDate: %ad%nSubject: %s%nBody: %b%n",
		"--tags",
		"--all",
		"--raw",
		"--date-order",
		"--reverse",
		"--numstat",
		"--date=rfc2822",
    ];

    let stdout = Command::new("git")
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
		.expect("Could not spawn git process.")
        .stdout
        .expect("Could not capture standard output.");

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
			println!("{}", line);
			chomp_line(line);
		});
}
