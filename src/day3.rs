use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
enum Error {
    BadInstruction,
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Error {
        Error::BadInstruction
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(value: &str) -> Result<Instruction, Self::Error> {
        if value.len() < 2 {
            return Err(Error::BadInstruction);
        }

        let (dir_part, dist_part) = value.split_at(1);
        let dist = dist_part.parse::<i64>()?;

        match dir_part {
            "U" => Ok(Instruction::Up(dist)),
            "D" => Ok(Instruction::Down(dist)),
            "L" => Ok(Instruction::Left(dist)),
            "R" => Ok(Instruction::Right(dist)),
            _ => Err(Error::BadInstruction),
        }
    }
}

type Coord = (i64, i64);

pub fn part1(input: &str) {
    let paths = match get_paths(input) {
        None => return,
        Some(paths) => paths,
    };

    match closest_intersection_by_manhattan(&paths[0], &paths[1]) {
        Some(dist) => println!(
            "The closest intersection by manhattan distance is {} units away",
            dist
        ),
        None => println!("There are no intersections"),
    }
}

pub fn part2(input: &str) {
    let paths = match get_paths(input) {
        None => return,
        Some(paths) => paths,
    };

    match closest_intersection_by_steps(&paths[0], &paths[1]) {
        Some(dist) => println!(
            "The closest intersection by stepping along the paths is {} units away",
            dist
        ),
        None => println!("There are no intersections"),
    }
}

fn get_paths(input: &str) -> Option<Vec<Vec<Coord>>> {
    let paths_res: std::result::Result<Vec<_>, Error> = input.lines().map(get_path).collect();
    let paths = match paths_res {
        Ok(paths) => paths,
        Err(why) => {
            println!("Could not parse input instructions: {:?}", why);
            return None;
        }
    };
    if paths.len() != 2 {
        println!("Expected two input paths");
        return None;
    }

    Some(paths)
}

fn get_path(line: &str) -> Result<Vec<Coord>, Error> {
    let instructions = line
        .split(',')
        .map(|instruction| Instruction::try_from(instruction))
        .collect::<Result<Vec<_>, Error>>()?;
    let mut path = vec![(0, 0)];
    for i in instructions.iter() {
        path = apply_instruction(i, path);
    }
    Ok(path)
}

// Returns the closest intersection of the two paths by calculating the manhattan distance of the
// intersection from the origin.
fn closest_intersection_by_manhattan(path1: &Vec<Coord>, path2: &Vec<Coord>) -> Option<i64> {
    let coords1: HashSet<Coord> = path1.iter().cloned().collect();
    let coords2: HashSet<Coord> = path2.iter().cloned().collect();
    coords1.intersection(&coords2).fold(None, |closest, coord| {
        if *coord == (0, 0) {
            return closest;
        }
        let dist = origin_distance(coord);
        match closest {
            None => Some(dist),
            Some(x) => {
                if dist < x {
                    Some(dist)
                } else {
                    closest
                }
            }
        }
    })
}

// Returns the closest intersection of the two paths by calculating the distance along the paths.
fn closest_intersection_by_steps(path1: &Vec<Coord>, path2: &Vec<Coord>) -> Option<i64> {
    let coords1: HashSet<Coord> = path1.iter().cloned().collect();
    let coords2: HashSet<Coord> = path2.iter().cloned().collect();
    coords1.intersection(&coords2).fold(None, |closest, coord| {
        if *coord == (0, 0) {
            return closest;
        }
        let dist1 = path1.iter().position(|&c| c == *coord).unwrap();
        let dist2 = path2.iter().position(|&c| c == *coord).unwrap();
        let tot_dist = (dist1 + dist2) as i64;

        match closest {
            None => Some(tot_dist),
            Some(x) => {
                if tot_dist < x {
                    Some(tot_dist)
                } else {
                    closest
                }
            }
        }
    })
}

fn apply_instruction(instruction: &Instruction, mut path: Vec<Coord>) -> Vec<Coord> {
    let curr = path.last().unwrap().clone();

    match instruction {
        Instruction::Up(d) => {
            for i in (curr.1 + 1)..((curr.1 + 1) + d) {
                path.push((curr.0, i));
            }
        }
        Instruction::Down(d) => {
            for i in ((curr.1 - d)..(curr.1)).rev() {
                path.push((curr.0, i));
            }
        }
        Instruction::Left(d) => {
            for i in ((curr.0 - d)..(curr.0)).rev() {
                path.push((i, curr.1));
            }
        }
        Instruction::Right(d) => {
            for i in (curr.0 + 1)..((curr.0 + 1) + d) {
                path.push((i, curr.1));
            }
        }
    }
    path
}

fn origin_distance(coord: &Coord) -> i64 {
    (coord.0).abs() + (coord.1).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_instruction_from_str() {
        assert_eq!(Err(Error::BadInstruction), Instruction::try_from(""));
        assert_eq!(Err(Error::BadInstruction), Instruction::try_from("U"));
        assert_eq!(Err(Error::BadInstruction), Instruction::try_from("UD"));
        assert_eq!(Err(Error::BadInstruction), Instruction::try_from("F1"));
        assert_eq!(Ok(Instruction::Up(1)), Instruction::try_from("U1"));
        assert_eq!(Ok(Instruction::Down(10)), Instruction::try_from("D10"));
        assert_eq!(Ok(Instruction::Left(100)), Instruction::try_from("L100"));
        assert_eq!(Ok(Instruction::Right(1000)), Instruction::try_from("R1000"));
    }

    #[test]
    fn test_apply_instruction() {
        assert_eq!(
            vec![(0, 0)],
            apply_instruction(&Instruction::Up(0), vec![(0, 0)])
        );
        assert_eq!(
            vec![(0, 0)],
            apply_instruction(&Instruction::Down(0), vec![(0, 0)])
        );
        assert_eq!(
            vec![(0, 0)],
            apply_instruction(&Instruction::Left(0), vec![(0, 0)])
        );
        assert_eq!(
            vec![(0, 0)],
            apply_instruction(&Instruction::Right(0), vec![(0, 0)])
        );
        assert_eq!(
            vec![(0, 0), (0, 1)],
            apply_instruction(&Instruction::Up(1), vec![(0, 0)])
        );
        assert_eq!(
            vec![(0, 0), (0, -1)],
            apply_instruction(&Instruction::Down(1), vec![(0, 0)])
        );
        assert_eq!(
            vec![(0, 0), (-1, 0)],
            apply_instruction(&Instruction::Left(1), vec![(0, 0)])
        );
        assert_eq!(
            vec![(0, 0), (1, 0)],
            apply_instruction(&Instruction::Right(1), vec![(0, 0)])
        );
        assert_eq!(
            vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)],
            apply_instruction(&Instruction::Up(5), vec![(0, 0)])
        );
        assert_eq!(
            vec![(0, 0), (0, -1), (0, -2), (0, -3), (0, -4), (0, -5)],
            apply_instruction(&Instruction::Down(5), vec![(0, 0)])
        );
        assert_eq!(
            vec![(0, 0), (-1, 0), (-2, 0), (-3, 0), (-4, 0), (-5, 0)],
            apply_instruction(&Instruction::Left(5), vec![(0, 0)])
        );
        assert_eq!(
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
            apply_instruction(&Instruction::Right(5), vec![(0, 0)])
        );
    }

    #[test]
    fn test_get_path() {
        assert_eq!(Err(Error::BadInstruction), Instruction::try_from("U1,"));
        assert_eq!(Ok(vec![(0, 0), (0, 1)]), get_path("U1"));
        assert_eq!(Ok(vec![(0, 0), (0, 1), (0, 0)]), get_path("U1,D1"));
        assert_eq!(
            Ok(vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (8, 1),
                (8, 2),
                (8, 3),
                (8, 4),
                (8, 5),
                (7, 5),
                (6, 5),
                (5, 5),
                (4, 5),
                (3, 5),
                (3, 4),
                (3, 3),
                (3, 2)
            ]),
            get_path("R8,U5,L5,D3")
        );
    }

    #[test]
    fn test_closest_intersection_by_manhattan() {
        let path1 = get_path("R1").unwrap();
        let path2 = get_path("U1").unwrap();
        assert_eq!(None, closest_intersection_by_manhattan(&path1, &path2));

        let path1 = get_path("R1").unwrap();
        let path2 = get_path("U1,R1,D1").unwrap();
        assert_eq!(Some(1), closest_intersection_by_manhattan(&path1, &path2));

        let path1 = get_path("R8,U5,L5,D3").unwrap();
        let path2 = get_path("U7,R6,D4,L4").unwrap();
        assert_eq!(Some(6), closest_intersection_by_manhattan(&path1, &path2));

        let path1 = get_path("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap();
        let path2 = get_path("U62,R66,U55,R34,D71,R55,D58,R83").unwrap();
        assert_eq!(Some(159), closest_intersection_by_manhattan(&path1, &path2));

        let path1 = get_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap();
        let path2 = get_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap();
        assert_eq!(Some(135), closest_intersection_by_manhattan(&path1, &path2));
    }

    #[test]
    fn test_closest_intersection_by_steps() {
        let path1 = get_path("R1").unwrap();
        let path2 = get_path("U1").unwrap();
        assert_eq!(None, closest_intersection_by_steps(&path1, &path2));

        let path1 = get_path("R8,U5,L5,D3").unwrap();
        let path2 = get_path("U7,R6,D4,L4").unwrap();
        assert_eq!(Some(30), closest_intersection_by_steps(&path1, &path2));

        let path1 = get_path("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap();
        let path2 = get_path("U62,R66,U55,R34,D71,R55,D58,R83").unwrap();
        assert_eq!(Some(610), closest_intersection_by_steps(&path1, &path2));

        let path1 = get_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap();
        let path2 = get_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap();
        assert_eq!(Some(410), closest_intersection_by_steps(&path1, &path2));
    }
}
