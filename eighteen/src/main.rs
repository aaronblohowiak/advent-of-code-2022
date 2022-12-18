use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::ops::Add;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3d {
    fn from(i: (i32, i32, i32)) -> Point3d {
        Point3d {
            x: i.0,
            y: i.1,
            z: i.2,
        }
    }
}

impl<'a, 'b> Add<&'b Point3d> for &'a Point3d {
    type Output = Point3d;

    fn add(self, other: &'b Point3d) -> Point3d {
        Point3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

const UP: Point3d = Point3d { x: 0, y: 0, z: 1 };
const DOWN: Point3d = Point3d { x: 0, y: 0, z: -1 };

const RIGHT: Point3d = Point3d { x: 1, y: 0, z: 0 };
const LEFT: Point3d = Point3d { x: -1, y: 0, z: 0 };

const BACK: Point3d = Point3d { x: 0, y: 1, z: 0 };
const FRONT: Point3d = Point3d { x: 0, y: -1, z: 0 };

const DIRECTIONS: [Point3d; 6] = [UP, DOWN, LEFT, RIGHT, BACK, FRONT];

fn main() {
    let points = parse_input("18.input");

    //i could construct a tree with six edges per node and that would make this fast. OR, brute force. Hulk smash!

    println!("Part 1: {}", part1(&points));

    println!("Part 2: {}", part2(&points));
}

fn part1(points: &HashSet<Point3d>) -> i32 {
    points
        .iter()
        .map(|p| {
            DIRECTIONS
                .iter()
                .map(|d| !points.contains(&(p + d)) as i32)
                .sum::<i32>()
        })
        .sum()
}

//make a box that fully sorrounds the lava with steam, then for every lava point, count directions that touch steam.
fn part2(lava_points: &HashSet<Point3d>) -> i32 {
    let mut maxes = Point3d::default();
    let mut mins = Point3d::default();

    lava_points.iter().for_each(|p| {
        maxes.x = maxes.x.max(p.x + 1); //bounding box should SORROUND lava with steam, so extend 1 more
        maxes.y = maxes.y.max(p.y + 1);
        maxes.z = maxes.z.max(p.z + 1);

        mins.x = mins.x.min(p.x - 1); //bounding box should SORROUND lava with steam, so pull back 1
        mins.y = mins.y.min(p.y - 1);
        mins.z = mins.z.min(p.z - 1);
    });

    let mut steam_box: HashSet<Point3d> = HashSet::default();

    steam_fill(&mut steam_box, lava_points, &maxes, &maxes, &mins);

    lava_points
        .iter()
        .cartesian_product(DIRECTIONS.iter())
        .filter(|(p, d)| steam_box.contains(&(*p + *d)))
        .count() as i32
}

fn steam_fill(
    steam_box: &mut HashSet<Point3d>,
    lava_points: &HashSet<Point3d>,
    from: &Point3d,
    maxes: &Point3d,
    mins: &Point3d,
) {
    steam_box.insert(*from);

    for dir in DIRECTIONS {
        let next = from + &dir;

        if contained3d(&next, maxes, mins)
            && !lava_points.contains(&next)
            && !steam_box.contains(&next)
        {
            steam_fill(steam_box, lava_points, &next, maxes, mins);
        }
    }
}

fn contained3d(next: &Point3d, maxes: &Point3d, mins: &Point3d) -> bool {
    next.x >= mins.x
        && next.x <= maxes.x
        && next.y >= mins.y
        && next.y <= maxes.y
        && next.z >= mins.z
        && next.z <= maxes.z
}

fn parse_input(fname: &str) -> HashSet<Point3d> {
    let input = fs::read_to_string(fname).expect("could not read file");
    input
        .lines()
        .flat_map(|l| {
            l.split(',')
                .map(|s| s.parse().unwrap())
                .tuples()
                .map(Point3d::from)
        })
        .collect()
}
