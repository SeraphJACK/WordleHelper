use std::io;
use std::io::Write;
use crate::core::WordleWord;

mod core;
mod util;

fn main() {
    let wordlist = util::builtin_wordlist();
    let mut guesser = core::WordleGuesser::new(&wordlist);
    // let humor: WordleWord = "humor".parse().unwrap();
    // let furor: WordleWord = "furor".parse().unwrap();
    // println!("{}", humor.guess(&furor).to_string());
    loop {
        print!("Please input command: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let res = io::stdin().read_line(&mut input);
        if res.is_err() || input.trim().is_empty() {
            break;
        }
        let i = input.trim();
        if i.eq("suggest") {
            guesser.suggest(5).into_iter().for_each(print_suggestion);
        } else if i.eq("update") {
            let mut word = String::new();
            let mut result = String::new();
            print!("Please input word: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut word).unwrap();
            print!("Please input result: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut result).unwrap();
            let r = guesser.update(&word.trim().parse().unwrap(), &result.trim().parse().unwrap());
            println!("Entropy: {:.2}, Actual: {:.2}", r.0, r.1);
        } else {
            println!("Please input suggest/update, enter nothing to exit.");
        }
    }
    println!("Exiting...");
}

fn print_suggestion(p: (&WordleWord, f64)) {
    println!("{} - {:.2} bits", p.0.to_string(), p.1);
}
