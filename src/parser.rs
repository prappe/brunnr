
use regex::{Regex, Captures};

use abs::Expr::{self, Id, LitInt, LitFloat, LitString, Neg, Mult, Div, Plus, Minus};
use abs::Stm::{self, Vardef, Assign};
use abs::Type;

#[derive(Debug)]
pub struct Line<'a> {
    pub content: &'a str
}

struct ParseRule {
    name: String,
    regex: Regex,
}
pub struct Parser {
    rules: Vec<ParseRule>
}

impl Parser {

    pub fn new() -> Parser {
        let id = r"([:lower:][:alnum:]*)";
        let typ = r"([:upper:][:alnum:]*)";
        let litfloat = r"([:digit:]+\.[:digit:]*)";
        let litstring = r##"(?:["'`])(.*)(?:["'`])"##;
        let litint = r"([:digit:]+)";
        let expr = r"(.*)";

        let parse_patterns = vec![
            ("Vardef", vec![typ, r" ", id, r" = ", expr]),
            ("Assign", vec![id, r" = ", expr]),

            ("Type", vec![typ]),

            ("Id", vec![id]),
            ("LitFloat", vec![litfloat]),
            ("LitInt", vec![litint]),
            ("LitString", vec![litstring]),
            //("Paren", vec![r"\(", r"([^\)]*)", r"\)"]),
            ("Mult", vec![expr, r" \* ", expr]),
            ("Div", vec![expr, r" / ", expr]),
            ("Plus", vec![expr, r" \+ ", expr]),
            ("Minus", vec![expr, r" - ", expr]),
            ("Neg", vec![r"-", expr]),
        ];

        let mut rules = vec![];
        for pp in parse_patterns.iter() {
            let (name, ref pattern_parts) = *pp;
            let mut regex_string = String::new();
            regex_string.push_str("^");
            for part in pattern_parts.iter() {
                regex_string.push_str(*part);
            }
            regex_string.push_str("$");
            let regex = Regex::new(&regex_string[..]).unwrap();
            rules.push(ParseRule {name: String::from(name), regex: regex});
        }
        return Parser {rules: rules};
    }

    pub fn parse<'a>(&'a self, s: Vec<Line<'a>>) -> Vec<Stm> {
        let mut res: Vec<Stm> = vec![];
        for line in s.iter() {
            let l = self.parse_stm((*line).content);
            res.push(l);
        }
        return res;
    }

    fn parse_stm<'a>(&'a self, s: &'a str) -> Stm {
        for rule in self.rules.iter() {
            if rule.regex.is_match(s) {
                let c = rule.regex.captures(s).expect("No captures");
                return match &rule.name[..] {
                    "Vardef" => self.vardef(c),
                    "Assign" => self.assign(c),
                    _ => panic!("Bad command: {}", rule.name)
                };
            }
        }
        panic!("No valid command found: {}", s);
    }

    fn vardef<'a>(&'a self, cap: Captures<'a>) -> Stm {
        let t : &'a str = cap.at(1).unwrap();
        let e1 = self.parse_expr(cap.at(2).unwrap());
        let e2 = self.parse_expr(cap.at(3).unwrap());
        return Vardef(Type(t), e1, e2);
    }

    fn assign<'a>(&'a self, cap: Captures<'a>) -> Stm {
        let e1 = self.parse_expr(cap.at(1).unwrap());
        let e2 = self.parse_expr(cap.at(2).unwrap());
        return Assign(e1, e2);
    }

    fn parse_expr<'a >(&'a self, s: &'a str) -> Expr {
        for rule in self.rules.iter() {
            if rule.regex.is_match(s) {
                let c = rule.regex.captures(s).expect("No captures");
                return match &rule.name[..] {
                    "Id" => self.id(c),
                    "LitFloat" => self.litfloat(c),
                    "LitInt" => self.litint(c),
                    "LitString" => self.litstring(c),
                    //"Paren" => self.paren(c),
                    "Neg" => self.neg(c),
                    "Mult" => self.mult(c),
                    "Div" => self.div(c),
                    "Plus" => self.plus(c),
                    "Minus" => self.minus(c),
                    _ => panic!("Bad expression: {}", rule.name)
                };
            }
        }
        panic!("No valid expression found: {}", s);
    }

    fn id<'a>(&'a self, cap: Captures<'a>) -> Expr {
        let s : &'a str = cap.at(1).unwrap();
        return Id(s);
    }

    fn litint(&self, cap: Captures) -> Expr {
        let i = cap.at(1).unwrap().parse().unwrap();
        return LitInt(i);
    }

    fn litfloat(&self, cap: Captures) -> Expr {
        let f = cap.at(1).unwrap().parse().unwrap();
        return LitFloat(f);
    }

    fn litstring(&self, cap: Captures) -> Expr {
        let s = cap.at(1).unwrap().parse().unwrap();
        return LitString(s);
    }

    fn neg<'a>(&'a self, cap: Captures<'a>) -> Expr {
        let e = self.parse_expr(cap.at(1).unwrap());
        return Neg(Box::new(e));
    }

    fn mult<'a>(&'a self, cap: Captures<'a>) -> Expr {
        let e1 = self.parse_expr(cap.at(1).unwrap());
        let e2 = self.parse_expr(cap.at(2).unwrap());
        return Mult(Box::new(e1), Box::new(e2));
    }

    fn div<'a>(&'a self, cap: Captures<'a>) -> Expr {
        let e1 = self.parse_expr(cap.at(1).unwrap());
        let e2 = self.parse_expr(cap.at(2).unwrap());
        return Div(Box::new(e1), Box::new(e2));
    }

    fn plus<'a>(&'a self, cap: Captures<'a>) -> Expr {
        let e1 = self.parse_expr(cap.at(1).unwrap());
        let e2 = self.parse_expr(cap.at(2).unwrap());
        return Plus(Box::new(e1), Box::new(e2));
    }

    fn minus<'a>(&'a self, cap: Captures<'a>) -> Expr {
        let e1 = self.parse_expr(cap.at(1).unwrap());
        let e2 = self.parse_expr(cap.at(2).unwrap());
        return Minus(Box::new(e1), Box::new(e2));
    }

    /*fn paren<'a>(&'a self, cap: Captures<'a>) -> Expr {
        let e = self.parse_expr(cap.at(1).unwrap());
        return Paren(Box::new(e));
    }*/

}

