use std::env;
use std::fs;
use std::error::Error;
use std::process;



fn main(){


    let args :Vec<String>  = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    let results =  if ignore_case {
        search_case_insensitive(&config.query, &contents)
    }
    else {
        search(&config.query, &contents)
    };


    for line in results {
        println!("{}", line);
    }
    Ok(())
}


struct Config {
    query: String,
    filename: String,
}



impl Config{
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }


        let query = args[1].clone();
        let filename =  args[2].clone();

        Ok(Config { query , filename})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(&query){
            results.push(line);
        }
    }
    results
}

