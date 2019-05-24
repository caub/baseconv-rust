use std::collections::HashMap;

pub const DEFAULT_ALPHABET: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_";

pub struct Convertor {
  pub input_map: HashMap<char, u32>,
  pub output_vec: Vec<char>,
}

impl Convertor {
  pub fn new(input_alphabet: &str, output_alphabet: &str) -> Convertor {
    let mut i = 0u32;
    let mut m: HashMap<char, u32> = HashMap::new();
    for c in input_alphabet.chars() {
      m.insert(c, i);
      i = i + 1;
    }

    let v: Vec<char> = output_alphabet.chars().collect();

    Convertor {
      input_map: m,
      output_vec: v,
    }
  }

  pub fn conv(&self, s: &str) -> String {
    self.conv_io(s, self.input_map.len() as u32, self.output_vec.len() as u32)
  }

  // inspired from https://codegolf.stackexchange.com/questions/1620/arbitrary-base-conversion/21672#21672
  pub fn conv_io(&self, s: &str, input_base: u32, output_base: u32) -> String {
    let mut res = vec![];
    let mut nums: Vec<u32> = vec![];
    for c in s.chars() {
      nums.push(self.input_map[&c]);
    }
    while nums.len() > 0 {
      // divide successive powers of output_base
      let mut quotient = vec![];
      let mut remainder = 0u32;
      for n in 0..nums.len() {
        if nums[n] >= input_base {
          continue;
        }
        let accumulator = nums[n] + remainder * input_base;
        let digit = accumulator / output_base; // result is floored, since u32 types
        remainder = accumulator % output_base;
        if quotient.len() > 0 || digit > 0 {
           quotient.push(digit);
        }
      }

      // the remainder of current division is the next rightmost digit
      res.push(remainder);

      // rinse and repeat with next power of output_base
      nums = quotient;
    }
    res.iter().rev()
      .map(|x| self.output_vec[*x as usize])
      .collect::<String>()
  }
}

#[macro_export]
macro_rules! convertor {
  () => (
    Convertor::new(DEFAULT_ALPHABET, DEFAULT_ALPHABET);
  );
  ($input_alphabet:expr) => (
    Convertor::new($input_alphabet, $input_alphabet)
  );
  ($input_alphabet:expr, $output_alphabet:expr) => (
    Convertor::new($input_alphabet, $output_alphabet)
  );
}

#[cfg(test)]
mod tests {
  use super::{Convertor};

  #[test]
  fn it_works() {
    
    let s: Convertor = convertor!("01234567", "0123");
    assert_eq!(s.conv("513027"), "221120113");
  }
}
