use std::io;
use std::cmp::Ordering;
// 기본 라이브러리 집합을 prelude라고 부르기도한다?
use rand::Rng;

fn main() {
    // println! 특) 메크로임
    println!("Guess NUM");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("secret num is {secret_number}");

    loop{
        println!("input your guess");

        // let 으로 변수 선언
        // rust는 기본이 immutable로 상수임.
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Fail 2 read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // println!("{}",_);
                continue;
            },
        };
        
        println!("you guess : {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("perpect");
                break;
            }
        }
    }
}
