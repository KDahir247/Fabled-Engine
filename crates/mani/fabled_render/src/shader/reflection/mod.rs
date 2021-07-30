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
