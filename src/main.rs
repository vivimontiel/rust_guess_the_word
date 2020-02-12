use cursive::Cursive;
use cursive::view::{Boxable, Identifiable};
use cursive::views::{Dialog, EditView, LinearLayout, TextView, Button};

// imports used for handling files
use std::fs::File;
use std::io::prelude::*;

// imports used for creating random numbers
extern crate rand;
use rand::Rng;

fn main() {
    let mut siv = Cursive::default();
    siv.add_layer(Dialog::text(""));
    next_one(&mut siv);
	siv.run();
}

fn next_one(siv : &mut Cursive){
    siv.pop_layer();
    siv.add_layer(
        Dialog::text("Welcome to our game! Are you strong enough to guess the hidden word?!")
            .title("Guess The Word")
            .button("Start", next_two)
            .button("Quit", Cursive::quit));
}

// removes the welcome layer and calls the game layer
fn next_two(mut siv : &mut Cursive){
    siv.pop_layer();
    start(&mut siv);
}

//gameplay layer
fn start(siv : &mut Cursive){

    siv.pop_layer();
    
    //random number
    let random_index : usize = rand::thread_rng().gen_range(0, 24);

    let selected_word : String = select_word(random_index);
    let selected_hint : String = select_hint(random_index);
    let letters = word_to_list(&selected_word);

    //main gameplay layer
    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("\nEnter word\n").with_name("1"))

                //displays the word hidden by _
                .child(TextView::new(format!("\n{}\n", letters)).with_name("2")) 

                //player inserts their guess here
                .child(EditView::new().on_edit(on_edit).with_name("3")) 

                .child(TextView::new("\n\n").with_name("match"))

                //displays the hint 
                .child(TextView::new("\n").with_name("hint")) 

                //when the button 'Guess' is pressed, it checks whether the word the player entered is a match or not
                .child(Button::new("Guess", move |siv| { 
                    let edit_3 = siv.find_name::<EditView>("3").unwrap();
                    let input = edit_3.get_content();
                    let answer = selected_word.to_string();
                    let rcc = String::clone(&input);

                    //comparison of the player's input with the hidden word
                    let matches = rcc == answer; 

                    //displays if the word is a match or not
                    siv.call_on_name("match", |v: &mut TextView| {
                        v.set_content(
                            if matches { "\n\nCongratulations! It's a match!" } else { "\n\nTry again, not a match"})
                    });       
                }))
                //when the button 'Hint' is pressed, a hint corresponding to the word will appear
                .child(Button::new("Hint", move |siv| {
                    siv.call_on_name("hint", |v: &mut TextView| {
                        v.set_content(format!("Hint : {}\n\n", selected_hint))
                    });     
                }))
                .fixed_height(50)
                .fixed_width(100),
        )
        .title("Guess the hidden word")
        .button("Back to main page", next_one),
    );

	siv.run();

}

fn on_edit(s: &mut Cursive, _content: &str, _cursor: usize) {
    let _text_3 = s.find_name::<EditView>("3").unwrap();
}


// reads and processes the file containing the words to guess
// selects a word in the index given and returns it as a String
fn select_word(index: usize) -> String {
    let mut file = File::open("words.txt").expect("Error opening file!");
    let mut content = String::new();
    
    file.read_to_string(&mut content).expect("An error occured while reading the file!");

    let words: Vec<&str> = content.split('\n').collect();

    return String::from(words[index]);
}

// reads and processes the file containing the words to guess
// selects the hint in the index given and returns it as a String
fn select_hint(index: usize) -> String {
    let mut file = File::open("hints.txt").expect("Error opening file!");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("An error occured while reading the file!");

    let words: Vec<&str> = content.split('\n').collect();

    return String::from(words[index]);
}

// function taking a word (String) on entry and creation a list (Vector) composed of each letter 
fn word_to_list(word: &String) -> String {
    let mut l: Vec<char> = Vec::new();
    let mut word_string = String::from(""); 

    //converts the string into a vector of chars
    for character in word.chars() {
        l.push(character);
    }

    //converts the vector of chars back into string but hidden
    for _letter in l {
        word_string.push(' ');
        word_string.push('_');
        word_string.push(' ');
    }

    // Example: _ _ _ _
    return word_string;
}


