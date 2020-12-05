
#[derive(Debug)]
struct Snowfield {
    f: Vec<Vec<bool>>
}

struct Slope {
    x: usize,
    y: usize,
}

impl Snowfield {
    fn count_trees_on_slope(&self, s: Slope) -> u64 {
        let len_y = self.f.len();
        let len_x = self.f[0].len();

        let mut x : usize= 0;
        let mut y : usize = 0;

        let mut count = 0;

        while y < len_y {
            let line = &self.f[y];
            let pos = line[x];

            if pos {
                count+=1;
            }

            x = (x+s.x)%len_x;
            y = y+s.y;
        }

        return count;
    }

    fn trees_on_slopes(&self, slopes: Vec<Slope>) -> u64 {
        slopes.into_iter()
            .map(|s| self.count_trees_on_slope(s))
            .fold(1,|a, b| a*b)
    }
}

const TREE: char = '#';

fn build_snowfield(description: Vec<String>) -> Snowfield {
    let field = description.into_iter().map(|line| line.chars().map(
          |c| if c == TREE {true} else {false}
    ).collect()).collect();
    return Snowfield{
        f: field
    };
}

#[cfg(test)]
mod tests {
    use std::{env, fs};
    use std::path::Path;
    use crate::day3::{build_snowfield, Slope};

    #[test]
    fn first_example() {
        let path1 = env::current_dir().unwrap();
        let path = Path::new("resources/day3_example.txt");

        let contents = fs::read_to_string(path1.join(path))
            .expect("Something went wrong reading the file")
            .lines()
            .map(|e| e.to_string())
            .collect();

        let f = build_snowfield(contents);
        assert_eq!(f.count_trees_on_slope(Slope{x: 3, y:1}),7);
    }

    #[test]
    fn second_example() {
        let path1 = env::current_dir().unwrap();
        let path = Path::new("resources/day3_example.txt");

        let contents = fs::read_to_string(path1.join(path))
            .expect("Something went wrong reading the file")
            .lines()
            .map(|e| e.to_string())
            .collect();

        let f = build_snowfield(contents);

        let slopes: Vec<Slope> = vec![Slope{x:1,y:1}, Slope{x:3,y:1}, Slope{x:5,y:1}, Slope{x:7,y:1}, Slope{x:1,y:2}];

        assert_eq!(f.trees_on_slopes(slopes), 336);
    }

    #[test]
    fn first_star() {
        let path1 = env::current_dir().unwrap();
        let path = Path::new("resources/day3.txt");

        let contents = fs::read_to_string(path1.join(path))
            .expect("Something went wrong reading the file")
            .lines()
            .map(|e| e.to_string())
            .collect();

        let f = build_snowfield(contents);

        let res = f.count_trees_on_slope(Slope{x: 3, y:1});

        println!("Trees: {}", res);
        assert_eq!(res,244);
    }


    #[test]
    fn second_star() {
        let path1 = env::current_dir().unwrap();
        let path = Path::new("resources/day3.txt");

        let contents = fs::read_to_string(path1.join(path))
            .expect("Something went wrong reading the file")
            .lines()
            .map(|e| e.to_string())
            .collect();

        let slopes: Vec<Slope> = vec![Slope{x:1,y:1}, Slope{x:3,y:1}, Slope{x:5,y:1}, Slope{x:7,y:1}, Slope{x:1,y:2}];

        let f = build_snowfield(contents);
        let res = f.trees_on_slopes(slopes);
        println!("Trees multiplied: {}", res);
    }
}
