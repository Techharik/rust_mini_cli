use std::env::args;
use std::fs;

fn main() {
    let args: Vec<String> = args().collect();

    // let search_term = &args[1];
    // let file_path = &args[2];

    let Config { query, file_path } = Config::new(&args);
    println!("searching for a text {:?}", query);

    println!("File in the text {:?}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&*query) {
            results.push(line);
        }
    }

    match results.get(0) {
        Some(cont) => println!("I have the result ---- {}", cont),
        None => println!("I given word is not found"),
    };
    // println!("{:?}------", ans);
}
struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
}

impl Config<'_> {
    fn new<'a>(args: &'a [String]) -> Config<'a> {
        let query = &args[1];
        let file_path = &args[2];
        Config { query, file_path }
    }
}
