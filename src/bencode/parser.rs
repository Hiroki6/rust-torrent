use super::code_type::*;

// BenCodeのパーサー
pub trait BenCodeParser {
    // BenCodeにencodeする
    fn encode(&self) -> BenCode;
    // BenCodeをdecodeする
    fn decode(ben_code: BenCode) -> Self;
}

impl BenCodeParser for BenString {
    fn encode(&self) -> BenCode {
        let length = self.get_value().len();
        let code = length.to_string() + ":" + &self.get_value();
        BenCode::create(code)
    }

    fn decode(ben_code: BenCode) -> BenString {
        let length = ben_code.get_value().len();
        let string_value = String::from(&ben_code.get_value()[2..length]);
        BenString::new(string_value)
    }
}
