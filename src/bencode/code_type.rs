use std::collections::HashMap;

// BenCode
#[derive(Debug)]
pub struct BenCode {
    value: Vec<u8>
}

impl BenCode {
  pub fn new(parameter: Vec<u8>) -> BenCode {
      BenCode{ value: parameter }
  }

  pub fn getValue(&self) -> &Vec<u8> {
      &self.value
  }
}

#[derive(Debug)]
pub struct BenString {
    value: String
}

#[derive(Debug)]
pub struct BenNum {
    value: i64
}

#[derive(Debug)]
pub struct BenList<T> {
    value: Vec<T>
}

pub struct BenDict<K, V> {
    value: HashMap<K, V>
}

impl BenString {
    fn new(&self, parameter: String) -> BenString {
        BenString{ value: parameter }
    }

    pub fn decode(ben_code: BenCode) -> BenString {
        let length = ben_code.getValue().len();
        let mut bytes_value = Vec::with_capacity(ben_code.getValue().len()-2);
        bytes_value.extend_from_slice(&ben_code.getValue()[2..length]);
        let string_value = bytes_value.iter().map(|&s| s as char).collect::<String>();
        BenString {
            value: string_value
        }
    }
}

impl BenNum {
    fn new(&self, parameter: i64) -> BenNum {
        BenNum { value: parameter }
    }
}

impl<T> BenList<T> {
    fn new(&self, parameter: Vec<T>) -> BenList<T> {
        BenList { value: parameter }
    }
}

impl<K, V> BenDict<K, V> {
    fn new(&self, parameter: HashMap<K, V>) -> BenDict<K, V> {
        BenDict { value: parameter }
    }
}

// BenCodeのパーサー
trait BenCodeParser {
    // BenCodeにencodeする
    //fn encode(&self) -> BenCode;
    // BenCodeをdecodeする
    fn decode(ben_code: BenCode) -> Self;
}

impl BenCodeParser for BenString {
    /*fn encode(&self) -> BenCode {
        let length = self.value.len();
        let mut code = Vec::with_capacity(2 + length);
        code.push(length.to_str());
        code.push(":");
        code.extend_from_slice(&self.value[2..length]);
        BenCode {
            value: code
        }
    }*/
    fn decode(ben_code: BenCode) -> BenString {
        let length = ben_code.getValue().len();
        let mut bytes_value = Vec::with_capacity(ben_code.getValue().len()-2);
        bytes_value.extend_from_slice(&ben_code.getValue()[1..length]);
        //let string_value = String::from_utf8(bytes_value).unwrap();
        let string_value = bytes_value.iter().map(|&s| s as char).collect::<String>();
        BenString {
            value: string_value
        }
    }


}
/*
impl BenCodeParser for BenNum {
    fn encode(&self) -> BenCode {}
    fn decode(ben_code: BenCode) -> BenString {}
}

impl<T> BenCodeParser for BenList<T> {
    fn encode(&self) -> BenCode {}
    fn decode(ben_code: BenCode) -> BenList<T> {}
}
*/

/*
fn decodeBenObject<T>(buffer: &[u8]) -> BenCode {
    let index = 0;
    if(0x30 <= buffer[index] && buffer[index] <= 0x39) {
        return BenString.encode()
    } else if(0x69 <= buffer[index]) {

    }
}*/
