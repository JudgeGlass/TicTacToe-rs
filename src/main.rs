
extern crate rand;

use std::io::{self, Write};
use std::{u32};

use rand::Rng;

struct Game{
    board: [String; 9],
    user_x:u32
}

fn main(){
    let board_chars = [String::from("1"), String::from("2"), String::from("3"), String::from("4"), String::from("5"), String::from("6"), String::from("7"), 
                        String::from("8"), String::from("9")];
    let mut g = Game{board:board_chars, user_x:1};
    g.draw_board();

    loop{
        g.get_pos();
        g.computer();
        g.draw_board();
        let winner = g.check_winner();
        if winner != "NONE"{
            if winner == "Tie"{
                println!("Tie");
            }
            println!("{} has won!", winner);

            print!("Play again?[Y/N]: ");
            io::stdout().flush();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error reading line");

            if input == "Y" || input == "y"{
                g.restart_board();
            }

            break;
        }
    }
}

impl Game{
    pub fn restart_board(&mut self){
        self.board[0] = String::from("1");
        self.board[1] = String::from("2");
        self.board[2] = String::from("3");
        self.board[3] = String::from("4");
        self.board[4] = String::from("5");
        self.board[5] = String::from("6");
        self.board[6] = String::from("7");
        self.board[7] = String::from("8");
        self.board[8] = String::from("9");
    }

    pub fn draw_board(&self){
        println!("");
        println!("{}|{}|{}", self.board[0], self.board[1], self.board[2]);
        println!("-----");
        println!("{}|{}|{}", self.board[3], self.board[4], self.board[5]);
        println!("-----");
        println!("{}|{}|{}", self.board[6], self.board[7], self.board[8]);
        println!("");
    }

    pub fn get_pos(&mut self){
        loop{
            print!("Choose a number: ");
            io::stdout().flush();

            let mut number = String::new();
            io::stdin().read_line(&mut number)
                .expect("Could not read line!");

            let number: u64 = match number.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input!");
                    continue;
                },
            };

            let pos = number - 1;
            if self.board[pos as usize] != "X" && self.board[pos as usize] != "O"{
                if self.user_x == 1{
                    self.board[pos as usize] = String::from("X");
                }else{
                    self.board[pos as usize] = String::from("O");
                }
                break;
            }else{
                println!("Spot taken / invalid input");
            }
        }
    } 

    pub fn make_pos(&self) -> u32{
        let random_pos = rand::thread_rng().gen_range(0, 9);
        random_pos
    }

    pub fn computer(&mut self){
        loop{
            let position = self.make_pos();
            if self.board[position as usize] != "X" && self.board[position as usize] != "O"{
                if self.user_x == 1{
                    self.board[position as usize] = String::from("O");
                }else{
                    self.board[position as usize] = String::from("X");
                }
                break;
            }
        }
    }

    pub fn check_winner(&self) -> String{
        if self.board[0] == self.board[1] && self.board[1] == self.board[2]{
            return self.board[0].to_string()
        }else if self.board[3] == self.board[4] && self.board[4] == self.board[5]{
            return self.board[3].to_string()
        }else if self.board[6] == self.board[7] && self.board[7] == self.board[8]{
            return self.board[6].to_string()
        }else if self.board[0] == self.board[3] && self.board[3] == self.board[6]{
            return self.board[0].to_string()
        }else if self.board[1] == self.board[4] && self.board[4] == self.board[7]{
            return self.board[1].to_string()
        }else if self.board[2] == self.board[5] && self.board[5] == self.board[8]{
            return self.board[2].to_string()
        }else if self.board[0] == self.board[4] && self.board[4] == self.board[8]{
            return self.board[0].to_string()
        }else if self.board[2] == self.board[4] && self.board[4] == self.board[6]{
            return self.board[2].to_string()
        }else if self.check_tie() == 1{
            return String::from("Tie");
        }

        String::from("NONE")
    }

    pub fn check_tie(&self) -> u32{
        let board_count = 9;
        let mut current_count: u32 = 0;
        for i in 0..9{
            if self.board[i] == "X" || self.board[i] == "O"{
                current_count+=1;
            }
        }

        if current_count == board_count{
            return 1
        }

        return 0
    }
}