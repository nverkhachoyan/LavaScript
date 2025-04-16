use crate::ast::Stmt;
use super::*;

pub trait StatementGenerator {
    fn generate_statements(&self, statements:Vec<Stmt>) -> String;
    fn convert_statement(&self, statement: Stmt) -> String;
}

impl StatementGenerator for CodeGenerator {
    fn generate_statements(&self, statements: Vec<Stmt>) -> String {
        let stmt_collection: Vec<_> = statements.iter().map(|s| self.convert_statement(s.clone())).collect();
        stmt_collection.join("\n")
    }
    fn convert_statement(&self, statement: Stmt) -> String {
         let stmt = match statement {
            Stmt::Expr(expr_stmt) => self.convert_expression(*expr_stmt.expr),
            Stmt::VarDecl(var_decl_stmt) => ["let".to_string(), var_decl_stmt.name].join(" "),
            Stmt::Assign(assign_stmt) => [assign_stmt.name, "=".to_string(), self.convert_expression(*assign_stmt.expr)].join(" "),
            Stmt::VarDeclWithAssign(var_decl_with_assign) => 
                ["let".to_string(), var_decl_with_assign.name, "=".to_string(), self.convert_expression(*var_decl_with_assign.expr)].join(" "),
            Stmt::While(while_stmt) => {
                let condition = self.convert_expression(*while_stmt.condition);
                let body = self.convert_statement(*while_stmt.body);
                ["while (".to_string(), condition, ")".to_string(), body].join("")
            }
            Stmt::If(if_stmt) => {
                let condition = self.convert_expression(*if_stmt.condition);
                let then = self.convert_statement(*if_stmt.then_branch);
                let els = match if_stmt.else_branch {
                    Some(stmt) => self.convert_statement(*stmt),
                    None => "".to_string(),
                };
                ["if (".to_string(), condition, ")".to_string(), then, els ].join("")
            }
            Stmt::Break(_break_stmt) => "break".to_string(),
            Stmt::Return(return_stmt) => {
                match return_stmt.value {
                    Some(expr) => ["return".to_string(), self.convert_expression(*expr)].join(" "),
                    None => "return".to_string(),
                }
            },
            Stmt::Block(block_stmt) => {
                ["{".to_string(), self.generate_statements(block_stmt.statements), "}".to_string()].join(" ")
            }
            Stmt::Empty => "".to_string(),
        };
        [stmt,";".to_string()].join("")
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::*, parser::*, codegen::*};

    fn gen_stmt(input: &str) -> String {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        println!("{:?}",ast);
        let generator = CodeGenerator::new(ast);
        let stmt = generator.generate_statements(generator.statements.clone());
        stmt
    }

    #[test]
    fn test_generate_empty_statement() {
        let stmt = gen_stmt("");
        assert_eq!(stmt, "".to_string())
    }

    #[test]
    fn test_generate_var_decl() {
        let stmt = gen_stmt("let x: Int;");
        assert_eq!(stmt, "let x".to_string())
    }

    #[test]
    fn test_generate_var_assignment() {
        let stmt = gen_stmt("x = 5;");
        assert_eq!(stmt, "x = 5")
    }

    #[test]
    fn generate_var_decl_w_assignment() {
        let stmt = gen_stmt("let x: Int = 5;");
        assert_eq!(stmt, "let x = 5")
    }

    #[test]
    fn generate_while_loop() {
        let stmt = gen_stmt("while (i < 5) {i = i + 1;}");
        println!("{:?}", stmt);
    }
}