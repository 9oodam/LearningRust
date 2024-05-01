// 추측 게임

// 1) 1에서 100 사이의 임의의 정수를 생성
// 2) 플레이어에게 추측을 입력하라는 메시지가 표시
// 3) 추측이 입력되면 프로그램은 추측이 너무 낮은지 또는 너무 높은지 표시
// 4) 추측이 맞다면 게임은 축하 메시지를 출력하고 종료


use std::io; // 입출력 라이브러리
use rand::Rng; // 난수 생성에 필요한 메서드 정의
use std::cmp::Ordering;

fn main() {
    println!("Game Start!");

    // 난수 생성
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    loop {
        // 사용자 입력
        println!("Input your guess");
        let mut guess = String::new(); // 가변 변수
    
        io::stdin()
        .read_line(&mut guess) // 사용자 입력 받기
        .expect("Failed to read line"); // 잠재적 오류 처리
    
        // let guess: u32 = guess.trim().parse().expect("Please type a number, not a string");
        // 단순 expect 호출에서 표현식으로 전환
        // 유형을 반환하기 때문에 match 사용
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        // 입력값 비교
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
