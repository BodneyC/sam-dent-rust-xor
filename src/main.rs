use std::char;
use std::fs;
use colored::Colorize;
// BEN: These curlies are unnecessary, depending on your editor/LSP setup, your editor may have
//  told you
//
// In either case, it's *always* a good idea to work with a formatter of some sort, preferably one
//  you haven't customized the ruleset of *too* much. Not only can they help with readability,
//  clean code, etc. but when working with multiple people they can prevent pointless changes: if
//  my editor formats on save and yours doesn't, then most of my commits will include formatting
//  changes in your changes, making change-reviews harder and possibly starting beefs
//
// Particular for Rust where, as discussed, the assosciated toolset is unbeatable, a formatter is
//  built in, just run:
//     cargo fmt
use clap::{Parser};

#[derive(Debug, Parser)]
#[clap(name = "Name: 2B XOR !2B")]
#[clap(author = "Author: Lil' ol' me <lom@something>")]
#[clap(version = "Version: 0.0.0001")]
#[clap(about = "About: Super XOR encryption of strings?", long_about = None)]
#[clap(arg_required_else_help(true))]
struct Cli {
    // BEN: Big fan of having both a positional argument and a switch for input, it's something
    //  usually forgotten but allow the command to be composed in any order which I find useful
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

// BEN: At the minute it's not too long as the program is quite simple but this is becoming quite a
//  long `main`
//
// It's always worth seeing what can be extracted to other functions, the argument against this is
//  usually "but I'd only call the function once" which is true but if we think about this from
//  the perspective of that supply-chain-attack which used a xor cipher, that function *only*
//  performed the xor-cipher, it expected a string input and a key - if you wished to use this
//  logic in another place, you would *then* have to perform the same refactor I'm suggesting now
//
// I've got a composition-ish topic planned but that's in the future :D so can be discussed then
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

    // BEN: This would be one of the chunks extracted to a function. For me, `main` should be:
    //     args = parse_args();
    //     return run_app(args);
    //
    // See: https://softwareengineering.stackexchange.com/questions/85657/why-should-main-be-short
    let matched_input = if args.file.is_some() { 
        // Though I like that you have a file input, particularly it makes shell-based unit testing
        //  easier, but what if my `-f` was a 512GB drive, what does `read_to_string` do with that?
        //
        // This is why the streaming approach is often better when dealing with files out of your
        //  control - traditionally for this you'd `open` the file which would yield a pointer to
        //  your current location in the file, then `read` a chunk into a buffer, when you want to
        //  read more, use the same buffer and lose the previous chunk - the memory footprint is
        //  then only the size of the buffer
        //
        // In Rust (and other high level languages) you'd normally get a buffered-reader which does
        //  exactly what I described above but is less fiddley and usually provides lots of extra
        //  utility you don't need to write yourself, See:
        //     https://doc.rust-lang.org/std/io/struct.BufReader.html#method.with_capacity
        //
        // **NOTE**: As you have multiple input types (which, again, I approve of :D), streaming
        //  the file would require a *big* refactor and is probably not worth doing, the other side
        //  of this is to mitigate the concern mentioned above through input validation:
        //
        //     if fs::metadata(file_path)?.len() > 2000000000 /* or whatever */ { ... }
        let file_path = args.file.unwrap();
        input_source = file_path.clone();
        let file_string = fs::read_to_string(file_path)
            // BEN: This is a "neat-code vs. ux" thing for me. If you had users of this code and
            //  provided the filename of a non-existant or non-readable file, should they recieve a
            //  "file does not exist" message or a "thread 'main' panicked at..." message? As you
            //  cannot guarantee that they're "tech-savy" or whatever, I'd say the friendly message is
            //  safer (this is obviously very nit-picky)
            .expect("There was an error reading the file.");
        file_string
    } else if args.input.is_some() { 
        // BEN: Just for extra clarity, maybe slap a "(--input)" at the end
        input_source.push_str("Command Line Input");
        args.input.unwrap() 
    } else { 
        // BEN: Just for extra clarity, maybe slap a "(positional)" at the end
        input_source.push_str("Command Line Input");
        String::from(args.text.as_deref().unwrap())
    };

    let mut encrypted = String::new();

    for (i, c) in matched_input.chars().enumerate() {
        let key_index = i % key_len;
        // BEN: I would question these as u32s, we're iterating the key and the input byte-by-byte,
        //  and bytes would be u8s (8-bit), in terms of resource-consumption it's kind of a
        //  non-issue: we'd use 8-bytes of memory instead of two to store them and the CPU will
        //  (in-effect) xor them as 64-bit values
        //
        // However, as we're dealing with chars, specifically 8-bit values, it might make sense,
        //  even make it more readable to keep things like that
        let key_char_code = key.chars().nth(key_index).unwrap() as u32;
        let new_char_code = c as u32 ^ key_char_code;
        // BEN: Related to the previous point, this line would just be:
        //     let new_char = new_char_code as char;
        //  probably omitting the line all together
        //
        // This conversion will probably be optimized out by LLVM noticing that they're all 8-bit,
        //  but still
        let new_char = char::from_u32(new_char_code).unwrap();
    
        encrypted.push(new_char);
    }

    println!("📖 Data to encrypt was taken from: {}\n", input_source.bright_yellow());
    // BEN: What if my file is 5GB, are we printing that to the console?
    // 
    // Also, the formatting library mutates the binary to make it printable, this is better for my
    //  console's sanity (note that running `stty sane` through the messy screen will restore
    //  this) but renders the output unusable unless perfect unicode
    println!("\n🔒 Encrypted value is:\n\n {}", encrypted.bright_green().on_black());

    // BEN: Say my --file was 5GB, so something that will fit in memory ignoring that other concern,
    //  but something that will take a while to complete
    //
    // I sit here for two minutes waiting for the program to encode my file, it moves to this line
    //  and I haven't provided a -o flag. The process exits and all that work is lost...
    if args.output.is_some() {
        let output = args.output.unwrap();
        // BEN: Similar to the above, this is a matter of input validation: not only do you need to
        //  check that I've provided an output, but in this case that output needs to be a
        //  writeable file
        fs::write(output.clone(), encrypted).expect("There was an error writing the file.");
        println!("\n📝 Encrypted data was written to: {}", output.bright_yellow());
    }
    // BEN: Perhaps an else clause here could print the output to the console, it would be nonsense
    //  binary but perhaps someone wants to pipe the output into another command for further
    //  processing - this is kind of done with the println but the processing mutates the binary
    //  and renders it unusable (as mentioned)
}

// BEN: Unit tests are great for sanity:
//   https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
