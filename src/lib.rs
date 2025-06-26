use nom::{
    character::complete::{alpha1, char, space0},
    combinator::{map, recognize},
    multi::many1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
    Parser as NomParser,
};

#[derive(Debug)]
pub struct Relation {
    pub relationship: String,
    pub first: String,
    pub second: String,
}

pub fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        map(alpha1, |s: &str| s.to_string()),
        char('"'),
    ).parse(input)
}

pub fn parse_relationship_name(input: &str) -> IResult<&str, String> {
    map(
        recognize(
            preceded(
                nom::character::complete::satisfy(|c| c.is_ascii_lowercase()),
                nom::character::complete::alpha0
            )
        ),
        |s: &str| s.to_string()
    ).parse(input)
}

pub fn parse_relation(input: &str) -> IResult<&str, Relation> {
    let (input, relationship) = parse_relationship_name(input)?;
    let (input, _) = char('(')(input)?;
    let (input, (first, second)) = separated_pair(
        parse_quoted_string,
        terminated(char(','), space0),
        parse_quoted_string,
    ).parse(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = char('.')(input)?;

    Ok((input, Relation { relationship, first, second }))
}

pub fn parse_datalog(input: &str) -> IResult<&str, Vec<Relation>> {
    many1(terminated(
        parse_relation,
        nom::character::complete::multispace0
    )).parse(input)
}
