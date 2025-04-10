use crate::ast::*;
use colored::*;
use std::fmt;

pub trait PrettyPrint {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result;

    fn print(&self)
    where
        Self: fmt::Display,
    {
        println!("{}", self);
    }

    fn to_pretty_string(&self) -> String
    where
        Self: fmt::Display,
    {
        format!("{}", self)
    }
}

fn indent_str(indent: usize) -> String {
    "  ".repeat(indent)
}

fn print_list<T: PrettyPrint>(
    f: &mut fmt::Formatter<'_>,
    items: &[T],
    indent: usize,
    separator: &str,
) -> fmt::Result {
    if items.is_empty() {
        return Ok(());
    }

    for (i, item) in items.iter().enumerate() {
        item.pretty_print(f, indent)?;
        if i < items.len() - 1 {
            write!(f, "{}", separator)?;
        }
    }
    Ok(())
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl fmt::Display for FunDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl fmt::Display for ClassDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl fmt::Display for MethDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl PrettyPrint for Expr {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        match self {
            Expr::IntegerLiteral(lit) => write!(f, "{}", lit.value.to_string().cyan()),
            Expr::StringLiteral(lit) => write!(f, "\"{}\"", lit.value.green()),
            Expr::BooleanLiteral(lit) => write!(f, "{}", lit.value.to_string().yellow()),
            Expr::Variable(var) => write!(f, "{}", var.name.cyan().italic()),
            Expr::Binary(bin_expr) => {
                write!(f, "(")?;
                bin_expr.left.pretty_print(f, indent)?;
                write!(f, " {} ", bin_expr.operator.to_string().magenta())?;
                bin_expr.right.pretty_print(f, indent)?;
                write!(f, ")")
            }
            Expr::FunCall(call) => {
                write!(f, "{}(", call.callee.blue().bold())?;
                for (i, arg) in call.args.iter().enumerate() {
                    arg.pretty_print(f, indent)?;
                    if i < call.args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Expr::MethCall(call) => {
                call.object.pretty_print(f, indent)?;
                write!(f, ".{}(", call.meth.blue().bold())?;
                for (i, arg) in call.args.iter().enumerate() {
                    arg.pretty_print(f, indent)?;
                    if i < call.args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Expr::New(new_expr) => {
                write!(
                    f,
                    "{} {}(",
                    "new".magenta(),
                    new_expr.class_name.blue().bold()
                )?;
                for (i, arg) in new_expr.args.iter().enumerate() {
                    arg.pretty_print(f, indent)?;
                    if i < new_expr.args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Expr::This(_) => write!(f, "{}", "this".yellow().bold()),
            Expr::Println(expr) => {
                write!(f, "{}(", "println".blue().bold())?;
                expr.arg.pretty_print(f, indent)?;
                write!(f, ")")
            }
            Expr::Print(expr) => {
                write!(f, "{}(", "print".blue().bold())?;
                expr.arg.pretty_print(f, indent)?;
                write!(f, ")")
            }
            Expr::Grouped(expr, _) => {
                write!(f, "(")?;
                expr.pretty_print(f, indent)?;
                write!(f, ")")
            }
            Expr::Empty => write!(f, "<empty>"),
        }
    }
}

impl PrettyPrint for Stmt {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let ind = indent_str(indent);
        match self {
            Stmt::Expr(expr_stmt) => {
                write!(f, "{}", ind)?;
                expr_stmt.expr.pretty_print(f, indent)?;
                writeln!(f, ";")
            }
            Stmt::VarDecl(var_decl) => {
                writeln!(
                    f,
                    "{}{} {}: {};",
                    ind,
                    "let".magenta(),
                    var_decl.name.cyan(),
                    var_decl.var_type.to_string().blue().bold()
                )
            }
            Stmt::Assign(assign) => {
                write!(f, "{}{} = ", ind, assign.name.cyan())?;
                assign.expr.pretty_print(f, indent)?;
                writeln!(f, ";")
            }
            Stmt::VarDeclWithAssign(var_decl) => {
                write!(
                    f,
                    "{}{} {}: {} = ",
                    ind,
                    "let".magenta(),
                    var_decl.name.cyan(),
                    var_decl.var_type.to_string().blue().bold()
                )?;
                var_decl.expr.pretty_print(f, indent)?;
                writeln!(f, ";")
            }
            Stmt::While(while_stmt) => {
                write!(f, "{}{} (", ind, "while".magenta().bold())?;
                while_stmt.condition.pretty_print(f, indent)?;
                writeln!(f, ") {{")?;
                while_stmt.body.pretty_print(f, indent + 1)?;
                writeln!(f, "{}}}", ind)
            }
            Stmt::If(if_stmt) => {
                write!(f, "{}{} (", ind, "if".magenta().bold())?;
                if_stmt.condition.pretty_print(f, indent)?;
                writeln!(f, ") {{")?;
                if_stmt.then_branch.pretty_print(f, indent + 1)?;
                writeln!(f, "{}}}", ind)?;

                if let Some(else_branch) = &if_stmt.else_branch {
                    writeln!(f, "{}{} {{", ind, "else".magenta().bold())?;
                    else_branch.pretty_print(f, indent + 1)?;
                    writeln!(f, "{}}}", ind)
                } else {
                    Ok(())
                }
            }
            Stmt::Break(_) => writeln!(f, "{}{};", ind, "break".red().bold()),
            Stmt::Return(ret) => {
                write!(f, "{}{}", ind, "return".red().bold())?;
                if let Some(value) = &ret.value {
                    write!(f, " ")?;
                    value.pretty_print(f, indent)?;
                }
                writeln!(f, ";")
            }
            Stmt::Block(block) => {
                for stmt in &block.statements {
                    stmt.pretty_print(f, indent)?;
                }
                Ok(())
            }
            Stmt::Empty => writeln!(f, "{}<empty statement>", ind),
        }
    }
}

impl PrettyPrint for ParamDecl {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, _indent: usize) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.name.cyan(),
            self.param_type.to_string().blue().bold()
        )
    }
}

impl PrettyPrint for FunDef {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let ind = indent_str(indent);

        write!(
            f,
            "{}{} {}(",
            ind,
            "fun".magenta().bold(),
            self.name.green().bold()
        )?;

        for (i, param) in self.params.iter().enumerate() {
            param.pretty_print(f, indent)?;
            if i < self.params.len() - 1 {
                write!(f, ", ")?;
            }
        }

        writeln!(f, ") -> {} {{", self.return_type.to_string().blue().bold())?;

        for stmt in &self.statements {
            stmt.pretty_print(f, indent + 1)?;
        }

        writeln!(f, "{}}}", ind)
    }
}

impl PrettyPrint for Constructor {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let ind = indent_str(indent);

        write!(f, "{}{} (", ind, "init".magenta().bold())?;

        for (i, param) in self.params.iter().enumerate() {
            param.pretty_print(f, indent)?;
            if i < self.params.len() - 1 {
                write!(f, ", ")?;
            }
        }

        writeln!(f, ") {{")?;

        if let Some(args) = &self.super_call {
            write!(f, "{}  {}(", ind, "super".yellow().bold())?;
            for (i, arg) in args.iter().enumerate() {
                arg.pretty_print(f, indent + 1)?;
                if i < args.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            writeln!(f, ");")?;
        }

        for stmt in &self.statements {
            stmt.pretty_print(f, indent + 1)?;
        }

        writeln!(f, "{}}}", ind)
    }
}

impl PrettyPrint for MethDef {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let ind = indent_str(indent);

        write!(
            f,
            "{}{} {}(",
            ind,
            "meth".magenta().bold(),
            self.name.green().bold()
        )?;

        for (i, param) in self.params.iter().enumerate() {
            param.pretty_print(f, indent)?;
            if i < self.params.len() - 1 {
                write!(f, ", ")?;
            }
        }

        writeln!(f, ") -> {} {{", self.return_type.to_string().blue().bold())?;

        for stmt in &self.statements {
            stmt.pretty_print(f, indent + 1)?;
        }

        writeln!(f, "{}}}", ind)
    }
}

impl PrettyPrint for ClassDef {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let ind = indent_str(indent);

        write!(
            f,
            "{}{} {}",
            ind,
            "class".magenta().bold(),
            self.name.green().bold()
        )?;

        if let Some(parent) = &self.extends {
            write!(f, " {} {}", "extends".magenta(), parent.green().bold())?;
        }

        writeln!(f, " {{")?;

        for var in &self.vars {
            var.pretty_print(f, indent + 1)?;
        }

        if !self.vars.is_empty() {
            writeln!(f)?;
        }

        self.constructor.pretty_print(f, indent + 1)?;

        for method in &self.methods {
            writeln!(f)?; // spacing between methods
            method.pretty_print(f, indent + 1)?;
        }

        writeln!(f, "{}}}", ind)
    }
}

impl PrettyPrint for Entry {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        for class_def in &self.class_defs {
            class_def.pretty_print(f, indent)?;
            writeln!(f)?; // spacing between classes
        }

        for fun_def in &self.fun_defs {
            fun_def.pretty_print(f, indent)?;
            writeln!(f)?; // spacing between functions
        }

        for stmt in &self.statements {
            stmt.pretty_print(f, indent)?;
        }

        Ok(())
    }
}

impl PrettyPrint for VarDeclStmt {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let ind = indent_str(indent);
        writeln!(
            f,
            "{}{} {}: {};",
            ind,
            "let".magenta(),
            self.name.cyan(),
            self.var_type.to_string().blue().bold()
        )
    }
}
