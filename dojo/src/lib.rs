pub fn fac(n: i32) -> i32 {
   if n <= 1 {
     1
   } else {
     n * fac(n-1)
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
}

