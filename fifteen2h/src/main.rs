use std::fs;
use std::ops::RangeInclusive;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Sensor {
    pos: Coord,
    beacon: Coord,
    distance: isize
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: isize,
    y: isize,
}


fn main() {
    let input = fs::read_to_string("./15.input").expect("could not read file");

    let res: Vec<Sensor> = input.lines().flat_map( sensor_parser::sensor).collect();

    let max_bound = 4000000;

    let mut ranges_in: &mut Vec<(isize, isize)> = &mut vec![];
    let mut ranges_out: &mut Vec<(isize, isize)> = &mut vec![];


    for y in 0..(max_bound+1){
        ranges_in.clear();
        ranges_in.push((0,max_bound));
        ranges_out.clear();

        for sensor in res.iter() {
            if !RangeInclusive::new(sensor.pos.y - sensor.distance, sensor.pos.y + sensor.distance).contains(&y) {
                continue //dont bother if the sensor doesnt cover this y
            }

            let sensor_x_distance = sensor.distance - (sensor.pos.y - y).abs();

            let x_min = sensor.pos.x - sensor_x_distance;
            let x_max = sensor.pos.x + sensor_x_distance;

            while let Some(mut span) = ranges_in.pop() {

                if x_min <= span.0 && x_max >= span.1 {
                    continue;
                }
            
                if x_max < span.0 || x_min > span.1 {
                    ranges_out.push(span);
                    continue;
                }

                if x_min <= span.0 && x_max <= span.1 {
                    span.0 = x_max + 1;
                    ranges_out.push(span);
                    continue;
                }

                if x_min >= span.0 && x_max >= span.1 {
                    span.1 = x_min -1;
                    ranges_out.push(span);
                    continue;
                }

                //we have a split!
                ranges_out.push((span.0, x_min - 1));
                ranges_out.push((x_max + 1, span.1));
            }

            (ranges_in, ranges_out) = (ranges_out, ranges_in);
        }

        //since there is only one place possible for beacon in area (program garauntee)
        // if we have any ranges of possible beacons left, we have our answer!
        if ranges_in.len() == 1 {
            //we have to have found a solution.
            println!("{} {:?} {}", y, ranges_in, y  + ranges_in[0].0 * 4000000);
            return;
        }
    }
}

peg::parser!{
    grammar sensor_parser() for str {

    rule number() -> isize
        = n:$(['-']? ['0'..='9']+) {? n.parse().or(Err("isize")) }

    pub rule sensor() -> Sensor
        = "Sensor at x=" sx:number() ", y=" sy:number() ": closest beacon is at x=" bx:number() ", y=" by:number() {
            Sensor {
                pos: Coord{x: sx, y:sy},
                beacon: Coord{x: bx, y: by},
                distance: (sx - bx).abs() + (sy - by).abs()
            }
        }
    }
}