use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

const DATA: &str = include_str!("res/day13.txt");

#[derive(Eq, Clone)]
pub enum Entry {
    Num(u32),
    List(Vec<Entry>),
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::Num(x) => Display::fmt(x, f),
            Entry::List(x) => f.debug_list().entries(x).finish(),
        }
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        use Entry::*;

        match (self, other) {
            (Num(a), Num(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (&Num(a), List(b)) => (&[Num(a)][..]).cmp(b),
            (List(a), &Num(b)) => (&a[..]).cmp(&[Num(b)][..]),
        }
    }
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{multispace0, newline, u32},
        combinator::{all_consuming, cut, verify},
        error::{context, convert_error, ContextError, ParseError, VerboseError},
        multi::{many1, separated_list0},
        sequence::{delimited, preceded, separated_pair, terminated},
        Finish, IResult, Parser,
    };

    use super::Entry;

    trait Err<'a>: ParseError<&'a str> + ContextError<&'a str> {}
    impl<'a, T> Err<'a> for T where T: ParseError<&'a str> + ContextError<&'a str> {}

    fn entry<'a, E: Err<'a>>(input: &'a str) -> IResult<&'a str, Entry, E> {
        let num = u32.map(Entry::Num);
        let list_inside = separated_list0(tag(","), entry);
        let list = delimited(tag("["), list_inside, tag("]")).map(Entry::List);
        let value = alt((num, list));
        context("entry", value)(input)
    }

    fn packet<'a, E: Err<'a>>(input: &'a str) -> IResult<&'a str, Entry, E> {
        let packet = verify(entry, |e| matches!(e, Entry::List(_)));
        context("packet", packet)(input)
    }

    fn packet_pair<'a, E: Err<'a>>(input: &'a str) -> IResult<&'a str, (Entry, Entry), E> {
        let packet_pair = preceded(
            multispace0,
            separated_pair(packet, cut(newline), cut(packet)),
        );
        context("packet_pair", packet_pair)(input)
    }

    fn verbose<'a, O>(parser: impl Parser<&'a str, O, VerboseError<&'a str>>, input: &'a str) -> O {
        let mut parser = all_consuming(terminated(parser, multispace0));
        match parser.parse(input).finish() {
            Ok((_, res)) => res,
            Err(e) => panic!("{}", convert_error(input, e)),
        }
    }

    pub fn parse(input: &str) -> Vec<(Entry, Entry)> {
        let parser = many1(packet_pair);
        verbose(parser, input)
    }
}

fn main() {
    let input = parser::parse(DATA);

    dbg!((1..)
        .zip(input.iter())
        .filter(|(_, (a, b))| a <= b)
        .map(|(idx, _)| idx)
        .sum::<usize>());

    let dividers = parser::parse("[[2]]\n[[6]]").into_iter().next().unwrap();

    let mut input: Vec<Entry> = input
        .into_iter()
        .flat_map(|(a, b)| [a, b].into_iter())
        .collect();
    input.push(dividers.0.clone());
    input.push(dividers.1.clone());
    input.sort();
    // input.iter().for_each(|x| println!("{:?}", x));

    let find = |e| input.binary_search(e).unwrap() + 1;
    dbg!(find(&dividers.0) * find(&dividers.1));
}
