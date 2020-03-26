use blockchainlib::*;

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

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();


    blockchain.update_with_block(genesis_block).expect("Failed to add gennesis block");

    let mut second_block = Block::new(1,now(),vec![0;32], vec![
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


    second_block.mine();

    println!("Mined second block {:?}", &second_block);


    blockchain.update_with_block(second_block).expect("Failed to add Second block");



}