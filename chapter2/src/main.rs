use std::io;
// 기본 라이브러리 집합을 prelude라고 부르기도한다?

fn main() {
    // println! 특) 메크로임
    println!("Guess NUM");

    println!("input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Fail 2 read line");

    println!("you guess : {guess}");


}
