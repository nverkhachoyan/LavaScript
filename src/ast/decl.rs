use crate::ast::*;

#[derive(Debug, Clone, Default)]
pub struct ParamDecl {
    pub name: String,
    pub param_type: TypeName,
}

#[derive(Debug, Clone, Default)]
pub struct FunDef {
    pub name: String,
    pub params: Vec<ParamDecl>,
    pub return_type: TypeName,
    pub statements: Option<Stmt>,
}

#[derive(Debug, Clone, Default)]
pub struct Constructor {
    pub params: Vec<ParamDecl>,
    pub super_call: Option<Vec<Expr>>,
    pub statements: Option<Stmt>,
}

#[derive(Debug, Clone, Default)]
pub struct MethDef {
    pub name: String,
    pub params: Vec<ParamDecl>,
    pub return_type: TypeName,
    pub statements: Option<Stmt>,
}

#[derive(Debug, Clone, Default)]
pub struct ClassDef {
    pub name: String,
    pub extends: Option<String>,
    pub vars: Vec<VarDeclStmt>,
    pub constructor: Constructor,
    pub methods: Vec<MethDef>,
}

#[derive(Debug, Default, Clone)]
pub struct Entry {
    pub statements: Vec<Stmt>,
    pub class_defs: Vec<ClassDef>,
    pub fun_defs: Vec<FunDef>,
}
