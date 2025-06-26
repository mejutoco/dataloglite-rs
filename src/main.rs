use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, space0},
    combinator::map,
    multi::many1,
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug)]
struct ParentRelation {
    parent: String,
    child: String,
}

fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        map(alpha1, |s: &str| s.to_string()),
        char('"'),
    ).parse(input)
}

fn parse_parent_relation(input: &str) -> IResult<&str, ParentRelation> {
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

fn parse_datalog(input: &str) -> IResult<&str, Vec<ParentRelation>> {
    many1(terminated(parse_parent_relation, space0)).parse(input)
}

fn main() {
    let input = r#"parent("Alice", "Bob").
parent("Alice", "Barbara").
parent("Bob", "Charlie").
parent("Bob", "Cindy").
parent("Barbara", "David").
parent("Barbara", "Diana").
parent("Charlie", "Eve").
parent("Cindy", "Frank").
parent("David", "Grace").
parent("Diana", "Henry")."#;

    match parse_datalog(input) {
        Ok((_, relations)) => {
            println!("Parsed relations:");
            for rel in relations {
                println!("{} is parent of {}", rel.parent, rel.child);
            }
        }
        Err(e) => println!("Error parsing: {:?}", e),
    }
}