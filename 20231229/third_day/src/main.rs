// macro = function that writes code (코드를 작성하는 함수)

// 인자 X
macro_rules! oh {
    () => {
        println!("oh!");
    };
}
// 인자 O
macro_rules! wow {
    ($arg:expr) => { // 인자 1개
        println!("{}", $arg);
    };
    ($arg:expr, $arg2:expr) => { // 인자 여러 개
        println!("{}, {}", $arg, $arg2);
    };
}
// 가변 인자
macro_rules! good {
    ($($arg:expr), *) => {
        $(print!("{} ", $arg);)*
    };
}

// fn main() {
//     oh!();

//     wow!("wow");
//     wow!("wow", "wowow");

//     good!(1, 2, 3, 'a', 'b', 'c', "good");
// }



fn give_age() -> i32 {
    26 // rust 에서는 return 생략 가능
}
fn main() {
    // {} 안에 변수를 바로 넣어도 됨, 함수는 안됨
    let name = "Damin";
    println!("My name is {name} and my age is {}", give_age());

    // 문자열 내에서 또다른 변수명을 만들고 대입시켜도 됨
    let num1 = 1;
    let num2 = 2;
    let num3 = 3;
    println!("The number is {n1}, {n2}, {n3}", // The number is 1, 2, 3
        n1 = num1,
        n2 = num2,
        n3 = num3
    );
    // 인덱스 순서로도 가능
    println!("The number is {0}, {1}, {2} and my favorite number is {1}", // The number is 1, 2, 3 and my favorite number is 2
        n1 = num1,
        n2 = num2,
        n3 = num3
    );
}