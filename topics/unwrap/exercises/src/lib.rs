pub fn parse_and_add(a: &str, b: &str) -> u32 {
  a.parse::<u32>().expect("Failed to parse variable") + b.parse::<u32>().expect("Failed to parse variable")
}

pub fn unwrap_and_add(x: Option<u32>, y: Option<u32>) -> u32 {
    x.unwrap() + y.unwrap()
}