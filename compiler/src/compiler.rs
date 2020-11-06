use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::values::{FunctionValue, PointerValue};

use parser::parse_tree::{SourceUnit, SourceUnitPart};
use parser::parser::parse_program;

#[allow(dead_code)]
pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub module: &'a Module<'ctx>,
    pub source_unit: &'a SourceUnit,

    variables: HashMap<String, PointerValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Compiles the specified `Function` in the given `Context` and using the specified `Builder`, `PassManager`, and `Module`.
    pub fn compile(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        pass_manager: &'a PassManager<FunctionValue<'ctx>>,
        module: &'a Module<'ctx>,
        source_unit: &SourceUnit,
    ) {
        let mut compiler = Compiler {
            context: context,
            builder: builder,
            fpm: pass_manager,
            module,
            source_unit,
            fn_value_opt: None,
            variables: HashMap::new(),
        };

        compiler.compile_fn();
    }

    fn compile_fn(&mut self) {
        for part in self.source_unit.0.iter() {
            match part {
                SourceUnitPart::ImportDirective(_) => {}
                SourceUnitPart::MultipleImportDirective(_) => {}
                SourceUnitPart::PackageDirective(_) => {}
                SourceUnitPart::StructFuncDef(fun) => {
                    if !fun.body.is_empty() {
                        println!("{:?}", fun.body);
                    }
                }
                SourceUnitPart::FuncDef(_) => {}
                SourceUnitPart::StructDef(_) => {}
            }
        }
    }
}

pub fn create_compiler(input: &str) {
    let context = Context::create();
    let module = context.create_module("repl");
    let builder = context.create_builder();

    let fpm = PassManager::create(&module);
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();

    let parse_ast = parse_program(input);
    match parse_ast {
        Ok(unit) => {
            Compiler::compile(&context, &builder, &fpm, &module, &unit);
        }
        Err(_) => {}
    }
}

#[cfg(test)]
mod test {
    use crate::compiler::create_compiler;

    #[test]
    #[rustfmt::skip]
    fn init_parser() {
        create_compiler("default$main(string name) {fmt.println(\"hello,world\")}");
    }
}