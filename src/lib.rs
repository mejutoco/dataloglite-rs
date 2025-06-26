use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, space0},
    combinator::{map, recognize},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
    Parser as NomParser,
};

#[derive(Debug)]
pub struct Fact {
    pub name: String,
    pub first: String,
}

#[derive(Debug)]
pub enum DatalogItem {
    Fact(Fact),
    Relation(Relation),
    Rule(Rule),
}

#[derive(Debug)]
pub struct Relation {
    pub name: String,
    pub first: String,
    pub second: String,
}

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub first: String,
    pub second: String,
    pub relations: Vec<Relation>,
}

pub fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        map(alpha1, |s: &str| s.to_string()),
        char('"'),
    ).parse(input)
}

pub fn parse_variable(input: &str) -> IResult<&str, String> {
    map(
        recognize(
            preceded(
                nom::character::complete::satisfy(|c| c.is_ascii_uppercase()),
                nom::character::complete::alphanumeric0
            )
        ),
        |s: &str| s.to_string()
    ).parse(input)
}

pub fn parse_argument(input: &str) -> IResult<&str, String> {
    alt((
        parse_quoted_string,
        parse_variable,
    )).parse(input)
}

pub fn parse_name(input: &str) -> IResult<&str, String> {
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
    let (input, name) = parse_name(input)?;
    let (input, _) = char('(')(input)?;
    let (input, (first, second)) = separated_pair(
        parse_quoted_string,
        terminated(char(','), space0),
        parse_quoted_string,
    ).parse(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = char('.')(input)?;

    Ok((input, Relation { name, first, second }))
}

pub fn parse_fact(input: &str) -> IResult<&str, Fact> {
    let (input, name) = parse_name(input)?;
    let (input, _) = char('(')(input)?;
    let (input, first) = parse_argument(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = char('.')(input)?;

    Ok((input, Fact { name, first }))
}

pub fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, name) = parse_name(input)?;
    let (input, _) = char('(')(input)?;
    let (input, first) = parse_argument(input)?;
    let (input, _) = terminated(char(','), space0).parse(input)?;
    let (input, second) = parse_argument(input)?;
    let (input, _) = terminated(char(')'), space0).parse(input)?;

    let (input, _) = tag(":-")(input)?;
    let (input, relations) = separated_list1(
        terminated(char(','), space0),
        parse_relation
    ).parse(input)?;
    let (input, _) = char('.')(input)?;

    Ok((input, Rule { name, first, second, relations }))
}

pub fn parse_datalog_item(input: &str) -> IResult<&str, DatalogItem> {
    alt((
        map(parse_fact, DatalogItem::Fact),
        map(parse_relation, DatalogItem::Relation),
        map(parse_rule, DatalogItem::Rule),
    )).parse(input)
}

pub fn parse_datalog(input: &str) -> IResult<&str, Vec<DatalogItem>> {
    many0(terminated(
        parse_datalog_item,
        nom::character::complete::multispace0
    )).parse(input)
}
