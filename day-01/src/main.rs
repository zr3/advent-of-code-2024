use std::{collections::HashMap, str::FromStr};

struct Lists {
    left: Vec<i32>,
    right: Vec<i32>,
}

#[derive(Debug)]
struct ListsParseErr;

impl FromStr for Lists {
    type Err = ListsParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        Ok(lines.fold(
            Lists {
                left: vec![],
                right: vec![],
            },
            |mut acc, l| {
                let mut col = l.split("   ");
                acc.left.push(col.next().unwrap().parse::<i32>().unwrap());
                acc.right.push(col.next().unwrap().parse::<i32>().unwrap());
                acc
            },
        ))
    }
}

fn main() {
    // let input = include_str!("./example.txt");
    let input = include_str!("./input.txt");
    let output_1 = part_1(input);
    println!("part 1: {}", output_1);
    let output_2 = part_2(input);
    println!("part 2: {}", output_2);
}

fn part_1(input: &str) -> String {
    let mut lists = Lists::from_str(input).unwrap();
    lists.left.sort();
    lists.right.sort();
    let result = lists
        .left
        .iter()
        .zip(lists.right)
        .fold(0, |acc, p| acc + (p.0 - p.1).abs());
    result.to_string()
}

fn part_2(input: &str) -> String {
    let lists = Lists::from_str(input).unwrap();
    let mut cache: HashMap<i32, i32> = HashMap::new();
    let result = lists.left.iter().fold(0, |acc, l| {
        if !cache.contains_key(l) {
            cache.insert(*l, lists.right.iter().filter(|r| *r == l).count() as i32);
        }
        return acc + (l * cache[l]);
    });
    result.to_string()
}

#[test]
fn test_part1() {
    let input = include_str!("./example.txt");
    let output = part_1(input);
    assert_eq!(output, "")
}

#[test]
fn test_part2() {
    let input = include_str!("./example.txt");
    let output = part_2(input);
    assert_eq!(output, "")
}
