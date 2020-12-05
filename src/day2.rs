struct PwEntry {
    min: usize,
    max: usize,
    letter: char,
    pw: String,
}

impl PwEntry {
    fn is_valid(&self) -> bool {
        let count = self.pw.chars().filter(|c| *c == self.letter).count();

        return count >= self.min && count <= self.max;
    }
}

/*
fn parse_input(input: &str) -> Vec<PwEntry> {

    return vec![];
}*/

fn count_valid_passwords(pwds: Vec<PwEntry>) -> i32 {
    pwds.into_iter().filter(|p| p.is_valid()).count() as i32
}

#[cfg(test)]
mod tests {
    use crate::day2::{count_valid_passwords, PwEntry};

    #[test]
    fn example_without_parser() {
        /*1-3 a: abcde
          1-3 b: cdefg
          2-9 c: ccccccccc
         */

        let input = vec![
            PwEntry{min:1,max:3,letter:'a', pw: String::from("adcde")},
            PwEntry{min:1,max:3,letter:'b', pw: String::from("cdefg")},
            PwEntry{min:2,max:9,letter:'c',pw: String::from("ccccccccc")},
        ];
        assert_eq!(count_valid_passwords(input),2);
    }
}