use crate::Context;
use lazy_static::lazy_static;
use ocl::{
    self,
    enums::{ProgramBuildInfo as Pbi, ProgramBuildInfoResult as Pbir},
};
use ocl_include::{self, Hook};
use regex::{Captures, Regex, RegexBuilder};
use std::path::Path;

lazy_static! {
    static ref LOCATION: Regex = RegexBuilder::new(r#"^([^:\r\n]*):(\d*):(\d*):"#)
        .multi_line(true)
        .build()
        .unwrap();
}

/// Handler for source code of the device OpenCL program.
///
/// It is responsible for building the program and showing errors and warnings it they're exist.
pub struct Program {
    source: String,
    index: ocl_include::Index,
}

impl Program {
    pub fn new<H: Hook>(hook: &H, main: &Path) -> crate::Result<Self> {
        let node = ocl_include::build(hook, main)?;
        let (source, index) = node.collect();

        Ok(Self { source, index })
    }

    pub fn source(&self) -> String {
        self.source.clone()
    }

    fn replace_index(&self, message: &str) -> String {
        LOCATION
            .replace_all(&message, |caps: &Captures| -> String {
                caps[2]
                    .parse::<usize>()
                    .map_err(|_| ())
                    .and_then(|line| self.index.search(line - 1).ok_or(()))
                    .and_then(|(path, local_line)| {
                        Ok(format!(
                            "{}:{}:{}:",
                            path.to_string_lossy(),
                            local_line + 1,
                            &caps[3],
                        ))
                    })
                    .unwrap_or(caps[0].to_string())
            })
            .into_owned()
    }

    pub fn build(&self, context: &Context) -> crate::Result<(ocl::Program, String)> {
        ocl::Program::builder()
            .devices(context.device())
            .source(self.source.clone())
            .build(context.context())
            .and_then(|p| {
                p.build_info(context.device().clone(), Pbi::BuildLog)
                    .map(|pbi| match pbi {
                        Pbir::BuildLog(s) => s,
                        _ => unreachable!(),
                    })
                    .map(|log| {
                        if log.len() > 0 {
                            println!("Build log: {}", log);
                        }
                        (p, self.replace_index(&log))
                    })
                    .map_err(|e| e.into())
            })
            .map_err(|e| {
                let message = self.replace_index(&e.to_string());
                ocl::Error::from(ocl::core::Error::from(message))
            })
            .map_err(|e| e.into())
    }
}
