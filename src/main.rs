use std::io;
use mlatu::{parse, pretty};
use std::process;
use std::io::Read;

fn run_app() -> Result<(), String> {
    let engine = mlatu::Engine::new();
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).map_err(|e| e.to_string())?;
        match parse::terms(&engine, &buffer) {
            Ok(terms) => {
                let new_terms = mlatu::rewrite(&engine, &im::Vector::new(), terms);
                println!("{}", pretty::terms(&engine, new_terms));
                Ok(())
            },
            Err(err) => Err(err)
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
