use crate::ast::{Statement, Expression, Op, Spanned, Span};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Str,
    Bool,
    Void,
    Struct(String),
    Enum(String),
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Variable(usize), // For HM inference
}

pub struct TypeChecker {
    next_var: usize,
    substitutions: HashMap<usize, Type>,
    env: HashMap<String, Type>,
    structs: HashMap<String, HashMap<String, Type>>,
    enums: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub struct TypeError {
    pub message: String,
    pub span: Span,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut env = HashMap::new();
        // Built-in functions or globals could be added here
        env.insert("log".to_string(), Type::Function { 
            params: vec![Type::Int], // Simplified for now
            return_type: Box::new(Type::Void) 
        });

        Self {
            next_var: 0,
            substitutions: HashMap::new(),
            env,
            structs: HashMap::new(),
            enums: HashMap::new(),
        }
    }

    fn new_var(&mut self) -> Type {
        let res = Type::Variable(self.next_var);
        self.next_var += 1;
        res
    }

    fn find(&self, var: usize) -> Type {
        if let Some(t) = self.substitutions.get(&var) {
            if let Type::Variable(next_var) = t {
                self.find(*next_var)
            } else {
                t.clone()
            }
        } else {
            Type::Variable(var)
        }
    }

    fn unify(&mut self, t1: &Type, t2: &Type, span: Span) -> Result<(), TypeError> {
        let t1 = match t1 {
            Type::Variable(v) => self.find(*v),
            _ => t1.clone(),
        };
        let t2 = match t2 {
            Type::Variable(v) => self.find(*v),
            _ => t2.clone(),
        };

        if t1 == t2 {
            return Ok(());
        }

        match (t1, t2) {
            (Type::Variable(v), t) | (t, Type::Variable(v)) => {
                self.substitutions.insert(v, t);
                Ok(())
            }
            (Type::Function { params: p1, return_type: r1 }, Type::Function { params: p2, return_type: r2 }) => {
                if p1.len() != p2.len() {
                    return Err(TypeError { message: "Function arity mismatch".to_string(), span });
                }
                for (a, b) in p1.iter().zip(p2.iter()) {
                    self.unify(a, b, span.clone())?;
                }
                self.unify(&*r1, &*r2, span)
            }
            (a, b) => Err(TypeError {
                message: format!("Type mismatch: {:?} and {:?}", a, b),
                span,
            }),
        }
    }

    pub fn check_program(&mut self, program: &[Spanned<Statement>]) -> Result<(), TypeError> {
        // First pass: collect definitions (Structs, Enums, Functions)
        for stmt in program {
            match &stmt.node {
                Statement::StructDef { name, fields } => {
                    let mut field_map = HashMap::new();
                    for (f_name, f_type) in fields {
                        field_map.insert(f_name.clone(), self.parse_type(f_type));
                    }
                    self.structs.insert(name.clone(), field_map);
                }
                Statement::EnumDef { name, variants } => {
                    self.enums.insert(name.clone(), variants.clone());
                }
                Statement::FunctionDef { name, params, body: _ } => {
                    let param_types = params.iter().map(|(_, t)| self.parse_type(t)).collect();
                    // Assume main returns Int, others Void for now unless specified
                    let ret_type = if name == "main" { Type::Int } else { Type::Void };
                    self.env.insert(name.clone(), Type::Function {
                        params: param_types,
                        return_type: Box::new(ret_type),
                    });
                }
                _ => {}
            }
        }

        // Second pass: check function bodies
        for stmt in program {
            if let Statement::FunctionDef { name, params, body } = &stmt.node {
                let mut local_env = self.env.clone();
                for (p_name, p_type) in params {
                    local_env.insert(p_name.clone(), self.parse_type(p_type));
                }
                
                let expected_ret = if name == "main" { Type::Int } else { Type::Void };
                self.check_block(body, &mut local_env, &expected_ret)?;
            }
        }

        Ok(())
    }

    fn check_block(&mut self, block: &[Spanned<Statement>], env: &mut HashMap<String, Type>, expected_ret: &Type) -> Result<(), TypeError> {
        for stmt in block {
            self.check_statement(stmt, env, expected_ret)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Spanned<Statement>, env: &mut HashMap<String, Type>, expected_ret: &Type) -> Result<(), TypeError> {
        match &stmt.node {
            Statement::Let { name, value } => {
                let val_type = self.infer_expression(value, env)?;
                env.insert(name.clone(), val_type);
            }
            Statement::Set { name, value } => {
                let val_type = self.infer_expression(value, env)?;
                if let Some(expected) = env.get(name) {
                    self.unify(&val_type, expected, value.span.clone())?;
                } else {
                    return Err(TypeError { message: format!("Undefined variable {}", name), span: stmt.span.clone() });
                }
            }
            Statement::Return(expr) => {
                let val_type = self.infer_expression(expr, env)?;
                self.unify(&val_type, expected_ret, expr.span.clone())?;
            }
            Statement::If { cond, then_block, else_block } => {
                let cond_type = self.infer_expression(cond, env)?;
                self.unify(&cond_type, &Type::Bool, cond.span.clone())?;
                self.check_block(then_block, &mut env.clone(), expected_ret)?;
                if let Some(eb) = else_block {
                    self.check_block(eb, &mut env.clone(), expected_ret)?;
                }
            }
            Statement::While { cond, body } => {
                let cond_type = self.infer_expression(cond, env)?;
                self.unify(&cond_type, &Type::Bool, cond.span.clone())?;
                self.check_block(body, &mut env.clone(), expected_ret)?;
            }
            Statement::Expression(expr) => {
                self.infer_expression(expr, env)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn infer_expression(&mut self, expr: &Spanned<Expression>, env: &HashMap<String, Type>) -> Result<Type, TypeError> {
        match &expr.node {
            Expression::Number(_) => Ok(Type::Int),
            Expression::StringLit(_) => Ok(Type::Str),
            Expression::Bool(_) => Ok(Type::Bool),
            Expression::Null => Ok(self.new_var()),
            Expression::Identifier(name) => {
                if let Some(t) = env.get(name) {
                    Ok(t.clone())
                } else {
                    Err(TypeError { message: format!("Undefined identifier {}", name), span: expr.span.clone() })
                }
            }
            Expression::BinaryOp(left, op, right) => {
                let lt = self.infer_expression(left, env)?;
                let rt = self.infer_expression(right, env)?;
                match op {
                    Op::Plus | Op::Minus | Op::Mul | Op::Div => {
                        self.unify(&lt, &Type::Int, left.span.clone())?;
                        self.unify(&rt, &Type::Int, right.span.clone())?;
                        Ok(Type::Int)
                    }
                    Op::Eq | Op::NotEq | Op::Gt | Op::Lt | Op::Gte | Op::Lte => {
                        self.unify(&lt, &rt, expr.span.clone())?;
                        Ok(Type::Bool)
                    }
                }
            }
            Expression::Call { func, args } => {
                let ft = self.infer_expression(func, env)?;
                let mut arg_types = Vec::new();
                for arg in args {
                    arg_types.push(self.infer_expression(arg, env)?);
                }
                let ret_var = self.new_var();
                let call_type = Type::Function {
                    params: arg_types,
                    return_type: Box::new(ret_var.clone()),
                };
                self.unify(&ft, &call_type, expr.span.clone())?;
                Ok(ret_var)
            }
            Expression::MemberAccess { object, member } => {
                let ot = self.infer_expression(object, env)?;
                match ot {
                    Type::Struct(struct_name) => {
                        if let Some(fields) = self.structs.get(&struct_name) {
                            if let Some(t) = fields.get(member) {
                                Ok(t.clone())
                            } else {
                                Err(TypeError { message: format!("Struct {} has no field {}", struct_name, member), span: expr.span.clone() })
                            }
                        } else {
                            Err(TypeError { message: format!("Undefined struct {}", struct_name), span: expr.span.clone() })
                        }
                    }
                    _ => Ok(self.new_var()), // Could be a module access or native object
                }
            }
            _ => Ok(self.new_var()),
        }
    }

    fn parse_type(&self, t: &str) -> Type {
        match t {
            "int" => Type::Int,
            "str" => Type::Str,
            "bool" => Type::Bool,
            "void" => Type::Void,
            _ => {
                if self.structs.contains_key(t) {
                    Type::Struct(t.to_string())
                } else if self.enums.contains_key(t) {
                    Type::Enum(t.to_string())
                } else {
                    Type::Int // Default
                }
            }
        }
    }
}
