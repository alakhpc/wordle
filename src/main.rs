mod game;

use clap::Parser;
use game::Wordle;
use std::io::{self, Write};
use wordle::{get_allowed_words, get_random_answer};

use crate::game::Placement;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Word to be guessed
    #[clap(short, long)]
    word: Option<String>,

    /// Number of guesses allowed
    #[clap(short, long, default_value_t = 6)]
    guesses: u32,
}

fn main() {
    let args = Args::parse();

    let word = args.word.unwrap_or_else(get_random_answer);
    let max_guesses = args.guesses;
    let mut game = Wordle::new(word);

    let allowed_words = get_allowed_words();

    loop {
        print!("\nEnter your guess: ");
        io::stdout().flush().expect("Could not flush stdout");
        let mut guess = String::new();
        let line = std::io::stdin().read_line(&mut guess);

        if let Err(e) = line {
            eprintln!("{}", e);
            break;
        }

        let guess = guess.trim().to_ascii_lowercase().to_string();

        if !allowed_words.contains(&guess) {
            println!("'{}' is not a valid guess!", guess);
            continue;
        }

        let result = game.guess(guess);
        println!("{}", result);

        if result.placement.iter().all(|p| p == &Placement::Correct) {
            println!(
                "You win! You guessed '{}' in {} guesses",
                game.word, game.guesses
            );
            break;
        }

        if game.guesses >= max_guesses {
            println!("You lose! The correct word was '{}'", game.word);
            break;
        }
    }

    println!("Thanks for playing!");
}
