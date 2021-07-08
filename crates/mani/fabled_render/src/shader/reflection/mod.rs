mod shader_reflection;
pub use shader_reflection::*;

// mostly all or all the data are UnAligned and invoking reflection on a shader is time consuming,
// so use reflection as scarcely as possible and cache the results.

#[derive(Debug)]
pub struct FunctionArgument<'a> {
    pub name: Option<String>,
    pub ty: Option<&'a naga::Type>,
    pub binding: Option<naga::Binding>,
}

#[derive(Debug)]
pub struct FunctionResult<'a> {
    pub ty: Option<&'a naga::Type>,
    pub binding: Option<naga::Binding>,
}

#[derive(Debug)]
pub struct LocalVariables<'a> {
    pub name: Option<String>,
    pub ty: Option<&'a naga::Type>,
    pub init: Option<&'a naga::Constant>,
}

#[derive(Debug)]
pub struct NamedExpressions<'a> {
    pub name: Option<String>,
    pub expr: Vec<Option<&'a naga::Expression>>,
}

#[derive(Default)]
pub struct ShaderRefFunc<'a> {
    pub name: Option<String>,
    pub args: Vec<FunctionArgument<'a>>,
    pub result: Option<FunctionResult<'a>>,
    pub local_var: Vec<LocalVariables<'a>>,
    pub expr: Vec<naga::Expression>,
    pub named_expr: Vec<NamedExpressions<'a>>,
    pub block: Vec<naga::Statement>,
}

pub struct ShaderRefEntry<'a> {
    pub name: String,
    pub stage: naga::ShaderStage,
    pub early_depth_test: Option<naga::EarlyDepthTest>,
    pub workgroup: [u32; 3],
    pub entry_function: ShaderRefFunc<'a>,
}

impl<'a> Default for ShaderRefEntry<'a> {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            stage: naga::ShaderStage::Vertex,
            early_depth_test: None,
            workgroup: [0, 0, 0],
            entry_function: Default::default(),
        }
    }
}

#[cfg(test)]
mod data_alignment_test {
    use crate::shader::reflection::*;

    #[test]
    fn data_alignment() {
        let func_arg = std::mem::size_of::<FunctionArgument>();
        println!("FunctionArgument is {} bytes", func_arg);

        let func_result = std::mem::size_of::<FunctionResult>();
        println!("FunctionResult is {} bytes", func_result);

        let local_var = std::mem::size_of::<LocalVariables>();
        println!("Local Variable is {} bytes", local_var);

        let named_expr = std::mem::size_of::<NamedExpressions>();
        println!("Named Expressions is {} bytes", named_expr);

        let ref_func = std::mem::size_of::<ShaderRefFunc>();
        println!("ShaderRefFunc is {} bytes", ref_func);

        let ref_entry = std::mem::size_of::<ShaderRefEntry>();
        println!("ShaderRefEntry is {} bytes", ref_entry);
    }
}

#[cfg(test)]
mod reflection_test {
    use crate::init_shader_test_env;
    use crate::shader::parser::*;
    use crate::shader::reflection::*;
    use naga::{Constant, Function, GlobalVariable, Type};

    #[test]
    fn reflect_entry() {
        init_shader_test_env();

        println!("-----------------------------------------------");

        let wgsl_parser = ShaderParser::parse(std::env::var("WGSL_FILE").unwrap()).unwrap();
        let wgsl_reflection = ShaderReflection::new(wgsl_parser.module);
        let wgsl_entries = wgsl_reflection.reflect_entry();

        println!("WGSL REFLECTION\n");
        iter_reflection(wgsl_entries);
        println!("-----------------------------------------------");

        println!("\n\n\nGLSL REFLECTION\n");
        let vert_parser = ShaderParser::parse(std::env::var("VERT_FILE").unwrap()).unwrap();
        let vert_reflection = ShaderReflection::new(vert_parser.module);
        let vert_entries = vert_reflection.reflect_entry();

        println!("VERTEX REFLECTION\n");
        iter_reflection(vert_entries);
        println!("-----------------------------------------------");

        let frag_parser = ShaderParser::parse(std::env::var("FRAG_FILE").unwrap()).unwrap();
        let frag_reflection = ShaderReflection::new(frag_parser.module);
        let frag_entries = frag_reflection.reflect_entry();

        println!("\n\nFRAGMENT REFLECTION\n");
        iter_reflection(frag_entries);
        println!("-----------------------------------------------");

        let comp_parser = ShaderParser::parse(std::env::var("COMP_FILE").unwrap()).unwrap();
        let comp_reflection = ShaderReflection::new(comp_parser.module);
        let comp_entries = comp_reflection.reflect_entry();

        println!("\n\nCOMPUTE REFLECTION\n");
        iter_reflection(comp_entries);
        println!("-----------------------------------------------");

        println!("\n\n\nSPV REFLECTION\n");

        let spv_parser = ShaderParser::parse(std::env::var("SPV_FILE").unwrap()).unwrap();
        let spv_reflection = ShaderReflection::new(spv_parser.module);
        let spv_entries = spv_reflection.reflect_entry();

        iter_reflection(spv_entries);

        println!("-----------------------------------------------");
    }

    fn iter_reflection(reflection: Vec<ShaderRefEntry>) {
        for entry in reflection {
            println!("Shader Name : {}", entry.name);
            println!("Stage : {:?}", entry.stage);
            println!("EarlyDepthTest : {:?}", entry.early_depth_test);
            println!("Workgroup : {:?}", entry.workgroup);

            println!("Function Name : {:?}", entry.entry_function.name);
            for arg in entry.entry_function.args {
                println!(
                    "Function Arguments : name {:?}, \t binding {:?}, \t type {:?} ",
                    arg.name, arg.binding, arg.ty
                );
            }

            for local_var in entry.entry_function.local_var {
                println!(
                    "Local Variables : name {:?}, \t type {:?}, \t initial value {:?}",
                    local_var.name, local_var.ty, local_var.init
                );
            }

            for named_expr in entry.entry_function.named_expr {
                println!("Named Expression : name {:?}", named_expr.name);

                for named_ex in named_expr.expr {
                    println!("Expression : {:?}", named_ex);
                }
            }

            for expr in entry.entry_function.expr {
                println!("Expression : {:?}", expr);
            }

            for block in entry.entry_function.block {
                println!("Statement : {:?}", block);
            }

            for result in entry.entry_function.result {
                println!(
                    "Result : binding {:?}, \t result {:?}",
                    result.binding, result.ty
                );
            }

            println!("\n\n");
        }
    }

    #[test]
    fn release_module() {
        init_shader_test_env();

        // Will cause a error since into_inner() move out variable thus partial move module
        // Be sure that you are done with reflection before moving out values.

        /*      let wgsl_parse = ShaderParser::parse(std::env::var("WGSL_FILE").unwrap()).unwrap();
        wgsl_parse.module.functions.into_inner();
        wgsl_parse.module.constants.into_inner();

        let err_partial_move = ShaderReflection::new(wgsl_parse.module);*/

        //Will cause a error since ShaderReflection::new(naga::Module) move Module to new(), thus making wgsl_parse lose ownership of Module.

        /* let wgsl_parse = ShaderParser::parse(std::env::var("WGSL_FILE").unwrap()).unwrap();
        let wgsl_reflection = ShaderReflection::new(wgsl_parse.module);
        let wgsl_entries = wgsl_reflection.reflect_entry();
        wgsl_parse.module.constants.into_inner();
        wgsl_parse.module.functions.into_inner();*/

        //To make this work you have to call release on reflection type ex.

        let wgsl_parse = ShaderParser::parse(std::env::var("WGSL_FILE").unwrap()).unwrap();
        let wgsl_reflection = ShaderReflection::new(wgsl_parse.module);
        let wgsl_entries = wgsl_reflection.reflect_entry();

        println!("WGSL Entry Reflection");
        iter_reflection(wgsl_entries);

        let module = wgsl_reflection.release();

        //Cannot use ShaderReflection or the entry
        //println!("{:?}", wgsl_entries[0].entry_function.name);

        let functions = module.functions.into_inner();
        let constants = module.constants.into_inner();
        let types = module.types.into_inner();
        let global_variables = module.global_variables.into_inner();

        func_iter(functions);
        println!("\n");
        constant_iter(constants);
        println!("\n");
        type_iter(types);
        println!("\n");
        global_iter(global_variables);
    }

    fn global_iter(global_variables: Vec<GlobalVariable>) {
        for global_var in global_variables {
            println!("GlobalVariable : name {:?}, \t binding {:?}, \t storage access {:?}, \t class {:?}, \t binding {:?}, \t initial value {:?}", global_var.name, global_var.binding, global_var.storage_access, global_var.class, global_var.binding, global_var.init);
        }
    }

    fn type_iter(types: Vec<Type>) {
        for ty in types {
            println!("Type : name {:?}, \t inner {:?}", ty.name, ty.inner);
        }
    }

    fn constant_iter(constants: Vec<Constant>) {
        for constant in constants {
            println!(
                "Constant : name {:?}, \t inner {:?}, \t specialization {:?}",
                constant.name, constant.inner, constant.specialization
            );
        }
    }

    fn func_iter(functions: Vec<Function>) {
        for func in functions {
            println!("Functions : name {:?}", func.name);

            for arg in func.arguments {
                println!(
                    "function arguments : name {:?}, \t binding {:?}, \t type {:?}",
                    arg.name, arg.binding, arg.ty
                );
            }

            for local in func.local_variables.into_inner().to_owned() {
                println!(
                    "Local variables : name {:?}, \t type {:?}, \t initial value: {:?}",
                    local.name, local.ty, local.init
                );
            }

            for expression in func.expressions.into_inner() {
                println!("Expression : {:?}", expression);
            }

            for named_exp in func.named_expressions {
                println!(
                    "Named Expression :name {:?}, \t handle {:?}",
                    named_exp.1, named_exp.0
                );
            }

            for body in func.body {
                println!("Statement : {:?}", body);
            }

            if let Some(result) = func.result {
                println!(
                    "Result : binding {:?}, \t type {:?}",
                    result.binding, result.ty
                );
            }
        }
    }
}
