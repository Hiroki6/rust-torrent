mod bencode;

fn main() {

    let mut test_vec = Vec::new();
    test_vec.push(52); // 4
    test_vec.push(58); // :
    test_vec.push(111); // o
    test_vec.push(100); // d
    test_vec.push(101); // e
    test_vec.push(110); // n

    // input string
    let input_string = test_vec.iter().map(|&s| s as char).collect::<String>();
    println!("input bencode: {:?}", input_string);

    // BenCode
    let ben_code = bencode::code_type::BenCode::new(test_vec);

    // BenString
    let ben_string = bencode::code_type::BenString::decode(ben_code);
    println!("decodeed bencode: {:?}", ben_string);
}
