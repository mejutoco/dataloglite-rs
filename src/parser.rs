use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, char, not_line_ending, space0},
    combinator::{map, recognize, value},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult, Parser as NomParser,
};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Fact {
    pub name: String,
    pub first: String,
}

#[derive(Debug)]
pub enum DatalogItem {
    Fact(Fact),
    Relation(Relation),
    Rule(Rule),
    Query(Query),
}

#[derive(Debug)]
pub enum NonQueryDatalogItem {
    Fact(Fact),
    Relation(Relation),
    VariableBasedRelation(VariableBasedRelation),
    ConjunctiveQuery(ConjunctiveQuery),
    Rule(Rule),
}

#[derive(Debug)]
pub struct Query {
    pub data: NonQueryDatalogItem,
}

#[derive(Debug)]
pub struct ConjunctiveQuery {
    pub name: String,
    pub first: String,
    pub second: String,
    pub definition: ConjunctiveQueryDefinition,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Relation {
    pub name: String,
    pub first: String,
    pub second: String,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum VariableBasedRelation {
    VariableBasedRelationFirstIsVar(VariableBasedRelationFirstIsVar),
    VariableBasedRelationSecondIsVar(VariableBasedRelationSecondIsVar),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct VariableBasedRelationFirstIsVar {
    pub name: String,
    pub second: String,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct VariableBasedRelationSecondIsVar {
    pub name: String,
    pub first: String,
}

#[derive(Debug)]
pub struct ConjunctiveQueryDefinition {
    pub data: Vec<NonQueryDatalogItem>,
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
        recognize(pair(
            nom::character::complete::satisfy(|c| c.is_ascii_lowercase()),
            many0(alt((
                nom::character::complete::satisfy(|c| c.is_ascii_alphanumeric()),
                nom::character::complete::char('_'),
            ))),
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

pub fn parse_variable_based_relation(input: &str) -> IResult<&str, VariableBasedRelation> {
    let variable_char = "X";
    let (input, name) = parse_name(input)?;
    let (input, _) = char('(')(input)?;
    let (input, (first, second)) = separated_pair(
        parse_argument,
        terminated(char(','), space0),
        parse_argument,
    )
    .parse(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = char('.')(input)?;

    match (&first == variable_char, &second == variable_char) {
        (true, false) => Ok((
            input,
            VariableBasedRelation::VariableBasedRelationFirstIsVar(
                VariableBasedRelationFirstIsVar { name, second },
            ),
        )),
        (false, true) => Ok((
            input,
            VariableBasedRelation::VariableBasedRelationSecondIsVar(
                VariableBasedRelationSecondIsVar { name, first },
            ),
        )),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Fail,
        ))),
    }
}

pub fn parse_query(input: &str) -> IResult<&str, Query> {
    let (input, _) = char('?')(input)?;
    let (input, item) = alt((
        map(parse_fact, NonQueryDatalogItem::Fact),
        map(
            parse_conjunctive_query,
            NonQueryDatalogItem::ConjunctiveQuery,
        ),
        map(
            parse_variable_based_relation,
            NonQueryDatalogItem::VariableBasedRelation,
        ),
        // order is important
        // after parse_variable_based_relation
        map(parse_relation, NonQueryDatalogItem::Relation),
    ))
    .parse(input)?;

    Ok((input, Query { data: item }))
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

pub fn parse_conjunctive_query(input: &str) -> IResult<&str, ConjunctiveQuery> {
    let (input, name) = parse_rule(input)?;
    Ok((
        input,
        ConjunctiveQuery {
            name: name.name,
            first: name.first,
            second: name.second,
            definition: ConjunctiveQueryDefinition {
                data: name
                    .definition
                    .relations
                    .into_iter()
                    .map(|item| match item {
                        DatalogItem::Fact(fact) => NonQueryDatalogItem::Fact(fact),
                        DatalogItem::Relation(rel) => NonQueryDatalogItem::Relation(rel),
                        _ => panic!("Unexpected item in conjunctive query definition"),
                    })
                    .collect(),
            },
        },
    ))
}

pub fn parse_datalog_item(input: &str) -> IResult<&str, DatalogItem> {
    alt((
        map(parse_rule, DatalogItem::Rule),
        map(parse_fact, DatalogItem::Fact),
        map(parse_relation, DatalogItem::Relation),
        map(parse_query, DatalogItem::Query),
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
