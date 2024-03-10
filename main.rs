use std::io::{self};
use rand::Rng;

fn check_already_guessed(guess: &String, letters_g: &Vec<String>) -> bool {
    letters_g.contains(&guess.to_lowercase())
}

fn main() 
{
    let mut rng = rand::thread_rng();
    let mut strikes = 0;
    let mut letters_c = 0;
    let mut letters_g = vec![];
    let words = ["Apple", "Banana", "Orange", "Strawberry"];
    let hangman_parts = ["O", "|", "/","\\", "/", "\\"];
    let random_number = rng.gen_range(0..=(words.len()-1));
    let answer = words[random_number];
    let length = answer.len();
    let mut guess = String::new();
    let mut display: Vec<char> = vec!['_'; length];

    println!(" +----+");
    println!(" |    |");
    println!(" |  ");
    println!(" |  ");
    println!(" |  ");
    println!("_|__");
    println!("The word has {} letters", length);

    while strikes < 6 && letters_c < length
    {
        guess.clear();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        check_already_guessed(&guess, &letters_g);
        if check_already_guessed(&guess, &letters_g) 
        {
            println!("You have already guessed this letter!");
            println!("Please guess a different letter");
            continue;
        }

        letters_g.push(guess.clone());

        guess = guess.trim().to_string();
        let mut found = false;
        for (index, letter) in answer.chars().enumerate() {
            if letter.to_lowercase().to_string() == guess {
                display[index] = letter;
                letters_c += 1;
                found = true;
            }
        }

        if !found 
        {
            strikes += 1;
            if strikes == 1
            {
                println!(" +----+");
                println!(" |    |");
                println!(" |    {}", hangman_parts[0]);
                println!(" |  ");
                println!(" |  ");
                println!("_|__");
                println!("Enter guess:");
            }
            else if strikes == 2
            {
                println!(" +----+");
                println!(" |    |");
                println!(" |    {}", hangman_parts[0]);
                println!(" |    {}", hangman_parts[1]);
                println!(" |  ");
                println!("_|__");
                println!("Enter guess:");
            }
            else if strikes == 3
            {
                println!(" +----+");
                println!(" |    |");
                println!(" |    {}", hangman_parts[0]);
                println!(" |   {}{}", hangman_parts[2], hangman_parts[1]);
                println!(" |  ");
                println!("_|__");
                println!("Enter guess:");
            }
            else if strikes == 4
            {
                println!(" +----+");
                println!(" |    |");
                println!(" |    {}", hangman_parts[0]);
                println!(" |   {}{}{}", hangman_parts[2], hangman_parts[1], hangman_parts[3]);
                println!(" |  ");
                println!("_|__");
                println!("Enter guess:");
            }
            else if strikes == 5
            {
                println!(" +----+");
                println!(" |    |");
                println!(" |    {}", hangman_parts[0]);
                println!(" |   {}{}{}", hangman_parts[2], hangman_parts[1], hangman_parts[3]);
                println!(" |   {}", hangman_parts[4]);
                println!("_|__");
                println!("Enter guess:");
            }
            else if strikes == 6
            {
                println!(" +----+");
                println!(" |    |");
                println!(" |    {}", hangman_parts[0]);
                println!(" |   {}{}{}", hangman_parts[2], hangman_parts[1], hangman_parts[3]);
                println!(" |   {} {}", hangman_parts[4], hangman_parts[5]);
                println!("_|__");
                println!("Enter guess:");
            }
        }
        println!("Current progress: {}", display.iter().collect::<String>());
    }

    if letters_c == length {
        let mut exit = String::new();
        println!("You win! The word was {} :D", answer);
        println!("Press enter to exit");
        io::stdin().read_line(&mut exit).expect("Failed to read line");
    } else {
        let mut exit = String::new();
        println!("You lose! The word was {} D:", answer);
        println!("Press enter to exit");
        io::stdin().read_line(&mut exit).expect("Failed to read line");
    }
}