
extern crate rand;

use std::io;
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
        g.draw_board();
        g.computer();
        g.draw_board();
        let winner = g.check_winner();
        if winner != "NONE"{
            if winner == "Tie"{
                println!("Tie");
            }else{
                println!("{} has won!", winner);
                break;
            }
        }
    }
}

impl Game{
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

        let mut number = String::new();
        io::stdin().read_line(&mut number)
            .expect("Could not read line!");
        print!("Choose a number: ");

        let number: u64 = number.trim().parse().expect("Error: not a number");

        let pos = number - 1;
        for _i in 0..9{
            if self.board[pos as usize] != "X" && self.board[pos as usize] != "O"{
                if self.user_x == 1{
                    self.board[pos as usize] = String::from("X");
                }else{
                    self.board[pos as usize] = String::from("O");
                }
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