// code block
// fn give_num(firstNum: i32, secondNum: i32) -> i32 {
//     firstNum * secondNum
// }
// fn give_num_2(firstNum: i16, secondNum: i16) -> i16 {
//     let multiplied_by_ten = {
//         let num = 10;
//         num * firstNum * secondNum
//     };
//     multiplied_by_ten
// }
// fn main() {
//     let result = give_num(2, 5);
//     println!("{}", result); // 10

//     let result2 = give_num_2(3, 4);
//     println!("{}", result2); // 120
// }


// mutability (가변성)
// let x = 5;
// x = 6; // Error!
// let mut x = 5;
// x = 6; // No problem!

// shadowing (같은 변수 이름을 다시 쓰는 것)
// fn double(input: i32) -> i32 {
//     input * 2
// }
// fn triple(input: i32) -> i32 {
//     input * 3
// }
// fn main() {
//     let x = 9;
//     let x = double(x);
//     let x = triple(x);
//     println!("{}", x); // 54
// }

fn main() {
    let variable = 10;
    println!("{}", variable); // 10
    {
        let variable = "ABC";
        println!("{}", variable); // ABC
    }
    println!("{}", variable); // 10
}