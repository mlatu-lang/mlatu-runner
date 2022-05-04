use im::Vector;
use mlatu::{parse, pretty, Engine, Rule};
use std::io;
use std::io::Read;
use std::process;

fn run_app() -> Result<(), String> {
    let engine = mlatu::Engine::new();
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|e| e.to_string())?;
    match buffer.rfind('.') {
        Some(index) => {
            let (rules_string, query_string) = buffer.split_at(index + 1);
            match parse::rules(&engine, rules_string) {
                Ok(rules) => query(engine, query_string, rules),
                Err(err) => Err(err),
            }
        }
        None => query(engine, &buffer, Vector::new()),
    }
}

fn query(engine: Engine, query: &str, rules: Vector<Rule>) -> Result<(), String> {
    match parse::terms(&engine, query) {
        Ok(terms) => {
            let new_terms = mlatu::rewrite(&engine, &rules, terms);
            println!("{}", pretty::terms(&engine, new_terms));
            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn main() {
    process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            1
        }
    })
}
