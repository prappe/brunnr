
#[derive(Debug, Clone)]
pub struct Type<'a>(pub &'a str);

#[derive(Debug, Clone)]
pub enum Stm<'a> {
    Vardef(Type<'a>, Expr<'a>, Expr<'a>),
    Assign(Expr<'a>, Expr<'a>)
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Id(&'a str),
    LitFloat(f64),
    LitInt(i64),
    LitString(String),
    //Paren(Box<Expr<'a>>),
    Neg(Box<Expr<'a>>),
    Mult(Box<Expr<'a>>, Box<Expr<'a>>),
    Div(Box<Expr<'a>>, Box<Expr<'a>>),
    Plus(Box<Expr<'a>>, Box<Expr<'a>>),
    Minus(Box<Expr<'a>>, Box<Expr<'a>>)
}
