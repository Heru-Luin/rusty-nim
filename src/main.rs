extern crate rand_num_gen;


use std::u32;
use rand_num_gen::{Rng, XorShift};
use std::io::stdin;

fn main() {
    // Those may be used later when options will be added

    // Number of sticks at the start of the round
	let nb_sticks = 21;
    // True if human player will play first
	let human_first = true;
    // Current round number
	let mut nb_round = 1;

	// Display main screen
    print_main_screen(nb_sticks, human_first);

    // Play a round of the game
    round(nb_sticks, human_first, nb_round);
    nb_round += 1;
    println!("{}", nb_round);
}

fn round(nb_sticks:u32, human_first:bool, nb_round:u32) {
    println!("~ ROUND {} ~", nb_round);

    let mut curr_nb_sticks:u32 = nb_sticks;
    let mut human_turn:bool = human_first;

    while curr_nb_sticks > 0 {
    	curr_nb_sticks = new_turn(curr_nb_sticks, human_turn, nb_sticks);
    	human_turn = !human_turn;
    }

    print_end_round(!human_turn);
}

fn print_main_screen(nb_sticks:u32, human_first:bool) {
	// Clear screen
    print!("{}[2J", 27 as char);

    println!("~~ A RUSTY NIM GAME ~~");
    println!("");
    println!("~ RULES ~");
    println!("");
    println!("This is a human vs IA board game.");
    println!("");
    println!("- At the start of each round, there are {} sticks on the board.", nb_sticks);
    if human_first {
	    println!("- Human plays first.");
    } else {
	    println!("- IA players first.");
    }
    println!("");
    println!("The player removes 1, 2 or 3 sticks from the board.");
    println!("Once removed, it's the other player's turn.");
    println!("The game continues until there are no more sticks on the board.");
    println!("");
    println!("The player who takes the last stick wins the round.");
    println!("");
}

fn new_turn(mut curr_nb_sticks:u32, human_turn:bool, nb_sticks:u32) -> u32 {
    let mut nb_removed_sticks:u32 = 1;
	println!("");
    if human_turn {
	    println!("~~~~~~~~~~~~~~~ Human turn ~~~~~~~~~~~~~~~");
		println!("");
		print_board(curr_nb_sticks);
		println!("");
		loop {
	        println!("How many sticks would you like to remove ?");
	        let mut line = String::new();
	        stdin().read_line(&mut line);
	        nb_removed_sticks = line.trim().parse::<u32>().unwrap();
	        if nb_removed_sticks > nb_sticks {
	            println!("There are {} sticks left, you cannot remove more of them.", nb_sticks);
	        } else if nb_removed_sticks > 3 {
	            println!("You can only remove between 1 and 3 sticks.");
	        } else if nb_removed_sticks == 0 {
	            println!("You must remove at least 1 stick.");
	        } else {
	        	break;
	        }
		}
		curr_nb_sticks -= nb_removed_sticks;
		if nb_removed_sticks == 1 {
			println!("You remove 1 stick. There are {} sticks left.", curr_nb_sticks);
		} else {
			println!("You remove {} sticks. There are {} sticks left.", nb_removed_sticks, curr_nb_sticks);
		}
    } else {
	    println!("~~~~~~~~~~~~~~~  Bot turn  ~~~~~~~~~~~~~~~");
		println!("");

		if curr_nb_sticks % 4 != 0 {
            nb_removed_sticks = curr_nb_sticks % 4;
        } else {
		    let mut rng = XorShift::new();
		    let mut i = curr_nb_sticks;
		    while i > 0 {
		    	rng.next_u32();
		    	i -= 1;
		    }
            nb_removed_sticks = (rng.next_u32() % 3) + 1;
        }

        curr_nb_sticks -= nb_removed_sticks;

        println!("Computer removes {} sticks, {} sticks left.", nb_removed_sticks, curr_nb_sticks);

    }
	println!("");
	return curr_nb_sticks;
}

fn print_board(curr_nb_sticks:u32) {
	let mut x = 1;
    while x <= curr_nb_sticks {
		print!("|");
        x += 1;
    }
	println!(" <= {} sticks left", curr_nb_sticks);
}

fn print_end_round(human_won:bool) {
    if human_won {
        println!("~~~~~~~~~~~~~~~ HUMAN WINS ~~~~~~~~~~~~~~~");
    } else {
        println!("~~~~~~~~~~~~~~ COMPUTER WON ~~~~~~~~~~~~~~");
    }
}