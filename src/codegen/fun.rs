use crate::ast::{FunDef, ParamDecl};
use super::*;

pub trait FunctionGenerator {
    fn generate_functions(&self, functions: Vec<FunDef>) -> String;
    fn convert_function(&self, function: FunDef) -> String;
    fn convert_params(&self, params: Vec<ParamDecl>) -> String;

}

impl FunctionGenerator for CodeGenerator {
    fn generate_functions(&self, functions: Vec<FunDef>) -> String {
        let fun_collection: Vec<String> = functions.iter().map(|f| self.convert_function(f.clone())).collect();
        fun_collection.join("; \n").trim().to_string()
    }
    
    fn convert_function(&self, function: FunDef) -> String {
        let name = function.name;
        let params = self.convert_params(function.params);
        let statements = self.convert_statement(function.statements.unwrap());

        ["function ".to_string(), name, "(".to_string(), params, ")".to_string(),statements,"\n".to_string()].join("")
    }

    fn convert_params(&self, params: Vec<ParamDecl>) -> String {
        let param_collection: Vec<_> = params.iter().map(|n| n.clone().name).collect();
        param_collection.join(",")
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::*, parser::*, codegen::*};

    fn gen_fun(input: &str) -> String {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        // println!("{:?}",ast);
        let generator = CodeGenerator::new(ast);
        let funs = generator.generate_functions(generator.functions.clone());
        println!("{}",funs);
        funs
    }

    #[test]
    fn test_generate_minimal_function() {
        let funs = gen_fun("fun functionName() -> Void {}");
        assert_eq!(funs, "function functionName(){  }")
    }

    #[test]
    fn test_generate_minimal_function_with_params() {
        let funs = gen_fun("fun functionName(intParam: Int, stringParam: Str, boolParam: Boolean) -> Void {}");
        assert_eq!(funs, "function functionName(intParam,stringParam,boolParam){  }")
    }

    #[test]
    fn test_generate_function() {
        let funs = gen_fun("fun square(x: Int) -> Int {let square: Int = x*x; return square;}");
        assert_eq!(funs, "function square(x){ let square = x * x; \nreturn square }")
    }

    #[test]
    fn test_generate_multiple_functions() {
        let funs = gen_fun("fun square(x: Int) -> Int {let square: Int = x*x; return square;}
                                    fun bark() -> Void {println(\"bark\");}");
        assert_eq!(funs, "function square(x){ let square = x * x; \nreturn square }\n; \nfunction bark(){ console.log(\"bark\") }")
    }
}