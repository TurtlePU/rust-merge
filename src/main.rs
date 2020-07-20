mod merge;

use merge::Mergeable;
use std::ops::{Mul, Sub};

#[allow(dead_code)]
fn process(iter: impl Iterator<Item = Line>) -> impl Iterator<Item = Line> {
    iter.filter(keep).merge(try_merge)
}

fn keep(line: &Line) -> bool {
    line.vec() != Point::default()
}

fn try_merge(a: Line, b: Line) -> Result<Line, (Line, Line)> {
    if a.vec() * b.vec() == 0 {
        Ok(Line(a.0, b.1))
    } else {
        Err((a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve(vec: Vec<[i32; 4]>) -> Vec<[i32; 4]> {
        process(vec.into_iter().map(format)).map(deformat).collect()
    }

    fn format(arr: [i32; 4]) -> Line {
        Line(Point { x: arr[0], y: arr[1] }, Point { x: arr[2], y: arr[3] })
    }

    fn deformat(line: Line) -> [i32; 4] {
        [ line.0.x, line.0.y, line.1.x, line.1.y ]
    }

    #[test]
    fn empty_in_empty_out() {
        assert_eq!(solve(vec![]), Vec::<[i32; 4]>::new());
    }

    #[test]
    fn dot_in_empty_out() {
        assert_eq!(solve(vec![[5, 7, 5, 7]]), Vec::<[i32; 4]>::new());
    }

    #[test]
    fn line_in_line_out() {
        assert_eq!(solve(vec![[0, 0, 1, 1]]), vec![[0, 0, 1, 1]]);
    }

    #[test]
    fn merge_parallel() {
        assert_eq!(
            solve(vec![[0, 0, 0, 1], [0, 1, 0, 2]]),
            vec![[0, 0, 0, 2]],
        );
    }

    #[test]
    fn two_lines_and_dot() {
        assert_eq!(
            solve(vec![
                [0, 0, 1, 1],
                [1, 1, 1, 2],
                [1, 2, 1, 2],
                [1, 2, 1, 3],
            ]),
            vec![[0, 0, 1, 1], [1, 1, 1, 3]],
        );
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Line(Point, Point);

impl Line {
    fn vec(&self) -> Point {
        self.1 - self.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Point { x: i32, y: i32 }

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul for Point {
    type Output = i64;

    fn mul(self, other: Self) -> Self::Output {
        self.x as i64 * other.y as i64 - self.y as i64 * other.x as i64
    }
}

fn main() {
    println!("run `cargo test`");
}
