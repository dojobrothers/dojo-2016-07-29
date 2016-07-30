pub fn fac(n: i32) -> i32 {
   if n <= 1 {
     1
   } else {
     n * fac(n-1)
   }
}

pub fn i2rm(n:i32) -> String {
    if n >= 5  {
        let x = 
        if n == 10{
            "X".to_string()
        }
        else{
            "V".to_string() 
        }
        
        x + &(i2rm(n - 5))
        
    } else {
        if n == 4 {
            "IV".to_string()
        } else {
            if n>=1{
                "I".to_string() + &(i2rm(n-1))
            } else{
                "".to_string()

            }
            
        }
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
        assert_eq!(::i2rm(10), "X");
        assert_eq!(::i2rm(11), "XI");
    }
}

