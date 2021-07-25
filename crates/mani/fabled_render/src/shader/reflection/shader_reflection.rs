use crate::shader::reflection::*;

pub struct ShaderReflection {
    module: naga::Module,
}

impl ShaderReflection {
    pub fn new(module: naga::Module) -> Self {
        Self { module }
    }

    pub fn reflect_entry(&self) -> Vec<ShaderRefEntry> {
        let entries = &self.module.entry_points;

        let mut local_vars: Vec<naga::LocalVariable> = Vec::new();
        let mut expr_vars: Vec<naga::Expression> = Vec::new();
        let mut ref_entries: Vec<ShaderRefEntry> = Vec::default();

        for entry in entries {
            for (local, expr) in entry
                .function
                .local_variables
                .iter()
                .zip(entry.function.expressions.iter())
            {
                local_vars.push(local.1.to_owned());
                expr_vars.push(expr.1.to_owned());
            }

            let args = Self::retrieve_entry_func_args(self, entry.function.arguments.to_owned());
            let result = Self::retrieve_entry_func_result(self, entry.function.result.to_owned());
            let local_var = Self::retrieve_entry_local_var(self, local_vars.to_owned());
            let named_expr =
                Self::retrieve_entry_name_expr(self, entry.function.named_expressions.to_owned());

            let func_handle = ShaderRefFunc {
                name: entry.function.name.to_owned(),
                args,
                result,
                local_var,
                expr: expr_vars.to_owned(),
                named_expr,
                block: entry.function.body.to_owned(),
            };

            ref_entries.push(ShaderRefEntry {
                name: entry.name.to_owned(),
                stage: entry.stage,
                early_depth_test: entry.early_depth_test,
                workgroup: entry.workgroup_size,
                entry_function: func_handle,
            });
        }

        ref_entries
    }

    fn retrieve_entry_func_args(
        &self,
        func_args: Vec<naga::FunctionArgument>,
    ) -> Vec<FunctionArgument> {
        let mut result: Vec<FunctionArgument> = Vec::new();

        for arg in func_args {
            let type_handle = arg.ty;

            let type_val = self.module.types.try_get(type_handle);

            result.push(FunctionArgument {
                name: arg.name,
                ty: type_val,
                binding: arg.binding,
            })
        }

        result
    }

    fn retrieve_entry_func_result(
        &self,
        func_result: Option<naga::FunctionResult>,
    ) -> Option<FunctionResult> {
        let mut result = None;

        if let Some(res) = func_result {
            let result_id = res.ty;
            result = Some(FunctionResult {
                ty: self.module.types.try_get(result_id),
                binding: res.binding,
            });
        }
        result
    }

    fn retrieve_entry_local_var(
        &self,
        local_vars: Vec<naga::LocalVariable>,
    ) -> Vec<LocalVariables> {
        let mut result: Vec<LocalVariables> = Vec::new();

        for local in local_vars {
            let handle_type = local.ty;

            let var_id = self.module.types.try_get(handle_type);

            let mut init_val = None;
            if let Some(constant_handle) = local.init {
                init_val = self.module.constants.try_get(constant_handle);
            }

            result.push(LocalVariables {
                name: local.name,
                ty: var_id,
                init: init_val,
            });
        }

        result
    }

    fn retrieve_entry_name_expr(
        &self,
        named_expr: naga::FastHashMap<naga::Handle<naga::Expression>, String>,
    ) -> Vec<NamedExpressions> {
        let mut result = Vec::new();

        for expr in named_expr {
            let expr_id = expr.0;

            let mut exprs = Vec::new();
            for entries in 0..self.module.entry_points.len() {
                exprs.push(
                    self.module.entry_points[entries]
                        .function
                        .expressions
                        .try_get(expr_id),
                );
            }

            result.push(NamedExpressions {
                name: Some(expr.1),
                expr: exprs,
            });
        }

        result
    }

    pub fn release(self) -> naga::Module {
        self.module
    }
}

// ------------- Test -------------

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

            if let Some(result) = entry.entry_function.result {
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
