use crate::ast::{ClassDef, Constructor, MethDef, VarDeclStmt};

use super::*;

pub trait ClassGenerator {
    fn generate_methods(&self, methods: Vec<MethDef>) -> String;
    fn generate_classes(&self, classes: Vec<ClassDef>) -> String;
    fn convert_class(&self, class: ClassDef) -> String;
    fn convert_fields(&self, vars: Vec<VarDeclStmt>) -> String;
    fn convert_constructor(&self, constructor: Constructor) -> String;
    fn convert_method(&self, method: MethDef) -> String;
}

impl ClassGenerator for CodeGenerator {
    fn generate_classes(&self, classes: Vec<ClassDef>) -> String {
        let class_collection: Vec<_> = classes.iter().map(|c| self.convert_class(c.clone())).collect();
        class_collection.join("\n")
    }
    
    fn convert_class(&self, class: ClassDef) -> String {
        let name = class.name;
        let extends = match class.extends {
            Some(string) => [" extends".to_string(),string].join(" "),
            None => "".to_string(),
        };
        let fields = self.convert_fields(class.vars);
        let constructor = self.convert_constructor(class.constructor);
        let methods = self.generate_methods(class.methods);
        

        ["class ".to_string(), name, extends, "{\n".to_string(), fields, constructor, methods, "\n}".to_string()].join("")
    }

    fn convert_fields(&self, vars: Vec<VarDeclStmt>) -> String {
        let field_collection: Vec<_> = vars.iter().map(|n| n.clone().name).collect();
        field_collection.join(";\n")
    }

    fn convert_constructor(&self, constructor: Constructor) -> String{
        let params = self.convert_params(constructor.params);
        let super_call = match &constructor.super_call {
            Some(_) => ["super(".to_string(), self.generate_expressions(constructor.super_call.unwrap(), ","),")".to_string()].join(""),
            None => "".to_string()
        };
        let statements = match &constructor.statements {
            Some(_) => self.convert_statement(constructor.statements.unwrap()),
            None => "".to_string()
        };
        
        ["constructor(".to_string(), params, ") {".to_string(), super_call, statements,"}\n".to_string()].join("")
    }
    
    fn generate_methods(&self, methods: Vec<MethDef>) -> String {
        let method_collection: Vec<String> = methods.iter().map(|m| self.convert_method(m.clone())).collect();
        method_collection.join("").trim().to_string()
    }
    
    fn convert_method(&self, method: MethDef) -> String {
        let name = method.name;
        let params = self.convert_params(method.params);
        let statements = match method.statements {
            Some(_) =>self.convert_statement(method.statements.unwrap()),
            None => "".to_string()
        };
        [name, "(".to_string(), params, ")".to_string(),statements,"\n".to_string()].join("")
    }
    
}

#[cfg(test)]
mod tests {
    use crate::{lexer::*, parser::*, codegen::*};

    fn gen_class(input: &str) -> String {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        println!("{:?}",ast);
        let generator = CodeGenerator::new(ast);
        let classes = generator.generate_classes(generator.classes.clone());
        println!("{}",classes);
        classes
    }

    #[test]
    fn test_generate_minimal_class() {
        let classes = gen_class("class Animal { init() {} }");
        assert_eq!(classes, "class Animal{\nconstructor() {}\n\n}")
    }

    #[test]
    fn test_generate_class_with_field() {
        let class = gen_class("class Animal { init(voice: Str) {{this.voice = voice;}} }");
        assert_eq!(class, "class Animal{\nconstructor(voice) {{ { this.voice = voice } }}\n\n}")
    }

    #[test]
    fn test_generate_class_with_fields() {
        let class = gen_class("class Animal { init(voice: Str, limbnum: Int) {{this.voice = voice; this.limbnum = limbnum}} }");
        assert_eq!(class, "class Animal{\nconstructor(voice,limbnum) {{ { this.voice = voice; \nthis.limbnum = limbnum } }}\n\n}")
    }

    #[test]
    fn test_generate_class_with_methods() {
        let class = gen_class("class Animal { init() {} 
        meth speak() -> Void { println(\"animal noise\"); }
        meth age() -> Int {return 0;}}");
        assert_eq!(class, "class Animal{\nconstructor() {}\nspeak(){ console.log(\"animal noise\") }\nage(){ return 0 }\n}".trim())
    }

    #[test]
    fn test_generate_inherited_class() {
        let class = gen_class("class Cat extends Animal { init() {super(\"meow\");} }");
        assert_eq!(class, "class Cat extends Animal{\nconstructor() {super(\"meow\")}\n\n}")
    }
}