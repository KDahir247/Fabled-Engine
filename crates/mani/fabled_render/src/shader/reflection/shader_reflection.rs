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
