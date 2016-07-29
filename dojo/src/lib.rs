use std::string::String;

pub fn fac(n: i32) -> i32 {
   if n <= 1 {
     1
   } else {
     n * fac(n-1)
   }
}

pub fn i2rm(n:i32)-> String {
    let string:String = "I";
    
    string  
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
    fn i2rm_test(){
        assert_eq!(::i2rm(1), "I");
        assert_eq!(::i2rm(5), "V");
    }
}

