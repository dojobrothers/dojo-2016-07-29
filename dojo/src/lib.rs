pub fn fac(n: i32) -> i32 {
  if n <= 1 {
    1
  } else {
    n * fac(n-1)
  }
}

pub fn i2rm(n:i32) -> String {
  if n >= 1000 { 
    "M".to_string() + &i2rm(n - 1000)  
  } else if n >= 900 {
    "CM".to_string() + &i2rm(n - 900)
  } else if n >= 500 {
    "D".to_string() + &i2rm(n - 500)
  } else if n >= 400 {
    "CD".to_string() + &i2rm(n - 400)
  } else if n >= 100 {
    "C".to_string() + &i2rm(n - 100)
  } else if n >= 90 {
    "XC".to_string() + &i2rm(n - 90)
  } else if n >= 50 {
    "L".to_string() + &i2rm(n - 50)
  } else if n >= 40 {
    "XL".to_string() + &i2rm(n - 40)
  } else if n >= 10 {
    "X".to_string() + &i2rm(n - 10)
  } else if n == 9 {
    "IX".to_string()
  } else if n >= 5 {
    "V".to_string() + &i2rm(n - 5)
  } else if n == 4 {
    "IV".to_string()
  } else if n >= 1 {
    "I".to_string() + &i2rm(n-1)
  } else {
    "".to_string()
  }  
}  

#[cfg(test)]
mod tests {
  #[test]
  fn fac() {
    assert!(::fac(0) == 1);
    assert!(::fac(1) == 1);
    assert!(::fac(3) == 6);
    assert!(::fac(5) == 120);
  }
  #[test]
  fn i2rm_test(){
    assert_eq!(::i2rm(1), "I");
    assert_eq!(::i2rm(2), "II");
    assert_eq!(::i2rm(3), "III");
    assert_eq!(::i2rm(4), "IV");
    assert_eq!(::i2rm(5), "V");
    assert_eq!(::i2rm(6), "VI");
    assert_eq!(::i2rm(7), "VII");
    assert_eq!(::i2rm(8), "VIII");
    assert_eq!(::i2rm(9), "IX");
    assert_eq!(::i2rm(10), "X");
    assert_eq!(::i2rm(11), "XI");
    assert_eq!(::i2rm(19), "XIX");
    assert_eq!(::i2rm(20), "XX");
    assert_eq!(::i2rm(38), "XXXVIII");
    assert_eq!(::i2rm(40), "XL");
    assert_eq!(::i2rm(41), "XLI");
    assert_eq!(::i2rm(45), "XLV");
    assert_eq!(::i2rm(50), "L");
    assert_eq!(::i2rm(59), "LIX");
    assert_eq!(::i2rm(63), "LXIII");
    assert_eq!(::i2rm(90), "XC");
    assert_eq!(::i2rm(99), "XCIX");
    assert_eq!(::i2rm(100), "C");
    assert_eq!(::i2rm(101), "CI");
    assert_eq!(::i2rm(105), "CV");
    assert_eq!(::i2rm(400), "CD");
    assert_eq!(::i2rm(401), "CDI");
    assert_eq!(::i2rm(500), "D");
    assert_eq!(::i2rm(574), "DLXXIV");
    assert_eq!(::i2rm(900), "CM");
    assert_eq!(::i2rm(1000), "M");
    assert_eq!(::i2rm(1001), "MI");
    assert_eq!(::i2rm(3999), "MMMCMXCIX");
  }
}

