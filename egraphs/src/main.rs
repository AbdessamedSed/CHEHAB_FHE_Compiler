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

    let vector_width: usize = matches
        .value_of("vector_width")
        .unwrap()
        .parse()
        .expect("Number must be a valid usize");

    let benchmark_type: usize = matches
        .value_of("benchmark_type")
        .unwrap()
        .parse()
        .expect("Number must be a valid usize");

    // Get a path string to parse a program.
    let path = matches.value_of("INPUT").unwrap();
    let timeout = env::var("TIMEOUT")
        .ok()
        .and_then(|t| t.parse::<u64>().ok())
        .unwrap_or(300);
    let prog_str = fs::read_to_string(path).expect("Failed to read the input file.");
    eprintln!("the input expression is : {:?}", prog_str);
    let mut prog_str = prog_str.trim().to_string(); // Trim any leading/trailing whitespace

     // if the benchmark is unstrucutred (10), we remove the initial part 'Vec(' which is added for
    // syntactic considerations, else, we let it 

    if benchmark_type == 10 {
        eprintln!("remowing useless chars");
        // Remove "(Vec" at the start
        prog_str = prog_str.strip_prefix("(Vec ").unwrap_or(&prog_str).to_string();
        // Remove the last character (if it exists)
        prog_str.pop();
        prog_str.pop();
    }

    // Print the cleaned-up expression
    eprintln!("The cleaned expression is: {:?}", prog_str);
    let prog = prog_str.parse().unwrap();

    // Record the start time
    let start_time = Instant::now();

    // Some parameters
    let rule_filtering = false;
    let sorting = true;
    let exp_rules = false;

    // Run rewriter
    eprintln!(
        "Running egg with benchmark_type {:?}, timeout {:?}s, width: {:?}, rule_filtering: {:?}, sorting: {:?}, exp-rules: {:?}",
        benchmark_type, timeout, vector_width, rule_filtering, sorting, exp_rules
    );
    let (cost, best) = 
        rules::run(&prog, timeout, benchmark_type, vector_width, rule_filtering, exp_rules, exp_rules);

    // Record the end time
    let duration = start_time.elapsed();

    // Print the results

    println!("{}", best.to_string()); /* Pretty print with width 80 */
    eprintln!("\nCost: {}", cost);
    eprintln!("Time taken: {:?} to finish", duration);
}
