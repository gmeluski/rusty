fn main() {
  println!("Hello, world!");
}

fn get_string() -> String {
  String::from("Hello")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_string() {
    assert_eq!(get_string(), "Hello")
  }
}
