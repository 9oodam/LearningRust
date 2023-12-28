// struct Book;

fn main() {
    // 러스트 시작이당
    // println!("Hello, world!");

    let x = 10; // 어떤 형인지 언급이 없으면 기본 i32 (signed integer 32)
    let xx: u8 = 100; // 형 지정은 이렇게, 타입스크립트 같다
                      // u8: 0~255
    let yy: u16 = 50;
    
    // 형이 다른 경우 계산 불가능
    let result = xx + yy; // no implementation for `u8 + u16`

    let _y = 100; // 언더바가 붙으면 에러가 안 보인다는데 정확히 무슨 말인지는?
                  // 언더바 안 붙이면 warning 뜸 => if this is intentional, prefix it with an underscore: `_yy`
}
