mod bencode;
use bencode::parser::BenCodeParser;

fn main() {

    let input_string = String::from("oden");
    println!("input string: {:?}", input_string); // 4:oden
    // BenCode
    //let ben_code = bencode::code_type::BenCode::new(input_string.as_bytes());
    let ben_string = bencode::code_type::BenString::new(input_string);
    let ben_code = bencode::code_type::BenString::encode(&ben_string);
    println!("bencode: {:?}", ben_code); // 4:oden

    // BenString
    let decodeed_ben_string = bencode::code_type::BenString::decode(ben_code);
    println!("decodeed bencode: {:?}", ben_string); // oden
}
