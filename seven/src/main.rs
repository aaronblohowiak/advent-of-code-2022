use std::fs;

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
    pub enum CommandHistory<'a> {
        Command { name: &'a str, arg: Option<&'a str> },
        File { size: usize, name: &'a str },
        Dir { name: &'a str },
    }

    //three options: command line, dir name or file size

    fn parse_command(i: &str) -> IResult<&str, CommandHistory> {
        let name = preceded(tag("$ "), is_not(" \n"));
        let arg = preceded(tag(" "), is_not("\n"));

        let full_line = terminated(pair(name, opt(arg)), tag("\n"));

        map(full_line, |(name, arg)| CommandHistory::Command {
            name: name,
            arg: arg,
        })(i)
    }

    fn parse_dir(i: &str) -> IResult<&str, CommandHistory> {
        let name = preceded(tag("dir "), is_not(" \n"));
        let full_line = terminated(name, tag("\n"));

        map(full_line, |name| CommandHistory::Dir { name: name })(i)
    }

    fn parse_file(i: &str) -> IResult<&str, CommandHistory> {
        let size_str = terminated(is_a("1234567890"), tag(" "));
        let name = is_not("\n");

        let full_line = terminated(pair(size_str, name), tag("\n"));

        map(full_line, |(size_str, name)| CommandHistory::File {
            name: name,
            size: size_str.parse().unwrap(),
        })(i)
    }

    pub fn parse_input(i: &str) -> IResult<&str, Vec<CommandHistory>> {
        many1(alt((parse_file, parse_command, parse_dir)))(i)
    }

    #[cfg(test)]
    mod tests {
        use crate::history::*;

        #[test]
        fn test_parse_command() {
            assert_eq!(
                parse_command("$ cd /\n").unwrap().1,
                CommandHistory::Command {
                    name: "cd",
                    arg: Some("/")
                }
            );
            assert_eq!(
                parse_command("$ ls\n").unwrap().1,
                CommandHistory::Command {
                    name: "ls",
                    arg: None
                }
            );
        }

        #[test]
        fn test_parse_dir() {
            assert_eq!(
                parse_dir("dir a\n").unwrap().1,
                CommandHistory::Dir { name: "a" }
            );
        }

        #[test]
        fn test_parse_file() {
            assert_eq!(
                parse_file("14848514 b.txt\n").unwrap().1,
                CommandHistory::File {
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

// fn parse_input(i: &str) -> IResult<&str, Vec<Any>> {
//     many1())(i)
// }

#[derive(Debug, PartialEq)]

struct File<'a> {
    size: usize,
    name: &'a str,
}

#[derive(Debug, PartialEq)]
struct Dir<'a> {
    name: &'a str,
    files: Vec<File<'a>>,
    children: Vec<Dir<'a>>,
}

#[cfg(test)]
mod test {
    use crate::history;
    use crate::*;

    #[test]

    fn test_initial_building() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
";

        let root = Dir {
            name: "/",
            files: Vec::new(),
            children: Vec::new(),
        };

        build(history::parse_input(&input).unwrap().1, &root);

        assert_eq!(
            root,
            Dir {
                name: "/",
                children: vec![
                    Dir {
                        name: "a",
                        children: vec![],
                        files: vec![]
                    },
                    Dir {
                        name: "d",
                        children: vec![],
                        files: vec![]
                    }
                ],
                files: vec![
                    File {
                        size: 14848514,
                        name: "b.txt"
                    },
                    File {
                        size: 8504156,
                        name: "c.dat"
                    },
                ]
            }
        )
    }
}

fn build(_hist: Vec<history::CommandHistory>, _cwd: &Dir) {}

fn main() {
    let input = fs::read_to_string("./7.input").expect("Error while reading");
    let result = history::parse_input(&input);
    println!("{:?}", result);

    let root = Dir {
        name: "/",
        files: Vec::new(),
        children: Vec::new(),
    };

    build(result.unwrap().1, &root);
}
