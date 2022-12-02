use std::char;
use std::fs;
use colored::Colorize;
use clap::{Parser};

#[derive(Debug, Parser)]
#[clap(name = "Name: 2B XOR !2B")]
#[clap(author = "Author: Lil' ol' me <lom@something>")]
#[clap(version = "Version: 0.0.0001")]
#[clap(about = "About: Super XOR encryption of strings?", long_about = None)]
#[clap(arg_required_else_help(true))]
struct Cli {
    #[clap(value_parser)]
    /// text to encrypt with XOR cipher
    text: Option<String>,
    #[clap(long, short, value_parser)]
    /// text to encrypt with XOR cipher
    input: Option<String>,
    #[clap(long, short, value_parser, default_value = "KEY")]
    /// key to be used in the XOR cipher
    key: String,
    #[clap(long, short, value_parser)]
    /// file path to file for encrypting
    file: Option<String>,
    #[clap(long, short, value_parser)]
    /// file path for writing encrypted value
    output: Option<String>,
}

fn main() {
    println!("\t\t**********************************************
    \t\t*                                            *
    \t\t*   Welcome to the {} 🤘 🚀 💀  *
    \t\t*                                            *
    \t\t**********************************************\n", "Encryption Zone".bright_green());

    let args = Cli::parse();

    let key = args.key;
    let key_len = key.len();
    let mut input_source = String::new();

    let matched_input = if args.file.is_some() { 
        let file_path = args.file.unwrap();
        input_source = file_path.clone();
        let file_string = fs::read_to_string(file_path).expect("There was an error reading the file.");
        file_string
    } else if args.input.is_some() { 
        input_source.push_str("Command Line Input");
        args.input.unwrap() 
    } else { 
        input_source.push_str("Command Line Input");
        String::from(args.text.as_deref().unwrap())
    };

    let mut encrypted = String::new();

    for (i, c) in matched_input.chars().enumerate() {
        let key_index = i % key_len;
        let text_char_code = c as u32;
        let key_char_code = key.chars().nth(key_index).unwrap() as u32;
        let new_char_code = text_char_code ^ key_char_code;
        let new_char = char::from_u32(new_char_code).unwrap();
    
        encrypted.push(new_char);
    }

    println!("📖 Data to encrypt was taken from: {}\n", input_source.bright_yellow());
    println!("\n🔒 Encrypted value is:\n\n {}", encrypted.bright_green().on_black());

    if args.output.is_some() {
        let output = args.output.unwrap();
        fs::write(output.clone(), encrypted).expect("There was an error writing the file.");
        println!("\n📝 Encrypted data was written to: {}", output.bright_yellow());
    }
}
