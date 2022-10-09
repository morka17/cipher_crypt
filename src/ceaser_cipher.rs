pub struct Ceaser {}

impl Ceaser {

    pub fn encrypt2(cipher: &str, shift: u8) -> String  {
        cipher
        .as_bytes()
        .iter().map(|c|{

            if c.is_ascii_alphabetic(){
                let start = if c.is_ascii_lowercase() {
                    b'a'
                }else {
                    b'A'
                };

                //Keep track of character range 
                (start + (c + shift - start)  % 26) as char
            }else {
                *c as char
            }
        }).collect()
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(Ceaser::encrypt2("", 13), "");
    }

    #[test]
    fn caesar_rot_13() {
        assert_eq!(Ceaser::encrypt2("rust", 13), "ehfg");
    }

    #[test]
    fn caesar_unicode() {
        assert_eq!(Ceaser::encrypt2("day 7 of 100 Days of code", 5), "ifd 7 tk 100 Ifdx tk htij");
    }
}