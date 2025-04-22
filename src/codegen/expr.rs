use crate::ast::Expr;
use super::*;

pub trait ExpressionGenerator {
    fn generate_expressions(&self, expressions: Vec<Expr>, seperator: &str) -> String;
    fn convert_expression(&self, expression: Expr) -> String;
}

impl ExpressionGenerator for CodeGenerator {
    fn generate_expressions(&self, expressions: Vec<Expr>, seperator: &str) -> String {
        let expression_collection: Vec<_> = expressions.iter().map(|e| self.convert_expression(e.clone())).collect();
        expression_collection.join(seperator)
    }

    fn convert_expression(&self, expression: Expr) -> String {
        match expression {
            Expr::IntegerLiteral(integer_literal) => integer_literal.value.to_string(),
            Expr::StringLiteral(string_literal) => ["\"".to_string(), string_literal.value, "\"".to_string()].join(""),
            Expr::BooleanLiteral(boolean_literal) => boolean_literal.value.to_string(),
            Expr::Variable(variable) => variable.name,
            Expr::Binary(binary_expr) => {
                let left = self.convert_expression(*binary_expr.left);
                let operation = match binary_expr.operator {
                    crate::ast::BinaryOp::Add => "+".to_string(),
                    crate::ast::BinaryOp::Subtract => "-".to_string(),
                    crate::ast::BinaryOp::Multiply => "*".to_string(),
                    crate::ast::BinaryOp::Divide => "/".to_string(),
                    crate::ast::BinaryOp::Equal => "==".to_string(),
                    crate::ast::BinaryOp::NotEqual => "!=".to_string(),
                    crate::ast::BinaryOp::Greater => ">".to_string(),
                    crate::ast::BinaryOp::Less => "<".to_string(),
                    crate::ast::BinaryOp::GreaterEqual => ">=".to_string(),
                    crate::ast::BinaryOp::LessEqual => "<=".to_string(),
                    crate::ast::BinaryOp::Or => "||".to_string(),
                    crate::ast::BinaryOp::And => "&&".to_string(),
                };
                let right = self.convert_expression(*binary_expr.right);
                [left, operation,right].join(" ")

            }
            Expr::Unary(unary_expr) => {
                let operation = match unary_expr.operator {
                    crate::ast::UnaryOp::Not => "!".to_string(),
                    crate::ast::UnaryOp::Negate => "-".to_string(),
                    crate::ast::UnaryOp::Plus => "+".to_string(),
                };
                let value = self.convert_expression(*unary_expr.expr);
                [operation, value].join("")
            },
            Expr::FunCall(fun_call) => {
                let function = fun_call.callee;
                let args = self.generate_expressions(fun_call.args, ",");
                [function, "(".to_string(), args, ")".to_string()].join("") 
            }
            Expr::MethCall(meth_call) => {
                let object = self.convert_expression(*meth_call.object);
                let method = meth_call.meth;
                let args = self.generate_expressions(meth_call.args, ",");
                [object,".".to_string(),method,"(".to_string(),args,")".to_string()].join("")
            }
            Expr::Field(field_call) => {
                let object = self.convert_expression(*field_call.object);
                let field = field_call.field;
                [object,".".to_string(),field.to_string()].join("")
            }
            Expr::New(new_expr) => {
                let name = new_expr.class_name;
                let args = self.generate_expressions(new_expr.args, ",");
                ["new".to_string(), [name,"(".to_string(),args,")".to_string()].join("")].join(" ")
            },
            Expr::This(_) => "this".to_string(),
            Expr::Println(println_expr) => ["console.log(".to_string(),self.convert_expression(*println_expr.arg),")".to_string()].join(""),
            Expr::Print(print_expr) => ["console.log(".to_string(),self.convert_expression(*print_expr.arg),")".to_string()].join(""),
            Expr::Grouped(expr, _span) => self.convert_expression(*expr),
            Expr::Empty => "".to_string(),
        }
    }
}