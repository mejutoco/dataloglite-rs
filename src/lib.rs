use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, space0},
    combinator::map,
    multi::many1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
    Parser as NomParser,
};

#[derive(Debug)]
pub struct ParentRelation {
    pub parent: String,
    pub child: String,
}

pub fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        map(alpha1, |s: &str| s.to_string()),
        char('"'),
    ).parse(input)
}

pub fn parse_parent_relation(input: &str) -> IResult<&str, ParentRelation> {
    let (input, _) = tag("parent")(input)?;
    let (input, _) = char('(')(input)?;
    let (input, (parent, child)) = separated_pair(
        parse_quoted_string,
        terminated(char(','), space0),
        parse_quoted_string,
    ).parse(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = char('.')(input)?;

    Ok((input, ParentRelation { parent, child }))
}

pub fn parse_datalog(input: &str) -> IResult<&str, Vec<ParentRelation>> {
    many1(terminated(
        parse_parent_relation,
        // Consume both spaces and newlines after each relation
        nom::character::complete::multispace0
    )).parse(input)
}
