extern crate combine;
use combine::{many1, Parser, skip_many};
use combine::parser::char::{letter, space, digit, char, newline};
use self::combine::{EasyParser, sep_by1};

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

    fn is_valid2(&self) -> bool {
        let pos1 = self.min;
        let pos2 = self.max;

        let char1 = self.pw.chars().nth(pos1-1).unwrap();
        let char2 = self.pw.chars().nth(pos2-1).unwrap();

        return (char1 == self.letter) ^ (char2 == self.letter);

    }
}

fn parse_input(input: &str) -> Vec<PwEntry> {
    let word = many1(letter());
    let number = || many1(digit()).map(|s: String| s.parse::<usize>().unwrap());
    let line =(number(),skip_many(char('-')),number(),skip_many(space()),
               letter(),skip_many((char(':'),space())),word)
        .map(|(min, (), max, (), c, (), pw)| PwEntry{
            min: min,
            max: max,
            letter: c,
            pw: pw
        });

    let mut pwd_file = sep_by1(line, newline()).map(|e : Vec<PwEntry>| e);
    match pwd_file.easy_parse(input) {
        Ok((res, _)) => return res,
        Err(err) => println!("{} in ", err)
    };
    return vec![];
}

fn count_valid_passwords(pwds: Vec<PwEntry>, func: fn(&PwEntry) -> bool) -> i32 {
    pwds.into_iter().filter(|p| func(p)).count() as i32
}

#[cfg(test)]
mod tests {
    use crate::day2::{count_valid_passwords, PwEntry, parse_input};
    use std::{env, fs};
    use std::path::Path;

    #[test]
    fn example_without_parser() {
        //let input : String::From("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc\n");

        let input = vec![
            PwEntry{min:1,max:3,letter:'a', pw: String::from("adcde")},
            PwEntry{min:1,max:3,letter:'b', pw: String::from("cdefg")},
            PwEntry{min:2,max:9,letter:'c', pw: String::from("ccccccccc")},
        ];
        assert_eq!(count_valid_passwords(input, PwEntry::is_valid),2);
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
        let input = String::from("1-3 a: abcde\n1-3 b: cdefg\n2-19 c: ccccccccc");
        assert_eq!(count_valid_passwords(parse_input(&input), PwEntry::is_valid),2);
    }

    #[test]
    fn first_star() {
        let path1 = env::current_dir().unwrap();
        let path = Path::new("resources/day2.txt");

        let contents = fs::read_to_string(path1.join(path)).unwrap();

        let res = count_valid_passwords(parse_input(&contents), PwEntry::is_valid);

        println!("Counted {} valid passwords", res);

        assert_eq!(res, 546);
    }

    #[test]
    fn second_star() {
        let path1 = env::current_dir().unwrap();
        let path = Path::new("resources/day2.txt");

        let contents = fs::read_to_string(path1.join(path)).unwrap();

        let res = count_valid_passwords(parse_input(&contents), PwEntry::is_valid2);

        println!("Counted {} valid passwords", res);

        assert_eq!(res, 275);
    }
}