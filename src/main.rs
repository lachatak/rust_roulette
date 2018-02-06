#![allow(dead_code)]
#[macro_use]
extern crate scan_rules;

extern crate rand;
//extern crate scheduled_thread_pool;

use rand::Rng;
use std::thread;
use std::time::Duration;
//use std::collections::HashMap;
//use scheduled_thread_pool::ScheduledThreadPool;
use scan_rules::scanner::Word;
//use std::sync::Arc;
//use std::sync::mpsc;
//use std::sync::Mutex;
//use std::sync::{Mutex, Arc};
//use std::fmt;

fn main() {
    #[derive(Debug, Copy, Clone)]
    struct Player {
        name: String,
        amount: u32,
        bets: Vec<Bet>,
    }

    #[derive(Debug, Copy, Clone)]
    struct Bet {
        bet_type: BetType,
        amount: u32,
    };

    #[derive(Debug)]
    struct PlayerBet {
        player: String,
        bet: Bet,
    };

    impl PlayerBet {
        pub fn from(player: String, bet_type: String, amount: u32) -> Result<PlayerBet, String> {
            BetType::from(bet_type)
                .and_then(|bet_type| Ok(PlayerBet { player, bet: Bet { bet_type, amount } }))
        }
    }

    #[derive(Debug, Copy, Clone)]
    enum BetType {
        Number(u32),
        Odd,
        Even,
    };

    impl BetType {
        pub fn from(bet: String) -> Result<BetType, String> {
            match bet.as_ref() {
                "even" => Ok(BetType::Even),
                "odd" => Ok(BetType::Odd),
                _ => match bet.parse() {
                    Ok(n @ 0 ... 36) => Ok(BetType::Number(n)),
                    Ok(n) => Err(format!("{} is not a valid pocket number Bet!", n)),
                    Err(_) => Err(format!("Provided {} cannot be converted to Bet!", bet))
                }
            }
        }
    }

    #[derive(Debug)]
    struct Pocket {
        pocket_number: u32
    }

    impl Pocket {
        pub fn from(pocket_number: u32) -> Result<Pocket, String> {
            match pocket_number {
                0 ... 36 => Ok(Pocket { pocket_number }),
                _ => Err(format!("Invalid pocket number {}", pocket_number)),
            }
        }

        fn is_even(&self) -> bool {
            self.pocket_number % 2 == 0
        }
    }

    #[derive(Debug, Copy, Clone)]
    struct Wheel;

    impl Wheel {
        pub fn spin(&self) -> Result<Pocket, String> {
            let rand = rand::thread_rng().gen_range(0, 37);
            Pocket::from(rand as u32)
        }
    }

    #[derive(Debug)]
    struct Croupier {
        name: String,
        table: Option<Table>,
    };

    impl Croupier {
        pub fn from(name: String) -> Croupier {
            Croupier { name, table: None }
        }

        pub fn open_table(self) -> Croupier {
            match self.table {
                Some(_) => panic!("Table is already open!"),
                None => {
                    let mut table = Table::new();
                    Croupier { table: Some(table), ..self }
                }
            }
        }

        pub fn register_player(self, player: Player) -> Croupier {
            match self.table {
                Some(table) => Croupier { table: Some(table.assign_player(player)), ..self },
                _ => panic!("Table is not open! Cannot register player!")
            }
        }

        pub fn start_game(self) {
            match self.table {
                Some(Table{wheel, players}) =>
//                    let p = self.table.unwrap().players;
                    thread::spawn(move || {
                        println!("{:?}", self);
                        loop {
                            thread::sleep(Duration::from_secs(10u64));
//                            table.players;
                            let _result = wheel.spin().unwrap();
//                            self.announce_winner(result);
                        }
                    }),
                None => panic!("Table is not defined! Cannot start game!")
            };
        }

        fn announce_winner(&self, pocket: Pocket) {
            println!("Winning pocket is {:?}", pocket)
        }
    }

//    impl fmt::Display for Croupier {
//        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//            write!(f, "Starting game with croupier {} with {} players.", self.name, self.table.unwrap().wheel)
//        }
//    }

    #[derive(Debug, Copy, Clone)]
    struct Table {
        wheel: Wheel,
        players: Vec<Player>,
    }

    impl Table {
        pub fn new() -> Table {
            Table { wheel: Wheel {}, players: Vec::new() }
        }

        pub fn assign_player(mut self, player: Player) -> Table {
            self.players.push(player);
            Table{players: self.players, ..self}
        }
    }

    Croupier::from(String::from("Krs"))
        .open_table()
        .register_player(Player { name: String::from("test1"), amount: 100, bets: Vec::new() })
        .register_player(Player { name: String::from("test2"), amount: 200, bets: Vec::new() })
        .start_game();

//    let pool = ScheduledThreadPool::new(5);

//    let _wheel = pool.execute_at_fixed_rate(
//        Duration::from_secs(0),
//        Duration::from_secs(5u64),
//        || {
//            thread::sleep(Duration::from_secs(30u64));
//            let result = Wheel::spin();
//            println!("{:?}", result);
//        },
//    );

    let player_bet = readln! {
        (let player:Word<String>, "|", let bet:Word<String>, "|", let amount) => PlayerBet::from(player, bet, amount),
        (..other) => Err(format!("Invalid bet '{}' !", other))
    };

    println!("{:?}", player_bet);
}