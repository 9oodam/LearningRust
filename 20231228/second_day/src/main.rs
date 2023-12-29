// char & string
/*
fn main() {
    println!("Hello, world!");
    // println!('Hello, world!'); // string은 무조건 "" 안에

    // char은 '' 안에
    let char1 = 'A';
    let char2 = ' '; // space
    let char3 = 'ㄱ'; // unicode 호환
    let char4 = '🤧'; // emoji 호환
    // char = 4 bytes (존재하는 모든 문자 가능)


    // casting = simple type change using 'as'
    let num1: u16 = 8;
    let num2: u8 = 10;
    let result = num1 + num2 as u16;


    // ASCII 255개이기 때문에 u8 호환해서 안전하게 사용 가능
    let ascii1 = 'a' as u8;
    println!("My number is {}", ascii1); // My number is 97
    let ascii2 = 100;
    println!("My char is {}", ascii2 as u8 as char); // My char is d


    // char과 string 크기 차이
    // char = 4 bytes
    // string = 1~4 bytes per char
    println!("{}", std::mem::size_of::<char>()); // 4
    println!("{}", "a".len()); // 1
    println!("{}", "ㄱ".len()); // 3
    println!("{}", "😕".len()); // 4

    let slice1 = "hello";
    println!("{} bytes, {} chars", slice1.len(), slice1.chars().count()); // 5 bytes, 5 chars
    let slice2 = "안녕!";
    println!("{} bytes, {} chars", slice2.len(), slice2.chars().count()); // 7 bytes, 3 chars
}
*/


// float
/* 
fn main() {
    let big_num = 1_000_000_000_u64; // 보기 좋게 언더바 넣어도 됨 (컴파일러가 전부 무시)

    // float 종류는 2가지, 통상적으로 f64를 씀
    // f32 (4 bytes) / f64 (8 bytes)
    let float = 9.5; // f64
    let num = 5;  // i32
    println!("{}", float as i32 + num); // 14 (i32로 변환하면 소수점 버리기)
    println!("{}", float + num as f64); // 14.5
}
*/
