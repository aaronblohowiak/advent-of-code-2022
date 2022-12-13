use std::cmp::Ordering;
use std::fs;

use itertools::Itertools;


use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::separated_list0,
    sequence::delimited,
    IResult
};

/*

 You'll need to re-order the list of received packets (your puzzle input) to decode the message.

Your list consists of pairs of packets; pairs are separated by a blank line. You need to identify how many pairs of packets are in the right order.
*/

#[derive(Debug, PartialEq, Eq, Clone)]
enum PacketEntry {
    List(Vec<PacketEntry>),
    Value(u32)
}

impl Ord for PacketEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self, other)
    }
}

impl PartialOrd for PacketEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packet_value(i: &str) -> IResult<&str, PacketEntry> {
    map(nom::character::complete::u32, PacketEntry::Value)(i)
}


fn parse_packet_list(i: &str) -> IResult<&str, PacketEntry> {
    map(
        delimited(tag("["),
            separated_list0(tag(","),
                alt((
                    parse_packet_value,
                    parse_packet_list
                ))
            ),
            tag("]")
        ),
        PacketEntry::List)(i) //map this to a PacketEntry::List
}

fn compare_lists(left: &Vec<PacketEntry>, right: &Vec<PacketEntry>) -> Ordering{
    let mut right_iter = right.iter();
    for l in left {
        if let Some(r) = right_iter.next(){
            let ordering = compare(l, r);
            if ordering != Ordering::Equal {
                return ordering;
            }
        }else{
            return Ordering::Greater;
        }
    }

    if let Some(_r) = right_iter.next() {
        return Ordering::Less
    }

    Ordering::Equal
}

fn compare(left: &PacketEntry, right: &PacketEntry) -> Ordering{
    match (left, right) {
        (PacketEntry::Value(left), PacketEntry::Value(right)) => left.cmp(right),
        (PacketEntry::List(left), PacketEntry::List(right)) => compare_lists(left, right),
        (PacketEntry::List(left), PacketEntry::Value(right)) => compare_lists(left, &vec![PacketEntry::Value(*right)]),
        (PacketEntry::Value(left), PacketEntry::List(right)) => compare_lists(&vec![PacketEntry::Value(*left)], right)
    }
}


fn main() {

    let input = fs::read_to_string("./13.input").expect("could not read file");

    let pairs : Vec<(PacketEntry, PacketEntry)> = input.split("\n\n").map(|g| {
        let (left, right) = g.split('\n').map(|s| {
            parse_packet_list(s).expect("parse packet fully").1
        }
        ).next_tuple().unwrap();
        (left, right)
    }).collect();

    let cnt : usize = pairs.iter()
        .map(|(l, r)| compare(l, r))
        .enumerate().map(|(i, p)| (i+1, p))
        .filter(|(_i, p)| p == &Ordering::Less)
        .map(|(i,_)| i ).sum();
    
    println!("{:?}", cnt);

    let mut packets = pairs.iter().fold(Vec::new(), |mut acc, (l,r)| {acc.push(l);acc.push(r);acc});
    let separators = ( PacketEntry::List(vec![PacketEntry::List(vec![PacketEntry::Value(2)])]), PacketEntry::List(vec![PacketEntry::List(vec![PacketEntry::Value(6)])]));
    packets.push(&separators.0);
    packets.push(&separators.1);


    packets.sort();

    let res : usize = packets.iter().enumerate().filter(|(_, p)| **p == &separators.0 || **p == &separators.1 ).map(|(i, _)| i + 1).product();

    println!("{:?}", res);
}


#[cfg(test)]
mod test_parsing {
    use crate::*;
    use crate::PacketEntry::*;


    fn pe(s: &str) -> PacketEntry {
        parse_packet_list(s).unwrap().1
    }

    #[test]
    fn test_parse(){
        let res = pe("[[],1,2]");

        assert_eq!(PacketEntry::List(vec![PacketEntry::List(vec![]), PacketEntry::Value(1), PacketEntry::Value(2)]), res);
    }

    #[test]
    fn test_cmp(){
        assert_eq!(compare(&PacketEntry::Value(1), &PacketEntry::Value(2)), Ordering::Less);

        //test list length
        assert_eq!(compare(&List(vec![Value(1)]), &List(vec![])), Ordering::Greater);
        assert_eq!(compare(&List(vec![]), &List(vec![Value(1)])), Ordering::Less);

        //test list of value
        assert_eq!(compare(&List(vec![Value(1)]), &List(vec![Value(1)])), Ordering::Equal);
        assert_eq!(compare(&List(vec![Value(1)]), &List(vec![Value(2)])), Ordering::Less);

        assert_eq!(compare(&pe("[1,[2,[3,[4,[5,6,7]]]],8,9]"), &pe("[1,[2,[3,[4,[5,6,0]]]],8,9]")), Ordering::Greater);

        assert_eq!(compare(&pe("[[[]]]"), &pe("[[]]")), Ordering::Greater);
    }
}
