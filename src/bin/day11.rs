use std::collections::VecDeque;

const DATA: &str = include_str!("res/day11.txt");

type Operation = Box<dyn Fn(u64) -> u64>;

pub struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test_divisible: u64,
    throw_true: u64,
    throw_false: u64,
    times_inspected: u64,
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("operation", &(self.operation)(2))
            .field("test_divisible", &self.test_divisible)
            .field("throw_true", &self.throw_true)
            .field("throw_false", &self.throw_false)
            .finish()
    }
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_until},
        character::complete::{digit1, multispace0, not_line_ending, u64},
        combinator::{all_consuming, eof, recognize, verify},
        multi::{many1, separated_list0},
        sequence::{preceded, terminated, tuple},
        AsChar, FindSubstring, Finish, IResult, Parser,
    };

    use super::{Monkey, Operation, DATA};

    fn ignore_till_str<'a, O, E>(
        pattern: &'a str,
        parser: impl Parser<&'a str, O, E>,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        E: nom::error::ParseError<&'a str>,
    {
        preceded(ignore_to(pattern), parser)
    }

    fn ignore_till<T, I, O, E>(
        pattern: T,
        parser: impl Parser<I, O, E>,
    ) -> impl FnMut(I) -> IResult<I, O, E>
    where
        I: nom::InputTake + nom::Compare<T> + nom::FindSubstring<T> + Clone,
        T: nom::InputLength + Clone,
        E: nom::error::ParseError<I>,
    {
        preceded(ignore_to(pattern), parser)
    }

    fn ignore_to<T, I, E>(pattern: T) -> impl FnMut(I) -> IResult<I, (), E>
    where
        I: nom::InputTake + nom::Compare<T> + nom::FindSubstring<T> + Clone,
        T: nom::InputLength + Clone,
        E: nom::error::ParseError<I>,
    {
        move |input| {
            tuple((take_until(pattern.clone()), tag(pattern.clone())))(input)
                .map(|(input, _)| (input, ()))
        }
    }

    fn ignore_to_str<'a, E>(pattern: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, (), E>
    where
        E: nom::error::ParseError<&'a str>,
    {
        move |input| tuple((take_until(pattern), tag(pattern)))(input).map(|(input, _)| (input, ()))
    }

    // fn ignore_to_lf<I, T, E>(pattern: T) -> impl FnMut(I) -> IResult<I, (), E>
    // where
    //     I: nom::Compare<T>
    //         + nom::Compare<&'static str>
    //         + nom::FindSubstring<T>
    //         + nom::Offset
    //         + nom::InputIter
    //         + nom::InputTake
    //         + nom::InputLength
    //         + nom::Slice<std::ops::RangeTo<usize>>
    //         + nom::Slice<std::ops::Range<usize>>
    //         + nom::Slice<std::ops::RangeFrom<usize>>
    //         + Clone,
    //     T: nom::InputLength + Clone,
    //     E: nom::error::ParseError<I>,
    //     <I as nom::InputIter>::Item: nom::AsChar,
    // {
    //     move |input| {
    //         let (rest, taken) = recognize(ignore_to(pattern.clone()))(input)?;

    //         let (must_empty, _) = not_line_ending(taken)?;
    //         let _ = eof(must_empty)?;

    //         Ok((rest, ()))
    //     }
    // }

    fn ignore_to_lf<I, T, E>(pattern: T) -> impl FnMut(I) -> IResult<I, (), E>
    where
        I: nom::Compare<T>
            + nom::Compare<&'static str>
            + nom::FindSubstring<T>
            + nom::Offset
            + nom::InputIter
            + nom::InputTake
            + nom::InputLength
            + nom::Slice<std::ops::RangeTo<usize>>
            + nom::Slice<std::ops::Range<usize>>
            + nom::Slice<std::ops::RangeFrom<usize>>
            + Clone,
        T: nom::InputLength + Clone,
        E: nom::error::ParseError<I>,
        <I as nom::InputIter>::Item: nom::AsChar,
        // T: nom::InputLength + Clone + nom::InputTake + nom::InputIter<Item = char>,
        // for<'a> E: nom::error::ParseError<&'a str>,
        // for<'a> <&'a str as nom::InputIter>::Item: nom::AsChar,
        // for<'a> &'a str: FindSubstring<T> + nom::Compare<T>,
    {
        move |input| {
            let (rest, taken) = recognize(ignore_to(pattern.clone()))(input)?;

            let (must_empty, _) = not_line_ending(taken)?;
            let _ = eof(must_empty)?;

            Ok((rest, ()))
        }
    }

    fn ignore_to_lf_str<'a, E>(pattern: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, (), E>
    where
        E: nom::error::ParseError<&'a str>,
    {
        move |input| {
            let (rest, taken) = recognize(ignore_to(pattern))(input)?;

            if !pattern.contains('\n') {
                let _ = all_consuming(not_line_ending)(taken)?;
            }

            Ok((rest, ()))
        }
    }

    #[test]
    fn test_ignore_to_lf() {
        let x = "fuck asdf foobar 12345\nmore stuff";
        // dbg!(ignore_to_lf("foobar")(x).finish());
        // dbg!(ignore_to_lf("foobar")(x).finish());
        assert_eq!(
            ignore_to_lf("foobar ")(x),
            Err(nom::Err::Error(nom::error::Error::new(
                "",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    fn operation(input: &str) -> IResult<&str, Operation> {
        let multiply =
            preceded(tag("* "), u64).map(|v| -> Operation { Box::new(move |old| old * v) });
        let add = preceded(tag("+ "), u64).map(|v| -> Operation { Box::new(move |old| old + v) });
        let square = tag("* old").map(|_| -> Operation { Box::new(move |old| old * old) });
        alt((multiply, add, square))(input)
    }

    fn monkey(input: &str) -> IResult<&str, Monkey> {
        use nom::sequence::Tuple;

        let (input, _) = tuple((multispace0, tag("Monkey "), digit1, tag(":")))(input)?;
        let (input, _) = tuple((multispace0, tag("Starting items: ")))(input)?;
        let (input, items) = separated_list0(tag(", "), u64)(input)?;
        let (input, _) = tuple((multispace0, tag("Operation: new = old ")))(input)?;
        let (input, operation) = operation(input)?;
        let (input, _) = tuple((multispace0, tag("Test: divisible by ")))(input)?;
        let (input, test_divisible) = u64(input)?;
        let (input, _) = tuple((multispace0, tag("If true: throw to monkey ")))(input)?;
        let (input, throw_true) = u64(input)?;
        // let (input, _) = tuple((multispace0, tag("If false: throw to monkey ")))(input)?;
        let (input, _) = ((multispace0, tag("If false: throw to monkey "))).parse(input)?;
        let (input, throw_false) = u64(input)?;

        // let (input, items) = ignore_till("items: ", separated_list0(tag(", "), u64))(input)?;
        // let (input, operation) = ignore_till("new = old ", operation)(input)?;
        // let (input, test_divisible) = ignore_till("divisible by ", u64)(input)?;
        // let (input, throw_true) = ignore_till("monkey ", u64)(input)?;
        // let (input, throw_false) = ignore_till("monkey ", u64)(input)?;

        let res = Monkey {
            items: items.into(),
            operation,
            test_divisible,
            throw_true,
            throw_false,
            times_inspected: 0,
        };
        Ok((input, res))
    }

    pub fn parse() -> Vec<Monkey> {
        let parser = many1(monkey);
        let res = all_consuming(terminated(parser, multispace0))(DATA);

        res.finish().unwrap().1
    }
}

fn sim_monkey(idx: usize, divide_three: bool, modulo: u64, monkeys: &mut [Monkey]) {
    while let Some(item) = monkeys[idx].items.pop_front() {
        monkeys[idx].times_inspected += 1;

        let mut item = (monkeys[idx].operation)(item);
        if divide_three {
            item = item / 3;
        } else {
            item = item % modulo;
        }

        let next_idx = if item % monkeys[idx].test_divisible == 0 {
            monkeys[idx].throw_true
        } else {
            monkeys[idx].throw_false
        } as usize;
        monkeys[next_idx].items.push_back(item);
    }
}

fn sim_rounds(rounds: usize, divide_three: bool, monkeys: &mut [Monkey]) -> u64 {
    let mut modulo = 1;
    for monkey in monkeys.iter() {
        modulo *= monkey.test_divisible;
    }
    dbg!(modulo);

    for _ in 0..rounds {
        for idx in 0..monkeys.len() {
            sim_monkey(idx, divide_three, modulo, monkeys);
        }
    }

    let mut times_inspected: Vec<u64> = monkeys
        .iter()
        .map(|monkey| monkey.times_inspected)
        .collect();
    times_inspected.sort();
    times_inspected.reverse();
    times_inspected[0] * times_inspected[1]
}

fn main() {
    dbg!(sim_rounds(20, true, &mut parser::parse()));
    dbg!(sim_rounds(10_000, false, &mut parser::parse()));
}
