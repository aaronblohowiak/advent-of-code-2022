use core::slice::Iter;
use std::fs;

use std::include_str;
use std::collections::HashMap;



/*
    Command(cmd name [args])
    Dir (name, &parent)
    File (size name)
*/

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

    //three options: command line, dir name or file size

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
    fn new(s : &str) -> Dir {
        return Dir{
            name: s.to_string(),
            files: vec![],
            children: HashMap::new()
        }
    }
}


#[cfg(test)]
mod test {
    use std::hash::Hash;

    use crate::history;
    use crate::*;

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
        children.insert("a".to_string(),  Dir::new("a"));
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
        a.files.push(File{size: 123, name:"test.txt".to_string()});
        children.insert("a".to_string(),  a);

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
        let input = include_str!("../7.test");

        let mut root = Dir::new("/");

        build(
            &mut history::parse_input(&input).unwrap().1.iter(),
            &mut root,
        );

        let mut verification_root = Dir::new("/");
        verification_root.files.push(File{name: "b.txt".to_string(), size: 14848514});
        verification_root.files.push(File{name: "c.dat".to_string(), size: 8504156});

        let mut a = Dir::new("a");
        a.files.push(File{size: 29116, name:"f".to_string()});
        a.files.push(File{size: 2557, name:"g".to_string()});
        a.files.push(File{size: 62596, name:"h.lst".to_string()});

        a.children.insert("e".to_string(), Dir{
            name: "e".to_string(),
            files: vec![File{name: "i".to_string(), size: 584}],
            children: HashMap::new()
        });
        verification_root.children.insert("a".to_string(),  a);

        let mut d = Dir::new("d");
        d.files.push(File{size: 4060174, name:"j".to_string()});
        d.files.push(File{size: 8033020, name:"d.log".to_string()});
        d.files.push(File{size: 5626152, name:"d.ext".to_string()});
        d.files.push(File{size: 7214296, name:"k".to_string()});
        verification_root.children.insert("d".to_string(),  d);

        assert_eq!(
            root,
            verification_root
        )
    }

}

use history::*;

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
            } => {}

            Line::Command {
                name: "ls",
                arg: None,
            } => {}

            Line::Command {
                name: "cd",
                arg: Some(".."),
            } => return hist,

            Line::Command {
                name: "cd",
                arg: Some(name),
            } => {
                //get a mutable reference to the directory
                let dir = cwd.children.get_mut(&name.to_string()).expect("trying to enter a directory that does not exist");
                /* find the dir with the same name, then call build inside that dir with the remaining history */
                hist = build(hist, dir);
            }

            _ => {
                println!("Not supported {:?}", line);
                return hist;
            }
        }
    }

    return hist;
}

fn main() {
    let input = fs::read_to_string("./7.input").expect("Error while reading");
    let result = history::parse_input(&input);
    println!("{:?}", result);

    let mut root = Dir {
        name: "/".to_string(),
        files: Vec::new(),
        children: HashMap::new(),
    };

    let h = result.unwrap().1;
    let mut hist = h.iter();
    let rest = build(&mut hist, &mut root);
}
