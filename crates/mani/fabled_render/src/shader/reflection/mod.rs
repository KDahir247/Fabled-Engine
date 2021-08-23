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
mod data_test {
    use crate::shader::reflection::*;

    #[test]
    fn data_size() {
        let func_arg_size = std::mem::size_of::<FunctionArgument>();
        println!("FunctionArgument is {} bytes", func_arg_size);

        let func_result_size = std::mem::size_of::<FunctionResult>();
        println!("FunctionResult is {} bytes", func_result_size);

        let local_var_size = std::mem::size_of::<LocalVariables>();
        println!("Local Variable is {} bytes", local_var_size);

        let named_expr_size = std::mem::size_of::<NamedExpressions>();
        println!("Named Expressions is {} bytes", named_expr_size);

        let ref_func_size = std::mem::size_of::<ShaderRefFunc>();
        println!("ShaderRefFunc is {} bytes", ref_func_size);

        let ref_entry_size = std::mem::size_of::<ShaderRefEntry>();
        println!("ShaderRefEntry is {} bytes", ref_entry_size);
    }

    #[test]
    fn data_alignment() {
        let func_arg_alignment = std::mem::size_of::<FunctionArgument>();
        println!(
            "Function Argument is aligned to {} bytes",
            func_arg_alignment
        );

        let func_result_alignment = std::mem::align_of::<FunctionResult>();
        println!(
            "Function Result is aligned to {} bytes",
            func_result_alignment
        );

        let local_var_alignment = std::mem::align_of::<LocalVariables>();
        println!(
            "Local Variables is aligned to {} bytes",
            local_var_alignment
        );

        let named_expr_alignment = std::mem::align_of::<NamedExpressions>();
        println!(
            "Named Expression is aligned to {} bytes",
            named_expr_alignment
        );

        let ref_func_alignment = std::mem::align_of::<ShaderRefFunc>();
        println!(" ShaderRefFunc is aligned to {} bytes", ref_func_alignment);

        let ref_entry_alignment = std::mem::align_of::<ShaderRefEntry>();
        println!(" ShaderRefEntry is {} bytes", ref_entry_alignment);
    }
}
