use std::fs;

fn main() {
    let input = fs::read_to_string("./5.input").expect("Error while reading");

    let mut lines = input.lines();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    //this will read in the starting thing and consume the line with the column numbers, but we dont need that because math.
    (&mut lines)
        .take_while(|line| {
            return line.trim().starts_with("[");
        })
        .for_each(|line| {
            if stacks.len() == 0 {
                // need to set up our vector.
                // we have a columnar format where each column width is three and a space between columns
                // so the line width = 3x columns + x -1 spaces. width = 4x -1. (width + 1) / 4 = x.
                (0..(line.len() + 1) / 4).for_each(|_| {
                    stacks.push(Vec::new());
                });
            };

            let mut num = 0;
            let mut row = line.chars().into_iter();
            row.next(); //skip opening paren.

            row.step_by(4).for_each(|c| {
                //dont push empty columns
                if c.is_alphabetic() {
                    stacks[num].push(c);
                }
                num += 1;
            })
        });

    stacks = dbg!(stacks);

    stacks.iter_mut().for_each(|a| a.reverse());

    lines.next();

    lines.map(|movement| {
        let mut parts = movement.split(" ");
        let (Some("move"), Some(count), Some("from"), Some(source), Some("to"), Some(dest), None) = (parts.next(),parts.next(),parts.next(),parts.next(),parts.next(),parts.next(),parts.next()) else{
            panic!("could not parse movement line {:?}", movement);
        };

        return (count.parse::<usize>().unwrap(), source.parse::<usize>().unwrap()-1, dest.parse::<usize>().unwrap()-1);
    }).for_each(|(count, source, dest)| {
        let split_pos = stacks[source].len() - count;
        let mut tmp = stacks[source].split_off(split_pos);
        stacks[dest].append(&mut tmp);
    });

    println!(
        "{:?}",
        stacks
            .into_iter()
            .map(|mut s| s.pop().unwrap())
            .collect::<String>()
    );
}
