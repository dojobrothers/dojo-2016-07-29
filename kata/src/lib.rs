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
    fn fac_works() {
        assert!(super::fac(0) == 1);
        assert!(super::fac(1) == 1);
        assert!(super::fac(2) == 2);
        assert!(super::fac(3) == 6);
        assert!(super::fac(5) == 120);
    }
}
