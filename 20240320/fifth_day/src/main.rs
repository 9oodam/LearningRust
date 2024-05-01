// mut
// fn main() {
//     let mut x = 5; 
//     println!("The value of x is: {x}");
//     x = 6; 
//     println!("The value of x is: {x}");

//     // 값이 변할 수는 있으나 유형은 변경 불가능
//     // let mut spaces = "   "; // string
//     // spaces = spaces.len(); // int
// }


// 동일한 이름의 변수 선언
// fn main() {
//     let x = 5;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}"); // 12
//     }
//     println!("The value of x is: {x}"); // 6

//     // 유형도 변경 가능
//     let spaces = "   ";
//     let spaces = spaces.len();
// }



// 튜플
// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0; // 인덱스처럼
//     let six_point_four = x.1;
//     let one = x.2;

//     println!("{five_hundred}, {six_point_four}, {one}"); // 500, 6.4, 1
// }

// 배열
// fn main() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5]; // 5개의 유형이 i32

//     let a = [3; 5]; // 3이 5개
// }

use std::io;

// fn main() {
    // let a = [1, 2, 3, 4, 5];

    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");
// }


// if
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }

// loop
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// for
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}