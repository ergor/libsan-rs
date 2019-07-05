
/// ported from https://github.com/chesszebra/standard-algebraic-notation

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod san;
use san::*;
use regex::Regex;

pub fn test() {
    println!("hello");
}

pub fn parse(value: &str) -> Move {
    let mut mov = Move::new();

    // Check for castling:
    let re = Regex::new(r"^(O-O|O-O-O)(\+|\#?)(\?\?|\?|\?\!|\!|\!\!)?$").unwrap();
    if re.is_match(value) {
        let cap = re.captures(value).unwrap();
        mov.move_type = CastleType::from_str(&cap[1]).unwrap();
        $this->check = $matches[2] === '+';
        $this->checkmate = $matches[2] === '#';
        $this->annotation = isset($matches[3]) ? $matches[3] : null;
        mov
    }

    mov
}