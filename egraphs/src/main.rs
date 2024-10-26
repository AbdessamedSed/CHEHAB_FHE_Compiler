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
            Arg::with_name("rule_filtering")
            .help("Boolean to enable or disable rule filtering before each iteartion when building the e-graph")
            .required(false)
            .index(3)
        )
        .arg(
            Arg::with_name("sorting")
            .help("Sort the eclasses before calculating the cost")
            .required(false)
            .index(4)
        )
        .arg(
            Arg::with_name("exp_rules")
            .help("Boolean to enable or disable the expensive rules")
            .required(false)
            .index(5)
        )
        .get_matches();

    use std::{env, fs};

    // Get a path string to parse a program.
    let path = matches.value_of("INPUT").unwrap();
    let timeout = env::var("TIMEOUT")
        .ok()
        .and_then(|t| t.parse::<u64>().ok())
        .unwrap_or(300);
    let prog_str = fs::read_to_string(path).expect("Failed to read the input file.");
    let prog = prog_str.parse().unwrap();
    let vector_width: usize = matches
        .value_of("vector_width")
        .unwrap()
        .parse()
        .expect("Number must be a valid usize");

    let rule_filtering = matches
        .value_of("rule_filtering")
        .unwrap_or("false")
        .parse::<bool>()
        .expect("rule_filtering must be a valid boolean");

    let sorting = matches
        .value_of("sorting")
        .unwrap_or("false")
        .parse::<bool>()
        .expect("sorting must be a valid boolean");

    let exp_rules = matches
        .value_of("exp-rules")
        .unwrap_or("false")
        .parse::<bool>()
        .expect("exp-rules must be a valid boolean");

    // Record the start time
    let start_time = Instant::now();

    // Run rewriter
    eprintln!(
        "Running egg with timeout {:?}s, width: {:?}, rule_filtering: {:?}, sorting: {:?}, exp-rules: {:?}",
        timeout, vector_width, rule_filtering, sorting, exp_rules
    );
    let (cost, best) = rules::run(&prog, timeout, vector_width, rule_filtering, sorting, exp_rules);

    // Record the end time
    let duration = start_time.elapsed();

    // Print the results
    println!("{}", best.to_string()); /* Pretty print with width 80 */
    eprintln!("\nCost: {}", cost);
    eprintln!("Time taken: {:?} to finish", duration);
}
