mod core;
mod stmt;
mod class;
mod fun;

pub use core::CodeGenerator;
use stmt::StatementGenerator;
use class::ClassGenerator;
use fun::FunctionGenerator;