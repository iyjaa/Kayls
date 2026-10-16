pub fn hitung(a: u32, b: u32) -> u32 {
    a + b
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn hasil_hitung() {
        assert_eq!(hitung(23, 21), 44);
    }
}
