pub trait Matcher {
    fn starts_with_repdig(&self, digit: usize) -> bool;
}

impl Matcher for str {
    // check if the string matches ^(1{digit}|2{digit}|...|e{digit}|f{digit})
    #[inline(always)]
    fn starts_with_repdig(&self, digit: usize) -> bool {
        let repdigit_str = self[0..1].repeat(digit);
        self.starts_with(&repdigit_str)
    }
}

#[cfg(test)]
mod tests {
    use super::Matcher;

    #[test]
    fn execute() {
        assert!("aiueo".starts_with_repdig(1));
        assert!(!"aiueo".starts_with_repdig(2));
        assert!("aaaaa".starts_with_repdig(3));
        assert!(!"aaaaa".starts_with_repdig(10));
        assert!("33333360472fd752f2f5a49725ccd163be15eb73".starts_with_repdig(6));
    }
}
