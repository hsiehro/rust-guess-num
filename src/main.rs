use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    //Computer generates 4 digits number to play
    let key = gen_num();

    //println!("Computer: {:?}", key);
    //Get user input value
    let mut input = user_input();

    //First time determine whether exact match or continue to play
    let mut play_continue = compare(input, &key);

    //Continue to play until all 4 digits are correct
    while play_continue {
        input = user_input();
        play_continue = compare(input, &key);
    }
}

fn gen_num() -> Vec<i32> {
    let mut rng = rand::thread_rng();

    //create a vector
    let mut rand_num = Vec::new();
    //push the first random gen number between 0 to 9
    rand_num.push(rng.gen_range(0, 10));

    //while vector size is not a 4 digits
    while rand_num.len() != 4 {
        let mut found_duplicate = false;
        let new_num = rng.gen_range(0, 10);

        for var in rand_num.iter() {
            if &new_num == var {
                //We have a duplicate!
                found_duplicate = true;
            }
        }

        //if not duplicate, add to vector list
        if !found_duplicate {
            rand_num.push(new_num);
        }
    }

    rand_num
}

fn compare(input: String, key: &Vec<i32>) -> bool {
    println!("Input number is {}", input);
    //println!("Computer number is {:?}", key);
    let mut a_count = 0;
    let mut b_count = 0;

    //get each char from user input number for comparison
    for (num_pos, num) in input.chars().enumerate() {
        //initialize computer random number position from first digit number
        let mut key_pos = 0;
        //iterates each digit value from computer random number
        for val in key.iter() {
            //if each number is equal match and digit position also match, count as A, otherwise, count as B
            //if number no match, move to the next digit position
            if num.to_string() == val.to_string() {
                if num_pos == key_pos {
                    a_count += 1;
                } else {
                    b_count += 1;
                }
            } else {
                key_pos += 1;
            }
        }
    }

    //if A count equals 4, then user wins. Otherwise, continue guessing
    if a_count == 4 {
        println!("The number you are guessing is {:?}", key);
        println!("Great job!");
        false
    } else {
        println!(
            "The result of your input value is {}A and {}B",
            a_count, b_count
        );
        true
    }
}

//Function to get user input of 4 digit numbers, no error check here if user inputs something invalid
fn user_input() -> String {
    let mut input = String::with_capacity(4);
    print!("Enter a 4 digits number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.pop();
    input
}
