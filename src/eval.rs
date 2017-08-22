
use abs::Type;
use abs::Expr;
use abs::Expr::{Id, LitInt, LitFloat, LitString, Neg, Mult, Div, Plus, Minus};
use abs::Stm;
use abs::Stm::{Vardef, Assign};

use env::EnvI;
use env::EnvF;
use env::EnvS;

pub struct Eval<'a> {
    envi: EnvI<'a>,
    envf: EnvF<'a>,
    envs: EnvS<'a>,
}

impl<'a> Eval<'a> {

    pub fn new() -> Eval<'a> {
        Eval {envi: EnvI::new(),envf: EnvF::new(),envs: EnvS::new()}
    }

    pub fn print_env(&self) {
        println!("Int Environment:\n{:?}", self.envi);
        println!("Float Environment:\n{:?}", self.envf);
        println!("String Environment:\n{:?}", self.envs);
    }

    pub fn exec_stm(&mut self, stm: Stm<'a>) {
        match stm {
            Vardef(t, Id(s), e) => {
                let Type(ty) = t;
                if ty == "Int" {
                    let x = self.evali(e);
                    self.envi.add(s, x);
                    self.envf.remove(s);
                    self.envs.remove(String::from(s));
                } else if ty == "Float" {
                    let x = self.evalf(e);
                    self.envi.remove(s);
                    self.envf.add(s, x);
                    self.envs.remove(String::from(s));
                } else if ty == "String" {
                    let x = self.evals(e).to_string();
                    self.envi.remove(s);
                    self.envf.remove(s);
                    self.envs.add(s, x);
                }
                
            },
            Assign(Id(s), e) => {
                if let Some(_) = self.envi.lookup(s) {
                    let x = self.evali(e);
                    self.envi.add(s, x);
                } else if let Some(_) = self.envf.lookup(s) {
                    let x = self.evalf(e);
                    self.envf.add(s, x);
                } else if let Some(_) = self.envs.lookup(s) {
                    let x = self.evals(e);
                    self.envs.add(s, x);
                } else {
                    panic!("Variable {:?} does not exist", s);
                }
            },
            _ => panic!("Unknown stm: {:?} in exec", stm)
        };
    }

    fn evali(&mut self, expr: Expr<'a>) -> i64 {
        match expr {
            Id(s) => match self.envi.lookup(s) {
                Some(x) => *x,
                _ => match self.envf.lookup(s) {
                    Some(x) => *x as i64,
                    _ => panic!("Could not evaluate {:?} as Integer", expr)
                }
            },
            LitInt(i) => i,
            LitFloat(f) => f as i64,
            //Paren(e) => self.evali(*e),
            Neg(e) => - self.evali(*e),
            Mult(e1, e2) => self.evali(*e1) * self.evali(*e2),
            Div(e1, e2) => self.evali(*e1) / self.evali(*e2),
            Plus(e1, e2) => self.evali(*e1) + self.evali(*e2),
            Minus(e1, e2) => self.evali(*e1) - self.evali(*e2),
            _ => panic!("Could not evaluate {:?} as Integer", expr)
        }
    }
    
    fn evalf(&mut self, expr: Expr<'a>) -> f64 {
        match expr {
            Id(s) => match self.envf.lookup(s) {
                Some(x) => *x,
                _ => match self.envi.lookup(s) {
                    Some(x) => *x as f64,
                    _ => panic!("Could not evaluate {:?} as Float", expr)
                }
            },
            LitInt(i) => i as f64,
            LitFloat(f) => f,
            //Paren(e) => self.evalf(*e),
            Neg(e) => - self.evalf(*e),
            Mult(e1, e2) => self.evalf(*e1) * self.evalf(*e2),
            Div(e1, e2) => self.evalf(*e1) / self.evalf(*e2),
            Plus(e1, e2) => self.evalf(*e1) + self.evalf(*e2),
            Minus(e1, e2) => self.evalf(*e1) - self.evalf(*e2),
            _ => panic!("Could not evaluate {:?} as Float", expr)
        }
    }

    fn evals(&mut self, expr: Expr<'a>) -> String {
        match expr {
            LitString(s) => s,
            LitInt(i) => String::from(i),
            LitFloat(f) => String::from(f),
            LitBool(b) => String::from(b),
            Id(s) => match self.envs.lookup(s) {
                Some(x) => String::from(x),
                _ => match self.envi.lookup(s) {
                    Some(x) => String::from(x),
                    _ => match self.envf.lookup(s) {
                       Some(x) => String::from(x),
                        //_ => match self.envb.lookup(s) {
                        //   Some(x) => String::from(x),
                            _ => panic!("Could not evaluate {:?} as String", expr)
                        //}
                    }
                }
            },
            Plus(e1, e2) => format!("{}{}", self.evals(*e1), self.evals(*e2)),
            _ => panic!("Could not evaluate {:?} as String", expr)
        }
    }

    /*fn evalb(&mut self, expr: Expr<'a>) -> bool {
        match expr {
            LitBool(b) => b,
            LitString(s) => s.to_lowercase() == "true" || s.to_lowercase() == "yes",
            Id(s) => match self.envb.lookup(s) {
                Some(x) => x,
                _ => match self.envs.lookup(s) {
                    Some(x) => x.to_lowercase() == "true" || x.to_lowercase() == "yes",
                    _ => panic!("Could not evaluate {:?} as Boolean", expr)
                }
            },
        }
    }*/
}
