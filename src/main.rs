use std::io;

fn check(answer: Vec<char>, guess: Vec<char>) -> Vec<usize>{
    let mut pos = Vec::new();
    let mut i = 0;
    for c in answer{
        if c == guess[0]{
            pos.push(i);
            
        }
        i+= 1;
    }

    return pos;
}

fn print_board(answer: Vec<char>, solve: &mut Vec<char>, pos: Vec<usize> ){
    
    let mut i = 0;
    while i < pos.len(){
        solve[pos[i] * 2] = answer[pos[i]];
        i+= 1;
    }
    
    if pos.is_empty(){
        println!("x");
    }

    for c in solve{
        print!("{}", c);
    }
    

}

fn main() {
    
    //answer variables
    let answer_str = String::from("foobar");
    let answer: Vec<char> = answer_str.chars().collect();

    //println!("Please guess a letter: ");
    
    //solve variable
    let mut solve = Vec::new();

    //fills solve with underscores for the amount of chars in answer
    let mut i = 0;
    while i < answer.len(){
        solve.push('_');
        solve.push(' ');
        i += 1;
    }

    loop{

        println!(" ");

        println!("Please guess a letter: ");
        //guess variables
        let mut guess_str = String::new();
        io::stdin().read_line(&mut guess_str).expect("failed to read input");
        let guess: Vec<char> = guess_str.chars().collect();

        let pos: Vec<usize> = check(answer.clone(), guess.clone());


        print_board(answer.clone(), &mut solve, pos.clone());
        println!(" ");

        if solve == answer{
            println!("Congrats, you win!");
            break;
        }

    }
}
