use crate::shader::validation_rule::*;
use anyhow::Context;
use naga::valid::ValidationFlags;
use std::error::Error;

impl ValidationLayer for naga::Module {
    fn validate(&self, flag: ValidationFlags) -> anyhow::Result<naga::valid::ModuleInfo> {
        match naga::valid::Validator::new(flag, naga::valid::Capabilities::all()).validate(self) {
            Ok(info) => Some(info),
            Err(err) => {
                #[cfg(target_os = "windows")]
                if let Err(err_code) = ansi_term::enable_ansi_support() {
                    println!("Error has occurred when enabling ansi supported on windows\nWindows error code: {}", err_code);
                    //how to handle 
                }

                eprintln!(
                    "{}\n{:?}",
                    ansi_term::Colour::Red.bold().paint("Validation Failed"),
                    err
                );

                let mut e = err.source();

                while let Some(source) = e {
                    eprintln!("\t{}", source);
                    e = source.source();
                }
                None
            }
        }.context("ModuleInfo was None when running the shader validator. Shader validation most likely failed.")
    }
}
