use core::slice::Iter;
use std::collections::HashMap;
use std::fs;

/*
    Our Domain has two data types: a Dir and File.
    Directories can contain directories and files.

    Our Directories know how to compute their size and a couple weird AoC things.
*/

#[derive(Debug, PartialEq)]

struct File {
    size: usize,
    name: String,
}

#[derive(Debug, PartialEq)]
struct Dir {
    name: String,
    files: Vec<File>,
    children: HashMap<String, Dir>,
}

impl Dir {
    fn new(s: &str) -> Dir {
        return Dir {
            name: s.to_string(),
            files: vec![],
            children: HashMap::new(),
        };
    }

    //size of all my files and that of all my children.
    fn size(&self) -> usize {
        let mut total = 0;
        for f in &self.files {
            total += f.size;
        }

        for (_, d) in &self.children {
            total += d.size();
        }

        return total;
    }

    //Walk the dir try to find the sum of directories that have a transitive sum less than 100k
    // return current branch disk size and current tally
    fn aoc_dir_sum(&self) -> (usize, usize) {
        let mut my_size = 0;
        let mut dir_sum = 0;

        for f in &self.files {
            my_size += f.size;
        }

        for (_, d) in &self.children {
            let (kid_size, kid_sum) = d.aoc_dir_sum();
            my_size += kid_size;
            dir_sum += kid_sum;
        }

        if my_size <= 100000 {
            dir_sum += my_size;
        }
        return (my_size, dir_sum);
    }

    //Walk the dir try to find the smallest directory that, when deleted, will free min_size space.
    // return current directory size and current winning size or 0 if none found yet.
    fn aoc_dir_size_min_above(&self, min_size: usize) -> (usize, usize) {
        let mut my_size = 0;
        let mut min_acceptable_so_far = 0;

        for f in &self.files {
            my_size += f.size;
        }

        for (_, d) in &self.children {
            let (kid_size, kid_min_dirsize) = d.aoc_dir_size_min_above(min_size);
            my_size += kid_size;

            if kid_min_dirsize > 0
                && (min_acceptable_so_far == 0 || kid_min_dirsize < min_acceptable_so_far)
            {
                min_acceptable_so_far = kid_min_dirsize;
            }
        }

        if my_size > min_size && (min_acceptable_so_far == 0 || my_size < min_acceptable_so_far) {
            min_acceptable_so_far = my_size;
        }

        return (my_size, min_acceptable_so_far);
    }
}

use history::*;

/*
the challenge that i struggled with was taking a vector of parsed command history lines
    and turning this into a tree.
I probably should have actually read about ownership and lifetimes before doing AoC in rust :D
One of the challenges here is that not only do i want to build it recursively,
    but I want to consume the stream of tokens in an inner invocation and then resume from
    where i left off on the outer invocation. This forced me into a mutable mutable ref to an
    iterator so i could lend it down the stack and recieve it back in order to continue.
not sure if i could get away with eliding some of the hints, but this makes compiler happy :/

*/
fn build<'h>(
    mut hist: &'h mut Iter<'h, history::Line<'h>>,
    cwd: &mut Dir,
) -> &'h mut Iter<'h, history::Line<'h>> {
    while let Some(line) = hist.next() {
        match *line {
            Line::Dir { name } => {
                cwd.children.insert(name.to_string(), Dir::new(name));
            }

            Line::File { name, size } => {
                let file = File {
                    name: name.to_string(),
                    size: size,
                };

                cwd.files.push(file);
            }

            //this is only done once so we cheat and ignore it.
            Line::Command {
                name: "cd",
                arg: Some("/"),
            }
            | Line::Command {
                name: "ls",
                arg: None,
            } => {}

            //pop the stack
            Line::Command {
                name: "cd",
                arg: Some(".."),
            } => return hist,

            Line::Command {
                name: "cd",
                arg: Some(name),
            } => {
                //get a mutable reference to the directory
                let dir = cwd
                    .children
                    .get_mut(&name.to_string())
                    .expect("trying to enter a directory that does not exist");

                /* find the dir with the same name, then call build inside that dir with the remaining history.
                when we've cd .. back to here, resume processing. */
                hist = build(hist, dir);
            }

            Line::Command {name, arg} => {
                panic!("Unsupported command: {:?} {:?}", name, arg);
            }
        }
    }

    return hist;
}

fn aoc_min_delete(root: &Dir) -> usize {
    /* The total disk space available to the filesystem is 70000000.
    To run the update, you need unused space of at least 30000000.
    You need to find a directory you can delete that will free up enough space to run the update.

    Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update.
    What is the total size of that directory? */

    let du = root.size();

    let current_free = 70000000 - du;
    let min_free = 30000000;

    let additional_to_delete = min_free - current_free;

    let (_, output) = root.aoc_dir_size_min_above(additional_to_delete);

    return output;
}

fn main() {
    let input = fs::read_to_string("./7.input").expect("Error while reading");
    let result = history::parse_input(&input);

    let mut root = Dir {
        name: "/".to_string(),
        files: Vec::new(),
        children: HashMap::new(),
    };

    let h = result.unwrap().1;
    let mut hist = h.iter();
    build(&mut hist, &mut root);

    let (_, output) = root.aoc_dir_sum();
    println!("{:?}", output);

    let output = aoc_min_delete(&root);
    println!("{:?}", output);
}

/* represents the file input. */
mod history {

    use nom::{
        branch::alt,
        bytes::complete::{is_a, is_not, tag},
        combinator::{map, opt},
        multi::many1,
        sequence::{pair, preceded, terminated},
        IResult,
    };

    #[derive(Debug, PartialEq)]
    pub enum Line<'a> {
        Command { name: &'a str, arg: Option<&'a str> },
        File { size: usize, name: &'a str },
        Dir { name: &'a str },
    }

    fn parse_command(i: &str) -> IResult<&str, Line> {
        let name = preceded(tag("$ "), is_not(" \n"));
        let arg = preceded(tag(" "), is_not("\n"));

        let full_line = terminated(pair(name, opt(arg)), tag("\n"));

        map(full_line, |(name, arg)| Line::Command {
            name: name,
            arg: arg,
        })(i)
    }

    fn parse_dir(i: &str) -> IResult<&str, Line> {
        let name = preceded(tag("dir "), is_not(" \n"));
        let full_line = terminated(name, tag("\n"));

        map(full_line, |name| Line::Dir { name: name })(i)
    }

    fn parse_file(i: &str) -> IResult<&str, Line> {
        let size_str = terminated(is_a("1234567890"), tag(" "));
        let name = is_not("\n");

        let full_line = terminated(pair(size_str, name), tag("\n"));

        map(full_line, |(size_str, name)| Line::File {
            name: name,
            size: size_str.parse().unwrap(),
        })(i)
    }

    pub fn parse_input(i: &str) -> IResult<&str, Vec<Line>> {
        many1(alt((parse_file, parse_command, parse_dir)))(i)
    }

    #[cfg(test)]
    mod tests {
        use crate::history::*;

        #[test]
        fn test_parse_command() {
            assert_eq!(
                parse_command("$ cd /\n").unwrap().1,
                Line::Command {
                    name: "cd",
                    arg: Some("/")
                }
            );
            assert_eq!(
                parse_command("$ ls\n").unwrap().1,
                Line::Command {
                    name: "ls",
                    arg: None
                }
            );
        }

        #[test]
        fn test_parse_dir() {
            assert_eq!(parse_dir("dir a\n").unwrap().1, Line::Dir { name: "a" });
        }

        #[test]
        fn test_parse_file() {
            assert_eq!(
                parse_file("14848514 b.txt\n").unwrap().1,
                Line::File {
                    name: "b.txt",
                    size: 14848514usize
                }
            );
        }

        #[test]
        fn test_parse_any() {
            let a = "$ cd /\n$ ls\ndir a\n";
            println!("{:?}", a);
        }
    }
}

#[cfg(test)]
mod test_parsing {
    const PROVIDED_INPUT: &str = include_str!("../7.test");

    use crate::history;
    use crate::*;

    #[test]
    fn test_size_one_layer_deep() {
        let mut d = Dir::new("/");
        d.files.push(File {
            name: "l".to_string(),
            size: 1000,
        });
        d.files.push(File {
            name: "eet".to_string(),
            size: 337,
        });
        assert_eq!(d.size(), 1337);
    }

    #[test]
    fn test_building_one_layer_deep() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
";

        let mut root = Dir::new("/");

        build(
            &mut history::parse_input(&input).unwrap().1.iter(),
            &mut root,
        );

        let mut children = HashMap::new();
        children.insert("a".to_string(), Dir::new("a"));
        children.insert("d".to_string(), Dir::new("d"));

        assert_eq!(
            root,
            Dir {
                name: "/".to_string(),
                children: children,
                files: vec![
                    File {
                        size: 14848514,
                        name: "b.txt".to_string()
                    },
                    File {
                        size: 8504156,
                        name: "c.dat".to_string()
                    },
                ]
            }
        )
    }

    #[test]

    fn test_building_two_layers_deep() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
$ cd a
$ ls
123 test.txt
";

        let mut root = Dir::new("/");

        build(
            &mut history::parse_input(&input).unwrap().1.iter(),
            &mut root,
        );

        let mut children = HashMap::new();
        let mut a = Dir::new("a");
        a.files.push(File {
            size: 123,
            name: "test.txt".to_string(),
        });
        children.insert("a".to_string(), a);

        assert_eq!(
            root,
            Dir {
                name: "/".to_string(),
                children: children,
                files: vec![
                    File {
                        size: 14848514,
                        name: "b.txt".to_string()
                    },
                    File {
                        size: 8504156,
                        name: "c.dat".to_string()
                    },
                ]
            }
        )
    }

    #[test]

    fn test_provided_input() {
        let mut root = Dir::new("/");

        build(
            &mut history::parse_input(&PROVIDED_INPUT).unwrap().1.iter(),
            &mut root,
        );

        let mut verification_root = Dir::new("/");
        verification_root.files.push(File {
            name: "b.txt".to_string(),
            size: 14848514,
        });
        verification_root.files.push(File {
            name: "c.dat".to_string(),
            size: 8504156,
        });

        let mut a = Dir::new("a");
        a.files.push(File {
            size: 29116,
            name: "f".to_string(),
        });
        a.files.push(File {
            size: 2557,
            name: "g".to_string(),
        });
        a.files.push(File {
            size: 62596,
            name: "h.lst".to_string(),
        });

        a.children.insert(
            "e".to_string(),
            Dir {
                name: "e".to_string(),
                files: vec![File {
                    name: "i".to_string(),
                    size: 584,
                }],
                children: HashMap::new(),
            },
        );
        verification_root.children.insert("a".to_string(), a);

        let mut d = Dir::new("d");
        d.files.push(File {
            size: 4060174,
            name: "j".to_string(),
        });
        d.files.push(File {
            size: 8033020,
            name: "d.log".to_string(),
        });
        d.files.push(File {
            size: 5626152,
            name: "d.ext".to_string(),
        });
        d.files.push(File {
            size: 7214296,
            name: "k".to_string(),
        });
        verification_root.children.insert("d".to_string(), d);

        assert_eq!(root, verification_root)
    }

    #[test]
    fn test_size_provided_input() {
        let mut root = Dir::new("/");

        build(
            &mut history::parse_input(&PROVIDED_INPUT).unwrap().1.iter(),
            &mut root,
        );

        assert_eq!(root.size(), 48381165);
    }

    #[test]
    fn test_aoc_size_thing() {
        let mut root = Dir::new("/");

        build(
            &mut history::parse_input(&PROVIDED_INPUT).unwrap().1.iter(),
            &mut root,
        );

        let (_, output) = root.aoc_dir_sum();
        assert_eq!(output, 95437);
    }

    #[test]
    fn test_aoc_min_delete() {
        let mut root = Dir::new("/");

        build(
            &mut history::parse_input(&PROVIDED_INPUT).unwrap().1.iter(),
            &mut root,
        );

        let output = aoc_min_delete(&root);
        assert_eq!(output, 24933642);
    }
}
