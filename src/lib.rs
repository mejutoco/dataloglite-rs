use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, char, not_line_ending, space0},
    combinator::{map, recognize, value},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser as NomParser,
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
    pub definition: RuleDefinition,
}

#[derive(Debug)]
pub struct RuleDefinition {
    pub relations: Vec<DatalogItem>,
}

pub fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    delimited(char('"'), map(alpha1, |s: &str| s.to_string()), char('"')).parse(input)
}

pub fn parse_variable(input: &str) -> IResult<&str, String> {
    map(
        recognize(preceded(
            nom::character::complete::satisfy(|c| c.is_ascii_uppercase()),
            nom::character::complete::alphanumeric0,
        )),
        |s: &str| s.to_string(),
    )
    .parse(input)
}

pub fn parse_argument(input: &str) -> IResult<&str, String> {
    alt((parse_quoted_string, parse_variable)).parse(input)
}

pub fn parse_name(input: &str) -> IResult<&str, String> {
    map(
        recognize(preceded(
            nom::character::complete::satisfy(|c| c.is_ascii_lowercase()),
            nom::character::complete::alpha0,
        )),
        |s: &str| s.to_string(),
    )
    .parse(input)
}

pub fn parse_relation(input: &str) -> IResult<&str, Relation> {
    let (input, name) = parse_name(input)?;
    let (input, _) = char('(')(input)?;
    let (input, (first, second)) = separated_pair(
        parse_quoted_string,
        terminated(char(','), space0),
        parse_quoted_string,
    )
    .parse(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = char('.')(input)?;

    Ok((
        input,
        Relation {
            name,
            first,
            second,
        },
    ))
}

pub fn parse_fact(input: &str) -> IResult<&str, Fact> {
    let (input, name) = parse_name(input)?;
    let (input, _) = char('(')(input)?;
    let (input, first) = parse_argument(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = char('.')(input)?;

    Ok((input, Fact { name, first }))
}

pub fn parse_relation_or_fact(input: &str) -> IResult<&str, DatalogItem> {
    alt((
        map(parse_relation_with_vars, DatalogItem::Relation),
        map(parse_fact_with_var, DatalogItem::Fact),
    ))
    .parse(input)
}

pub fn parse_relation_with_vars(input: &str) -> IResult<&str, Relation> {
    let (input, name) = parse_name(input)?;
    let (input, _) = char('(')(input)?;
    let (input, (first, second)) = separated_pair(
        parse_argument,
        terminated(char(','), space0),
        parse_argument,
    )
    .parse(input)?;
    let (input, _) = char(')')(input)?;

    Ok((
        input,
        Relation {
            name,
            first,
            second,
        },
    ))
}

pub fn parse_fact_with_var(input: &str) -> IResult<&str, Fact> {
    let (input, name) = parse_name(input)?;
    let (input, _) = char('(')(input)?;
    let (input, first) = parse_argument(input)?;
    let (input, _) = char(')')(input)?;

    Ok((input, Fact { name, first }))
}

pub fn parse_rule_definition(input: &str) -> IResult<&str, RuleDefinition> {
    let (input, relations) =
        separated_list1(terminated(char(','), space0), parse_relation_or_fact).parse(input)?;

    Ok((input, RuleDefinition { relations }))
}

pub fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, name) = parse_name(input)?;
    let (input, _) = char('(')(input)?;
    let (input, first) = parse_argument(input)?;
    let (input, _) = terminated(char(','), space0).parse(input)?;
    let (input, second) = parse_argument(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = delimited(space0, tag(":-"), space0).parse(input)?;
    let (input, definition) = parse_rule_definition(input)?;
    let (input, _) = char('.')(input)?;

    Ok((
        input,
        Rule {
            name,
            first,
            second,
            definition,
        },
    ))
}

pub fn parse_datalog_item(input: &str) -> IResult<&str, DatalogItem> {
    alt((
        map(parse_fact, DatalogItem::Fact),
        map(parse_relation, DatalogItem::Relation),
        map(parse_rule, DatalogItem::Rule),
    ))
    .parse(input)
}

pub fn parse_line_comment(input: &str) -> IResult<&str, ()> {
    value(
        (), // Output is thrown away
        (tag("//"), not_line_ending),
    )
    .parse(input)
}

pub fn parse_block_comment(input: &str) -> IResult<&str, ()> {
    value(
        (), // Output is thrown away
        (tag("/*"), take_until("*/"), tag("*/")),
    )
    .parse(input)
}

pub fn parse_comment(input: &str) -> IResult<&str, ()> {
    alt((parse_line_comment, parse_block_comment)).parse(input)
}

pub fn parse_datalog(input: &str) -> IResult<&str, Vec<DatalogItem>> {
    let (input, _) = many0(alt((
        value((), nom::character::complete::multispace1),
        parse_comment,
    )))
    .parse(input)?;

    many0(terminated(
        parse_datalog_item,
        many0(alt((
            value((), nom::character::complete::multispace1),
            parse_comment,
        ))),
    ))
    .parse(input)
}
