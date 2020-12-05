extern crate combine;
use combine::{many1, Parser, skip_many};
use combine::parser::char::{letter, space, digit, char, newline};

#[derive(Debug, PartialEq)]
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

fn parse_input(input: &str) -> Vec<PwEntry> {
    let word = many1(letter());
    let line =(digit(),skip_many(char('-')),digit(),skip_many(space()),
               letter(),skip_many((char(':'),space())),word,newline())
        .map(|(min, (), max, (), c, (), pw, _)| PwEntry{
            min: min.to_string().parse::<usize>().unwrap(),
            max: max.to_string().parse::<usize>().unwrap(),
            letter: c,
            pw: pw
        });

    let mut pwd_file = many1(line).map(|e : Vec<PwEntry>| e);
    return pwd_file.parse(input).map(|(r, _)| r).unwrap();
}

fn count_valid_passwords(pwds: Vec<PwEntry>) -> i32 {
    pwds.into_iter().filter(|p| p.is_valid()).count() as i32
}

#[cfg(test)]
mod tests {
    use crate::day2::{count_valid_passwords, PwEntry, parse_input};

    #[test]
    fn example_without_parser() {
        //let input : String::From("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc\n");

        let input = vec![
            PwEntry{min:1,max:3,letter:'a', pw: String::from("adcde")},
            PwEntry{min:1,max:3,letter:'b', pw: String::from("cdefg")},
            PwEntry{min:2,max:9,letter:'c', pw: String::from("ccccccccc")},
        ];
        assert_eq!(count_valid_passwords(input),2);
    }

    #[test]
    fn test_parser() {
        let input = "1-3 c: abcde";

        let res = parse_input(input);

        assert_eq!(res[0], PwEntry{
            min: 1,
            max: 3,
            letter: 'c',
            pw: "abcde".to_string()
        });
    }

    #[test]
    fn example_with_parser() {
        let input = String::from("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc\n");
        assert_eq!(count_valid_passwords(parse_input(&input)),2);
    }
}