extern crate nom;

use nom::{
    bytes::complete::{tag, take, take_until, is_a},
    combinator::{complete, opt, rest},
    sequence::tuple
};

fn combine_tag(txt: &str) -> nom::IResult<&str, &str> {
    let (txt, (done, _)) = tuple((
        tag("x"),
        tag(" "),
    ))(txt)?;

    Ok((txt, done))
}

fn combine_priority(txt: &str) -> nom::IResult<&str, &str> {
    let (txt, (_, priority, _)) = tuple((
        tag("("),
        take(1usize),
        tag(") "),
    ))(txt)?;

    Ok((txt, priority))
}

fn combine_date(txt: &str) -> nom::IResult<&str, &str> {
    let (txt, (year, _, month, _, day, _)) = tuple((
        take(4usize),
        tag("-"),
        take(2usize),
        tag("-"),
        take(2usize),
        tag(" "),
    ))(txt)?;

    Ok((txt, year))
}

fn combine_project(txt: &str) -> nom::IResult<&str, &str> {
    // let (txt, (project, rest)) = tuple((
    //     // alt((tag("@"), tag("+"))),
    //     is_a("@+"),
    //     rest
    // ))(txt)?;

    let (txt, project) = take_until("@|+")(txt)?;

    Ok((txt, project))
}

pub fn parse(txt: &str) -> nom::IResult<&str, &str> {
    let (txt, (done, priority, created, finished, project, rest)) = tuple((
        opt(complete(combine_tag)),
        opt(complete(combine_priority)),
        opt(complete(combine_date)),
        opt(complete(combine_date)),
        opt(complete(combine_project)),
        rest
    ))(txt)?;

    println!("{:?} {:?} {:?} {:?} {:?} {:?}", done, priority, created, finished, project, rest);

    Ok((txt, rest))
}