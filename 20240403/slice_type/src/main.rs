fn main() {
    let word = String::from("abc def");
    let word_length = first_word(&word);
    println!("{word}, {word_length}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // 공백을 찾으면 위치 반환
        }
    }

    s.len() // 공백이 없으면 문자열 길이 반환
}