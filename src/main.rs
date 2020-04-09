use blockchainlib::*;
use std::io;

fn main () {
	let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(0,now(),vec![0;32], vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],
        }
        ], difficulty);


    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();


    blockchain.update_with_block(genesis_block).expect("Failed to add gennesis block");

    println!("Here are the blockchain blocks {:?}", &blockchain.blocks);

    let mut sender = String::new();

    let mut sendee = String::new();

    println!("Hello, which account are you?");

    match io::stdin().read_line(&mut sender) {
        Ok(_) => {
            println!("Success, you are: {}", sender);
        },
        Err(e) => println!("Oops! Something went wrong: {}", e)
    }

    println!("Hello {}, who do you want to send that money to? ", sender);

    match io::stdin().read_line(&mut sendee) {
        Ok(_) => {
            println!("Success, you want to send money to: {}", sendee);
        },
        Err(e) => println!("Oops! Something went wrong: {}", e)
    }


    let mut amount_string = String::new();

    let mut amount = 0;

    println!("How many coins do you want to send?");

    match io::stdin().read_line(&mut amount_string) {
        Ok(_) => {
             amount = amount_string
        .trim()
        .parse()
        .expect("Wanted a number");
        },
        Err(e) => println!("Oops! Something went wrong: {}", e)
    }

    println!("This is how much {}", amount);

    let mut second_block = Block::new(1,now(),last_hash, vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],
        }, 
        Transaction {
            inputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],

            outputs: vec![
                transaction::Output {
                    to_addr: sendee.to_owned(),
                    value: amount,
                },
            ],
        } 
        ], difficulty);


    second_block.mine();

    println!("Mined second block {:?}", &second_block);

    blockchain.update_with_block(second_block).expect("Failed to add Second block");

    println!("Here are the blockchain blocks {:?}", &blockchain.blocks[1].transactions[1].outputs);


}
