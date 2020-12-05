
fn check_for_sum_and_multiply(list: Vec<i32>, sum: i32) -> i32 {
    for s1 in &list {
        for s2 in &list {
            if s1 + s2 == sum {
                return s1 * s2;
            }
        }
    }

    return 0;
}

fn check_for_triple_sum_and_multiply(list: Vec<i32>, sum: i32) -> i32 {
    for s1 in &list {
        for s2 in &list {
            for s3 in &list {
                if s1 + s2 + s3 == sum {
                    return s1 * s2 * s3;
                }
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day1::check_for_sum_and_multiply;
    use crate::day1::check_for_triple_sum_and_multiply;
    use std::{fs, env};
    use std::path::Path;

    #[test]
    fn example() {
        let the_list = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(check_for_sum_and_multiply(the_list, 2020), 514579);
    }

    #[test]
    fn example2() {
        let the_list = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(check_for_triple_sum_and_multiply(the_list, 2020), 241861950);
    }

    #[test]
    fn first_star() {
        let the_list = {
            let path1 = env::current_dir().unwrap();
            let path = Path::new("resources/day1.txt");

            let contents = fs::read_to_string(path1.join(path))
                .expect("Something went wrong reading the file")
                .lines()
                .map(|e| e.parse::<i32>().unwrap())
                .collect();
            contents
        };

        let res = check_for_sum_and_multiply(the_list, 2020);

        println!("Result is {:?}", res);
        assert_eq!(res, 996996);
    }

    #[test]
    fn second_star() {
        let the_list = {
            let path1 = env::current_dir().unwrap();
            let path = Path::new("resources/day1.txt");

            let contents = fs::read_to_string(path1.join(path))
                .expect("Something went wrong reading the file")
                .lines()
                .map(|e| e.parse::<i32>().unwrap())
                .collect();
            contents
        };

        let res = check_for_triple_sum_and_multiply(the_list, 2020);

        println!("Result is {:?}", res);
        assert_eq!(res, 9210402);
    }
}