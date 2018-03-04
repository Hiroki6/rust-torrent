use std::collections::HashMap;
use std::str;

// BenCode
#[derive(Debug)]
pub struct BenCode {
    value: String
}

impl BenCode {
  pub fn new(parameter: &[u8]) -> BenCode {
      //BenCode{ value: parameter.iter().map(|&s| s as char).collect::<String>()}
      BenCode{ value: String::from(str::from_utf8(parameter).unwrap()) }
  }

  pub fn create(parameter: String) -> BenCode {
      BenCode{ value: parameter }
  }

  pub fn get_value(&self) -> &str {
      &self.value
  }
}

#[derive(Debug)]
pub struct BenString {
    value: String
}

impl BenString {
    pub fn new(parameter: String) -> BenString {
        BenString{ value: parameter }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug)]
pub struct BenNum {
    value: i64
}

impl BenNum {
    fn new(&self, parameter: i64) -> BenNum {
        BenNum { value: parameter }
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }
}

#[derive(Debug)]
pub struct BenList<T> {
    value: Vec<T>
}

impl<T> BenList<T> {
    fn new(&self, parameter: Vec<T>) -> BenList<T> {
        BenList { value: parameter }
    }

    pub fn getValue(&self) -> &Vec<T> {
        &self.value
    }
}

pub struct BenDict<K, V> {
    value: HashMap<K, V>
}

impl<K, V> BenDict<K, V> {
    fn new(&self, parameter: HashMap<K, V>) -> BenDict<K, V> {
        BenDict { value: parameter }
    }
}

/*
fn decodeBenObject<T>(buffer: &[u8]) -> BenCode {
    let index = 0;
    if(0x30 <= buffer[index] && buffer[index] <= 0x39) {
        return BenString.encode()
    } else if(0x69 <= buffer[index]) {

    }
}*/
