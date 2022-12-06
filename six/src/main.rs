use std::collections::HashSet;
use std::fs;

//find the start of a particular message or packet by scanning
//     for a special header value and then returning the position
//      after that special value exists.

fn find_start(s: &str, window_size: usize) -> usize {
    let mut offset = 0;
    s.chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .take_while(|stuff| {
            let set = stuff.into_iter().collect::<HashSet<_>>();

            if set.len() != window_size {
                offset += 1; //is there a better trick for seeing how many things the take_while has consumed?
                return true;
            } else {
                return false;
            }
        })
        .for_each(|_| {}); //TODO: what is the bettter way to force the allocation?
    println!("{} {}", s, offset);
    return offset + window_size;
}

fn find_packet_start(s: &str) -> usize {
    return find_start(s, 4);
}

fn find_message_start(s: &str) -> usize {
    return find_start(s, 14);
}

fn main() {
    let input = fs::read_to_string("./6.input").expect("Error while reading");

    assert_eq!(find_packet_start("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(find_packet_start("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(find_packet_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(find_packet_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    println!("Find Packets: {}", find_packet_start(&input));

    assert_eq!(find_message_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(find_message_start("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(find_message_start("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(find_message_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(find_message_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);

    println!("Find Messages: {}", find_message_start(&input));
}
