/*
// BenCodeのパーサー
trait BenCodeParser {
    // BenCodeにencodeする
    fn encode(&self) -> BenCode;
    // BenCodeをdecodeする
    fn decode(ben_code: BenCode) -> Self;
}

impl BenCodeParser for BenString {
    pub fn encode(&self) -> BenCode {
        let length = self.value.len();
        let mut code = Vec::with_capacity(2 + length);
        code.push(length.to_str());
        code.push(":");
        code.extend_from_slice(&self.value[2..length]);
        BenCode {
            value: code
        }
    }
    pub fn decode(ben_code: BenCode) -> BenString {
        let string_value = Vec::with_capacity(ben_code.getValue().len()-2);
        string_value.extend_from_slice(&string_value);
        BenString {
            value: string_value
        }
    }
}

impl BenCodeParser for BenNum {
    fn encode(&self) -> BenCode {}
    fn decode(ben_code: BenCode) -> BenString {}
}

impl BenCodeParser for BenList<T> {
    fn encode(&self) -> BenCode {}
    fn decode(ben_code: BenCode) -> BenList<T> {}
}

impl BenCodeParser for BenNum {
    fn encode(&self) -> BenCode {}
    fn decode(ben_code: BenCode) -> BenNum {}
}*/
