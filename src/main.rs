use std::{
    io::{self, Write},
    process,
};


pub mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choise = String::new();

    get_input("Введите адрес майнера: ", &mut miner_addr);
    get_input("Сложность: ", &mut difficulty);

    let dif = difficulty
        .trim()
        .parse::<u32>()
        .unwrap();
    
    println!("Генерация genesis блока");

    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), dif);

    loop {
        println!("*****МЕНЮ*****");
        println!("1. НОВАЯ ТРАНЗАКЦИЯ");
        println!("2. ДОБЫТЬ БЛОК");
        println!("3. ИЗМЕНИТЬ СЛОЖНОСТЬ");
        println!("4. ХАЛВИНГ");
        println!("0. ВЫХОД");
        println!("**************");

        println!("ВВЕДИТЕ ВАШ ВЫБОР: ");
        io::stdout().flush();
        choise.clear();
        io::stdin().read_line(&mut choise);
        println!("");

        match choise.trim().parse().unwrap() {
            0 => process::exit(0),
            1 => {
                let mut sender = String::new();
                let mut reciever = String::new();
                let mut amount = String::new();

                get_input("Адрес отправителя: ", &mut sender);
                get_input("Адрес получателя: ", &mut reciever);
                get_input("Количество: ", &mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    reciever.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );
                match res {
                    true => println!("Ok"),
                    false => println!("Error"),
                }
            }
            2 => {
                println!("Генерация нового блока");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Ok"),
                    false => println!("Error"),
                }
            },
            3 => {
                let mut dificulty = String::new();
                get_input("Введите сложность: ", &mut dificulty);

                let res = chain.update_difficulty(dificulty.trim().parse().unwrap());
                match res {
                    true => println!("Ok"),
                    false => println!("Error"),
                }
            },
            _ => {
                println!("Не валидное значение");
                continue;
            }
        }
    }
}

fn get_input(ask_message: &str, s: &mut String) {
    print!("{}", ask_message);
    io::stdout().flush();
    io::stdin().read_line(s);
}
