const DATA: &str = include_str!("res/day71.txt");

mod parser {
    use nom::bytes::complete::is_a;
    use nom::character::complete::{char, newline, u32};
    use nom::combinator::eof;
    use nom::multi::separated_list0;
    use nom::sequence::{pair, separated_pair, terminated};
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        character::complete::{alpha1, digit1},
        combinator::map_res,
        sequence::preceded,
        Finish, IResult, Parser,
    };

    #[derive(Debug)]
    pub enum Path<'a> {
        Up,
        Root,
        Subdir(&'a str),
    }

    fn cd_path(input: &str) -> IResult<&str, Path> {
        let up_dir = tag("..").map(|_| Path::Up);
        let root_dir = tag("/").map(|_| Path::Root);
        let subdir = alpha1.map(Path::Subdir);
        alt((up_dir, root_dir, subdir))(input)
    }

    fn command_cd(input: &str) -> IResult<&str, Path> {
        let cd = tag("cd ");
        preceded(cd, cd_path)(input)
    }

    #[derive(Debug)]
    pub enum Ls<'a> {
        Dir(&'a str),
        File(u32, &'a str),
    }

    fn ls_entry_dir(input: &str) -> IResult<&str, &str> {
        let dir_tag = tag("dir ");
        let dir_name = alpha1;
        preceded(dir_tag, dir_name)(input)
    }

    fn ls_entry_file(input: &str) -> IResult<&str, (u32, &str)> {
        let file_size = u32;
        let file_name = take_while1(|c: char| c.is_alphabetic() || c == '.');
        separated_pair(file_size, char(' '), file_name)(input)
    }

    fn ls_entry(input: &str) -> IResult<&str, Ls> {
        let dir = ls_entry_dir.map(Ls::Dir);
        let file = ls_entry_file.map(|(size, name)| Ls::File(size, name));
        alt((dir, file))(input)
    }

    fn command_ls(input: &str) -> IResult<&str, Vec<Ls>> {
        let ls = tag("ls\n");
        let lines = separated_list0(newline, ls_entry);

        preceded(ls, lines)(input)
    }

    #[derive(Debug)]
    pub enum Command<'a> {
        Cd(Path<'a>),
        Ls(Vec<Ls<'a>>),
    }

    fn command(input: &str) -> IResult<&str, Command> {
        let cd = command_cd.map(Command::Cd);
        let ls = command_ls.map(Command::Ls);
        let command = alt((cd, ls));

        preceded(tag("$ "), command)(input)
    }

    fn commands(input: &str) -> IResult<&str, Vec<Command>> {
        let commands = separated_list0(newline, command);
        let terminate = pair(is_a("\n"), eof);

        terminated(commands, terminate)(input)
    }

    pub fn parse(input: &str) -> Vec<Command> {
        commands(input).finish().unwrap().1
    }

    #[allow(unused_macros)]
    macro_rules! test {
        ($expression:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
            match $expression.finish() {
                res => assert!(
                    matches!(res, $( $pattern )|+ $( if $guard )?),
                    "`{}` evaluated to `{:?}`, didnt match `{}`",
                    stringify!($expression),
                    res,
                    stringify!($( $pattern )|+ $( if $guard )?),
                )
            }
        };
    }

    #[test]
    fn test_command_cd() {
        use Path::*;
        test!(command_cd("cd foobar"), Ok(("", Subdir("foobar"))));
        test!(command_cd("cd .."), Ok(("", Up)));
        test!(command_cd("cd /"), Ok(("", Root)));
        test!(command_cd("cd "), Err(_));
        test!(command_cd("cd \n"), Err(_));
    }

    #[test]
    fn test_ls_entry_dir() {
        test!(ls_entry_dir("dir "), Err(_));
        test!(ls_entry_dir("dir foobar"), Ok(("", "foobar")));
    }

    #[test]
    fn test_ls_entry_file() {
        test!(ls_entry_file("dir "), Err(_));
        test!(ls_entry_file("1234"), Err(_));
        test!(ls_entry_file("1234 asdf"), Ok(("", (1234, "asdf"))));
        test!(ls_entry_file("1234  asdf"), Err(_));
        test!(ls_entry_file("-1234 asdf"), Err(_));
    }
}

fn main() {
    dbg!(parser::parse(DATA));
}
