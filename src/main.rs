use std::{
    fs::File,
    io::{self, Read, Write},
    env,
};
use rand::Rng;
use colored::Colorize;

fn usage() {
    println!("
    Rules:
		- You have 5 guesses
		- You can only guess using 5 letters words
		- You can only guess using lowercase letters
		- Green means correct at the right position
		- Blue means correct at the wrong position
		- Red means incorrect

	Give a filename in argument to have more words
	
	Good luck!
    ");
}

// take a random word from a file
fn random_word_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    // choose a random line
    let lines: Vec<&str> = contents.lines().collect();
    // random number between 0 and the number of lines
    let mut random_line = lines[rand::thread_rng().gen_range(0, lines.len())];
    random_line = random_line.trim();
    if random_line.len() != 5 {
        panic!("The word must be 5 letters long");
    }
    random_line.to_string()
}

// choose a random word from the dictionary
fn random_word() -> String {
    let words = vec![
            "apple",
			"music",
			"thing",
			"child",
			"night",
			"world",
			"house",
			"water",
			"heart",
			"light",
			"sound",
			"place",
			"right",
			"black",
			"white",
			"green",
			"happy",
    ];
    let random_word = words[rand::thread_rng().gen_range(0, words.len())];
    random_word.to_string()
}

fn play(word: &str) -> bool {
    let tries = 5;
    for _ in 0..tries {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess = guess.trim().to_string();
        if guess.len() != 5 {
            println!("The word must be 5 letters long");
            println!("the word is {} letters long", word.len());
            continue;
        }
        // green if correct at the right position
        // blue if correct at the wrong position
        // red if incorrect
        for (i, c) in guess.chars().enumerate() {
            if c == word.chars().nth(i).unwrap() {
                print!("{}", c.to_string().green().bold());
            } else if word.contains(c) {
                print!("{}", c.to_string().blue().bold());
            } else {
                print!("{}", c.to_string().red().bold());
            }
        }
        println!();
        // if the word is guessed, the game is over
        if guess == word {
            println!("You win!");
            return true;
        }
    }
    false
}

fn main() {
    // show how to use the game
    usage();
    // check if a file is given as argument
    let args: Vec<String> = env::args().collect();
    let secret_word = if args.len() == 2 {
        // read the file and return the secret word
        random_word_file(&args[1])
    } else {
        // if no file is given, return a random word
        random_word()
    };
    let is_winner = play(&secret_word);
    if is_winner {
        println!("You won!");
    } else {
        println!("You lost! The word was {}", secret_word);
    }
}