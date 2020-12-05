extern crate parser_combinators;

struct PwEntry {
    min: usize,
    max: usize,
    letter: char,
    pw:str
}

impl PwEntry {
    fn is_valid(&self) -> bool {
        let count = self.pw.chars().filter(|c| c == letter).count();

        return count >= self.min && count <= self.max;
    }
}

fn parse_input(input: &str) -> Vec<PwEntry> {

    return vec![];
}

fn count_valid_passwords(pwds: Vec<PwEntry>) -> i32 {
    pwds.into_iter().filter(|p| p.is_valid()).count().i32
}

#[cfg(test)]
mod tests {
    use crate::day2::{count_valid_passwords, parse_input};

    #[test]
    fn example() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc\n";
        assert_eq!(count_valid_passwords(parse_input(input)),2);
    }
}