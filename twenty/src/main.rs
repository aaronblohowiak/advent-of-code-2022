fn cycle_index(idx: isize, len: isize) -> usize {
    let idx = idx % len;
    if idx < 0 {
        //we are shifting left and have looped.
        return (len + idx) as usize;
    }

    idx as usize
}

fn shifter(data: &mut Vec<(usize, &isize)>, starting_idx: usize, shift: isize) {
    let mut current_pos = starting_idx as isize;
    let dir = shift.signum();
    let len = data.len();

    //perform 'shift' hops in 'dir'
    // our after len - 1 hops,
    //     we are in same order so only actually shift the remainder.
    for _ in 0..(shift % (len -1) as isize).abs() {
        let next_pos = cycle_index(current_pos + dir, len as isize);
        data.swap(current_pos as usize, next_pos as usize);

        current_pos = next_pos as isize;
    }
}

fn print_from_0(data: &Vec<(usize, &isize)>) {
    let idx = data
        .iter()
        .position(|s| *s.1 == 0)
        .expect("should contain 0");
    let len = data.len();

    println!(
        "{:?}",
        data.iter()
            .cycle()
            .skip(idx)
            .take(len)
            .collect::<Vec<&(usize, &isize)>>()
    );
}

fn mix(encryption_key: isize, times: usize) {
    let input: Vec<isize> = std::fs::read_to_string("./20.input")
        .expect("read input")
        .lines()
        .map(|s| s.parse::<isize>().unwrap() * encryption_key )
        .collect();

    let mut output = input.iter().enumerate().collect::<Vec<(usize, &isize)>>();

    for _ in 0..times {
        for n in 0..input.len() {
            // print_from_0(&output);
    
            let index = output
                .iter()
                .position(|x| x.0 == n)
                .expect("all starting items should be present in final");
            let x = *output[index].1;
    
            shifter(&mut output, index, x);
        }    
    }

    // print_from_0(&output);

    /* Then, the grove coordinates can be found by looking at the 1000th, 2000th, and 3000th numbers after the value 0,
    wrapping around the list as necessary. In the above example, the 1000th number after 0 is 4, the 2000th is -3,
     and the 3000th is 2; adding these together produces 3.
     */

    let index = output
        .iter()
        .position(|x| *x.1 == 0)
        .expect("all starting items should be present in final") as isize;
    let len = output.len() as isize;

    let total = [1000, 2000, 3000]
        .iter()
        .map(|offset| {
            let pos = cycle_index(index + offset, len);
            println!(
                "0 idx: {} + offset: {} lookup:({}) =>  {:?}",
                index, offset, pos, output[pos]
            );

            output[pos].1
        })
        .sum::<isize>();

    println!("{}: {}", times, total);
}


fn main() {
    mix(1, 1);
    mix(811589153, 10);
}