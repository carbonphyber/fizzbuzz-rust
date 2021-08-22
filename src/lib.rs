pub mod fizzbuzzapp {
    pub fn fizzbuzz(input: u32) -> Result<String, &'static str> {
        if input < 1 || input > 9999 {
            return Err("Invalid argument range")
        }
        if input % 15 == 0 {
            return Ok("fizzbuzz".to_string());
        } else if input % 3 == 0 {
            return Ok("fizz".to_string());
        } else if input % 5 == 0 {
            return Ok("buzz".to_string());
        }
        Ok(format!("{}", input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz0() {
        assert_eq!(fizzbuzzapp::fizzbuzz(0), Err("Invalid argument range"));
    }

    #[test]
    fn fizzbuzz10000() {
        assert_eq!(fizzbuzzapp::fizzbuzz(10000), Err("Invalid argument range"));
    }

    #[test]
    fn fizzbuzz1() {
        assert_eq!(fizzbuzzapp::fizzbuzz(1), Ok("1".to_string()));
    }

    #[test]
    fn fizzbuzz2() {
        assert_eq!(fizzbuzzapp::fizzbuzz(2), Ok("2".to_string()));
    }

    #[test]
    fn fizzbuzz3() {
        assert_eq!(fizzbuzzapp::fizzbuzz(3), Ok("fizz".to_string()));
    }

    #[test]
    fn fizzbuzz4() {
        assert_eq!(fizzbuzzapp::fizzbuzz(4), Ok("4".to_string()));
    }

    #[test]
    fn fizzbuzz5() {
        assert_eq!(fizzbuzzapp::fizzbuzz(5), Ok("buzz".to_string()));
    }

    #[test]
    fn fizzbuzz6() {
        assert_eq!(fizzbuzzapp::fizzbuzz(6), Ok("fizz".to_string()));
    }

    #[test]
    fn fizzbuzz10() {
        assert_eq!(fizzbuzzapp::fizzbuzz(10), Ok("buzz".to_string()));
    }

    #[test]
    fn fizzbuzz15() {
        assert_eq!(fizzbuzzapp::fizzbuzz(15), Ok("fizzbuzz".to_string()));
    }
}

#[test]
fn fizzbuzz16() {
    assert_eq!(fizzbuzzapp::fizzbuzz(16), Ok("16".to_string()));
}
