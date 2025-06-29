extern crate clap;
use clap::{App, Arg};
use egraphslib::*;
use std::time::Instant;

fn main() {
    let matches = App::new("Rewriter")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("vector_width")
                .help("Sets the vector_width")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("benchmark_type")
            .help("Specify the type of the benchamark to select the rules to apply")
            .required(true)
            .index(3)
        )
        .get_matches();

    use std::{env, fs};



    let rule_filtering = false;
    let exp_rules = true;



    let sorting = false;



    // Get a path string to parse a program.
    let path = matches.value_of("INPUT").unwrap();
    let timeout = env::var("TIMEOUT")
        .ok()
        .and_then(|t| t.parse::<u64>().ok())
        .unwrap_or(300);
    let prog_str = fs::read_to_string(path).expect("Failed to read the input file.");
    eprintln!("the input expression is : {:?}", prog_str);
    let prog_str = prog_str.trim().to_string(); 


    // Print the cleaned-up expression
    let prog = prog_str.parse().unwrap();
    let vector_width: usize = matches
        .value_of("vector_width")
        .unwrap()
        .parse()
        .expect("Number must be a valid usize");

    // Record the start time
    let start_time = Instant::now();

    // Run rewriter
    eprintln!(
        "Running egg with timeout {:?}s, width: {:?}",
        timeout, vector_width
    );
    let (_cost, best) = rules::run(&prog, timeout, vector_width, rule_filtering, sorting, exp_rules);

    // Record the end time
    let duration = start_time.elapsed();

    // Print the results

    println!("{}", best.to_string()); /* Pretty print with width 80 */
    println!("{} {}", vector_width, vector_width);
    //eprintln!("\nCost: {}", cost);
    eprintln!("Time taken: {:?} to finish", duration);
}
