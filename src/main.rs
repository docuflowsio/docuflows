mod parser;
mod flow_reader;
mod mermaid;

use clap::{Arg, Command};
use std::path::PathBuf;

fn main() {
    let matches = Command::new("docuflows")
        .version("0.1.0")
        .about("Visualize feature flows in codebases with Docuflows")
        .subcommand(
            Command::new("diagram")
                .short_flag('d')
                .long_flag("diagram")
                .about("Generate a Mermaid.js diagram from a .docuflow file")
                .arg(
                    Arg::new("flow")
                        .help("Name of the .docuflow file (without extension)")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("parse")
                .short_flag('p')
                .long_flag("parse")
                .about("Parse source directory and list all function names")
                .arg(
                    Arg::new("path")
                        .help("Source path")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(diagram_matches) = matches.subcommand_matches("diagram") {
        let flow_name = diagram_matches.get_one::<String>("flow").unwrap();
        let flow_path = PathBuf::from(".docuflows").join(format!("{}.docuflow", flow_name));
        let flow = flow_reader::load_flow_file(&flow_path).expect("Failed to load flow file");
        let mermaid_output = mermaid::generate_mermaid_diagram(&flow);
        println!("{}", mermaid_output);
    } else if let Some(parse_matches) = matches.subcommand_matches("parse") {
        let path = parse_matches.get_one::<String>("path").unwrap();
        let functions = parser::extract_function_names_from_dir(path);
        for func in functions {
            println!("{}", func);
        }
    }
}