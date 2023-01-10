use std::io;
use std::cmp::Ordering;
// 기본 라이브러리 집합을 prelude라고 부르기도한다?
use rand::Rng;

fn main() {
    // println! 특) 메크로임
    println!("Guess NUM");

    // rand 를 쓰려면 use도 해야 하지만 Cargo.toml에 추가도 해야함
    // 버전은 알아서 적당히 관리해줌
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

        // parse의 결과를 match를 통해 컨트롤
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // println!("{}",_);
                continue;
            },
        };
        
        println!("you guess : {guess}");

        // cmp의 결과들을 match로 컨트롤
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
