use logos::Lexer;

use super::lexer::Token;

#[derive(Debug)]
pub enum KeywordConstant {
    ConsTrue,
    ConsFalse,
    ConsNULL,
    ConsThis,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Neg,
    Not,
}

#[derive(Debug)]
pub struct SubroutineCall {
    var: Option<String>,
    name: String,
    exprs: Vec<Expression>,
}

#[derive(Debug)]
pub enum Term {
    IntegerCons(u16),
    StringCons(String),
    KeywordCons(KeywordConstant),
    VarName(String),
    VarIndex(String, Box<Expression>),
    Call(SubroutineCall),
    Expr(Box<Expression>),
    UnaryOp(UnaryOperator, Box<Term>)
}

#[derive(Debug)]
pub struct Expression {
    lhs: Term,
    op: Option<char>,
    rhs: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct LetStatement {
    var_name: String,
    index_expr: Option<Expression>,
    expr: Expression,
}

#[derive(Debug)]
pub struct IfStatement {
    expr: Expression,
    if_stat: Vec<Statement>,
    else_stat: Option<Vec<Statement>>,
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    If(IfStatement),
    While(Expression, Vec<Statement>),
    Do(SubroutineCall),
    Return(Option<Expression>),
}

#[derive(Debug, Copy, Clone)]
pub enum ClassVarType {
    Static,
    Field,
}

#[derive(Debug)]
pub struct ClassVarDec {
    ty: ClassVarType,
    var_type: String,
    var_name: String,
}

#[derive(Debug)]
pub enum SubroutineType {
    Constructor,
    Function,
    Method,
}

#[derive(Debug)]
pub struct VariableDef {
    var_type: String,
    var_name: String,
}

/// subroutine: ('constructor'|'function'|'method') ('void'|type) name
///             '(' parameterList ')' body
/// body: '{' varDec* statements '}'
/// statements: statement*
#[derive(Debug)]
pub struct SubroutineDec {
    sub_type: SubroutineType,
    ret_type: Option<String>,
    name: String,
    params: Vec<VariableDef>,
    vars: Vec<VariableDef>,
    statements: Vec<Statement>,
}

/// 'class' className '{' classVarDec* subroutineDec* '}'
#[derive(Debug)]
pub struct JackClass {
    name: String,
    vars: Vec<ClassVarDec>,
    subroutines: Vec<SubroutineDec>,
}

struct PeekToken<'a> {
    tokens: &'a mut Lexer<'a, Token>,
    cur: Option<Token>,
}

impl<'a> PeekToken<'a> {
    fn next(&mut self) -> Option<Token> {
        if self.cur.is_none() {
            self.tokens.next()
        } else {
            self.cur.take()
        }
    }

    fn peek(&mut self) -> &Option<Token> {
        if self.cur.is_none() {
            self.cur = self.tokens.next();
        }
        &self.cur
    }

    fn parser_panic(&mut self) -> ! {
        panic!("Unexpected token in {:?} {}", self.tokens.span(), self.tokens.slice());
    }

    fn eat(&mut self, expected: Token) {
        match self.next() {
            Some(t) => {
                if t != expected {
                    self.parser_panic();
                }
            },
            None => self.parser_panic(),
        }
    }

    fn assert_ident(&mut self) -> String {
        match self.next() {
            Some(Token::Ident(id)) => String::from(id),
            _ => self.parser_panic(),
        }
    }

    fn assert_keyword(&mut self, expected: &str) {
        match self.next() {
            Some(Token::Ident(id)) => {
                if id != expected {
                    self.parser_panic()
                }
            },
            _ => self.parser_panic(),
        }
    }

    fn assert_symbol(&mut self, expected: char) {
        match self.next() {
            Some(Token::Symbol(sym)) => {
                if sym != expected {
                    self.parser_panic()
                }
            },
            _ => self.parser_panic(),
        }
    }
}

fn cls_var(tokens: &mut PeekToken, params: &mut Vec<ClassVarDec>, ty: ClassVarType) {
    let var_type = tokens.assert_ident();
    
    loop {
        let var_name = tokens.assert_ident();
        params.push(ClassVarDec{ty, var_type: var_type.clone(), var_name});

        match tokens.next() {
            Some(Token::Symbol(',')) => continue,
            Some(Token::Symbol(';')) => break,
            _ => tokens.parser_panic(),
        };
    }
}

fn class_vars(tokens: &mut PeekToken, vars: &mut Vec<ClassVarDec>) {
    loop {
        let ty = match tokens.peek() {
            Some(Token::Ident(id)) => {
                match id.as_str() {
                    "static" => ClassVarType::Static,
                    "field" => ClassVarType::Field,
                    _ => break
                }
            },
            _ => break,
        };
        tokens.next();

        cls_var(tokens, vars, ty);
    }
}

fn sub_params(tokens: &mut PeekToken, params: &mut Vec<VariableDef>) {
    loop {
        let var_type = match tokens.peek() {
            Some(Token::Ident(_)) => tokens.assert_ident(),
            _ => break,
        };

        let var_name = tokens.assert_ident();
        params.push(VariableDef{var_type, var_name});

        match tokens.peek() {
            Some(Token::Symbol(',')) => tokens.next(),
            _ => break,
        };
    }
}

fn sub_var(tokens: &mut PeekToken, params: &mut Vec<VariableDef>) {
    tokens.assert_keyword("var");
    let var_type = tokens.assert_ident();
    
    loop {
        let var_name = tokens.assert_ident();
        params.push(VariableDef{var_type: var_type.clone(), var_name});

        match tokens.next() {
            Some(Token::Symbol(',')) => continue,
            Some(Token::Symbol(';')) => break,
            _ => tokens.parser_panic(),
        };
    }
}

fn sub_vars(tokens: &mut PeekToken, params: &mut Vec<VariableDef>) {
    loop {
        match tokens.peek() {
            Some(Token::Ident(id)) => {
                if id == "var" {
                    sub_var(tokens, params);
                } else {
                    break
                }
            }
            _ => break,
        }
    }
}

/// (expr (',' expr)*)?
fn sub_exprs(tokens: &mut PeekToken, params: &mut Vec<Expression>) {
    tokens.assert_symbol('(');
    if matches!(tokens.peek(), Some(Token::Symbol(')'))) {
        tokens.next();
        return
    }
    loop {
        let expr = expression(tokens);
        params.push(expr);

        match tokens.next() {
            Some(Token::Symbol(')')) => break,
            Some(Token::Symbol(',')) => continue,
            _ => tokens.parser_panic(),
        }
    }
}

/// int | string | keyword | var | var '[' expr ']' | subroutineCall
/// '(' expr ')' | unaryOp term
/// 
/// subroutineCall: name '(' exprs ')'
///                 cls '.' name '(' exprs ')'
fn eat_term(tokens: &mut PeekToken) -> Term {
    match tokens.next() {
        Some(Token::Number(num)) => Term::IntegerCons(num),
        Some(Token::Str(string)) => Term::StringCons(string),
        Some(Token::Symbol('(')) => {
            let expr = expression(tokens);
            tokens.assert_symbol(')');
            Term::Expr(Box::new(expr))
        },
        Some(Token::Symbol('-')) => {
            let term = eat_term(tokens);
            Term::UnaryOp(UnaryOperator::Neg, Box::new(term))
        },
        Some(Token::Symbol('~')) => {
            let term = eat_term(tokens);
            Term::UnaryOp(UnaryOperator::Not, Box::new(term))
        },
        Some(Token::Ident(id)) => {
            match id.as_str() {
                "true" => Term::KeywordCons(KeywordConstant::ConsTrue),
                "false" => Term::KeywordCons(KeywordConstant::ConsFalse),
                "null" => Term::KeywordCons(KeywordConstant::ConsNULL),
                "this" => Term::KeywordCons(KeywordConstant::ConsThis),
                _ => {
                    match tokens.peek() {
                        Some(Token::Symbol('[')) => {
                            tokens.next();
                            let expr = expression(tokens);
                            tokens.assert_symbol(']');
                            Term::VarIndex(id, Box::new(expr))
                        },
                        Some(Token::Symbol('.')) => {
                            tokens.next();
                            let name = tokens.assert_ident();
                            let mut exprs = Vec::new();
                            sub_exprs(tokens, &mut exprs);
                            Term::Call(SubroutineCall{var: Some(id), name, exprs})
                        },
                        Some(Token::Symbol('(')) => {
                            let mut exprs = Vec::new();
                            sub_exprs(tokens, &mut exprs);
                            Term::Call(SubroutineCall{var: None, name: id, exprs})
                        },
                        _ => Term::VarName(id)
                    }
                }
            }
        },
        _ => tokens.parser_panic(),
    }
}

/// term (op term)*
fn expression(tokens: &mut PeekToken) -> Expression {
    let lhs = eat_term(tokens);

    let op = match tokens.peek() {
        Some(Token::Symbol('+')) => '+',
        Some(Token::Symbol('-')) => '-',
        Some(Token::Symbol('*')) => '*',
        Some(Token::Symbol('/')) => '/',
        Some(Token::Symbol('&')) => '&',
        Some(Token::Symbol('|')) => '|',
        Some(Token::Symbol('<')) => '<',
        Some(Token::Symbol('>')) => '>',
        Some(Token::Symbol('=')) => '=',
        _ => return Expression{lhs, op: None, rhs: None},
    };
    tokens.next();

    let rhs = expression(tokens);

    Expression{lhs, op: Some(op), rhs: Some(Box::new(rhs))}
}

/// 'let' varName ('[' expr ']')? '=' expr ';'
fn let_statement(tokens: &mut PeekToken) -> Statement {
    tokens.assert_keyword("let");
    let var_name = tokens.assert_ident();

    let index_expr = match tokens.next() {
        Some(Token::Symbol('[')) => {
            let expr = expression(tokens);
            tokens.assert_symbol(']');
            tokens.assert_symbol('=');
            Some(expr)
        },
        Some(Token::Symbol('=')) => {
            None
        },
        _ => tokens.parser_panic(),
    };

    let expr = expression(tokens);
    tokens.assert_symbol(';');
    
    Statement::Let(LetStatement{var_name, index_expr, expr})
}

/// 'if' '(' expr ')' '{' statements '}' ( 'else' '{' statements '}')?
fn if_statement(tokens: &mut PeekToken) -> Statement {
    tokens.assert_keyword("if");
    tokens.assert_symbol('(');
    let expr = expression(tokens);
    tokens.assert_symbol(')');

    tokens.assert_symbol('{');
    let mut if_stat = Vec::new();
    eat_statements(tokens, &mut if_stat);
    tokens.assert_symbol('}');

    match tokens.peek() {
        Some(Token::Ident(id)) => {
            if id == "else" {
                tokens.next();
                tokens.assert_symbol('{');
                let mut else_stat = Vec::new();
                eat_statements(tokens, &mut else_stat);
                tokens.assert_symbol('}');
                Statement::If(IfStatement{expr, if_stat, else_stat: Some(else_stat)})
            } else {
                Statement::If(IfStatement{expr, if_stat, else_stat: None})
            }
        },
        _ => {
            Statement::If(IfStatement{expr, if_stat, else_stat: None})
        }
    }
}

/// 'while' '(' expr ')' '{' statements '}'
fn while_statement(tokens: &mut PeekToken) -> Statement {
    tokens.assert_keyword("while");
    tokens.assert_symbol('(');
    let expr = expression(tokens);
    tokens.assert_symbol(')');

    tokens.assert_symbol('{');
    let mut statements = Vec::new();
    eat_statements(tokens, &mut statements);
    tokens.assert_symbol('}');
    
    Statement::While(expr, statements)
}

/// 'do' subCall ';'
fn do_statement(tokens: &mut PeekToken) -> Statement {
    tokens.assert_keyword("do");
    let sub_call = match eat_term(tokens) {
        Term::Call(sc) => sc,
        _ => tokens.parser_panic(),
    };
    tokens.assert_symbol(';');
    Statement::Do(sub_call)
}

fn return_statement(tokens: &mut PeekToken) -> Statement {
    tokens.assert_keyword("return");
    match tokens.peek() {
        Some(Token::Symbol(';')) => {
            tokens.next();
            Statement::Return(None)
        },
        _ => {
            let expr = expression(tokens);
            tokens.assert_symbol(';');
            Statement::Return(Some(expr))
        }
    }
}

fn eat_statements(tokens: &mut PeekToken, statements: &mut Vec<Statement>) {
    loop {
        let statement = match tokens.peek() {
            Some(Token::Ident(id)) => {
                match id.as_str() {
                    "let" => let_statement(tokens),
                    "if" => if_statement(tokens),
                    "while" => while_statement(tokens),
                    "do" => do_statement(tokens),
                    "return" => return_statement(tokens),
                    _ => break,
                }
            },
            _ => break,
        };
        statements.push(statement);
    }
}

fn class_subroutines(tokens: &mut PeekToken, subs: &mut Vec<SubroutineDec>) {
    loop {
        let sub_type = match tokens.peek() {
            Some(Token::Ident(id)) => {
                match id.as_str() {
                    "constructor" => SubroutineType::Constructor,
                    "function" => SubroutineType::Function,
                    "method" => SubroutineType::Method,
                    _ => break,
                }
            },
            _ => break
        };
        tokens.next();

        let id = tokens.assert_ident();
        let ret_type = if id == "void" {
            None
        } else {
            Some(id)
        };

        let name = tokens.assert_ident();
        tokens.assert_symbol('(');
        let mut params = Vec::new();
        sub_params(tokens, &mut params);
        tokens.assert_symbol(')');

        tokens.assert_symbol('{');
        let mut vars = Vec::new();
        sub_vars(tokens, &mut vars);
        let mut statements = Vec::new();
        eat_statements(tokens, &mut statements);
        tokens.assert_symbol('}');

        subs.push(SubroutineDec { sub_type, ret_type, name, params, vars, statements })
    }
}

pub fn parser<'a>(tokens: &'a mut Lexer<'a, Token>) -> JackClass {
    let mut tokens = PeekToken{ tokens, cur: None };
    tokens.assert_keyword("class");
    let name = tokens.assert_ident();
    tokens.assert_symbol('{');

    let mut vars = Vec::<ClassVarDec>::new();
    let mut subs = Vec::<SubroutineDec>::new();

    class_vars(&mut tokens, &mut vars);
    class_subroutines(&mut tokens, &mut subs);

    tokens.eat(Token::Symbol('}'));
    JackClass{ name, vars, subroutines: subs }
}
