use std::io;
use std::io::prelude::*;
use regex::Regex;


fn main() {
	println!("Enter a string: ");
	let mut input = read_line_iter();
	println!("Your input: {}", input);

	let id_regex = r"^[a-z][a-z]*";
	let re_id = Regex::new(id_regex).unwrap();

	let num_regex = r"^[0-9][0-9]*";
	let re_num = Regex::new(num_regex).unwrap();

	let ws_regex = r"^\s";
	let re_ws = Regex::new(ws_regex).unwrap();

	let mut flag = true;

	let mut current_position;

	while flag {
		let current_string: Vec<char> = input.chars().collect();

		if current_string[0] == '+'{
			println!("PLUS");
			input = input[1..].to_string();
		}else if current_string[0] == '*' {
			println!("TIMES");
			input = input[1..].to_string();
		}else if re_ws.is_match(&input){
			input = input[1..].to_string();
		}else if re_id.is_match(&input){
			current_position = 0;
			let mut current_char = current_string[current_position];

			while current_char.is_alphabetic() {
					current_position += 1;
					if current_position >= current_string.len() {
						//current_position -= 1;
						break;
					}
					current_char = current_string[current_position];
			}
			let lexeme = input[0..current_position].to_string();
			input = input[current_position..].to_string();
			println!("ID {}", lexeme);
		}else if re_num.is_match(&input){
			current_position = 0;
			let mut current_char = current_string[current_position];
			while current_char.is_numeric()  {

					current_position += 1;
					if current_position >= current_string.len() {
						//current_position -= 1;
						break;
					}
					current_char = current_string[current_position];
			}
			let lexeme = input[0..current_position].to_string();
			input = input[current_position..].to_string();
			println!("NUMBER {}", lexeme);
		}else{
			println!("LEXICAL ERROR {}", current_string[0]);
			flag = false;
		}
		if input.len() == 0{
			flag = false;
		}

	}



}

fn read_line_iter() -> String{
	let stdin = io::stdin();
	let input = stdin.lock().lines().next();
	input
		.expect("No lines in buffer")
		.expect("Failed to read line")
		.trim()
		.to_string()

}
