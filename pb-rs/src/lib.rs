extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate log;
pub mod errors;
pub mod keywords;
pub mod parser;
pub mod scc;
pub mod types;

use errors::Error;
use failure::ResultExt;
use std::path::{Path, PathBuf};
use types::{Config, FileDescriptor};

#[derive(Debug, Default)]
pub struct CompileBuilder {
    in_files: Vec<PathBuf>,
    out_file: Option<PathBuf>,
    include_paths: Vec<PathBuf>,
    single_module: bool,
    no_output: bool,
    error_cycle: bool,
    headers: bool,
}

impl CompileBuilder {
    pub fn new<P: AsRef<Path>>(
        in_files: &[P],
        output: Option<&P>,
        output_dir: Option<&P>,
        include_paths: &[P],
    ) -> Result<CompileBuilder, ::failure::Error> {
        let in_files = in_files
            .iter()
            .map(|f| f.as_ref().into())
            .collect::<Vec<PathBuf>>();
        let output = output.map(|f| f.as_ref().into());
        let output_dir: Option<PathBuf> = output_dir.map(|f| f.as_ref().into());
        let mut include_paths = include_paths
            .iter()
            .map(|f| f.as_ref().into())
            .collect::<Vec<PathBuf>>();

        if in_files.is_empty() {
            return Err(Error::NoProto.into());
        }

        for f in &in_files {
            if !f.exists() {
                return Err(Error::InputFile(format!("{}", f.display())).into());
            }
        }

        let out_file = match (output, output_dir) {
            (Some(_), None) if in_files.len() > 1 => return Err(Error::OutputMultipleInputs.into()),
            (Some(output), None) => Some(output),
            (None, Some(output_dir)) => {
                if !output_dir.is_dir() {
                    return Err(Error::OutputDirectory(format!("{}", output_dir.display())).into());
                }
                Some(output_dir)
            }
            (Some(_), Some(_)) => return Err(Error::OutputAndOutputDir.into()),
            (None, None) => None,
        };

        let default = PathBuf::from(".");
        if include_paths.is_empty() || !include_paths.contains(&default) {
            include_paths.push(default);
        }

        Ok(CompileBuilder {
            in_files: in_files,
            out_file: out_file,
            include_paths: include_paths,
            ..Default::default()
        })
    }

    pub fn single_module(mut self, val: bool) -> Self {
        self.single_module = val;
        self
    }

    pub fn no_output(mut self, val: bool) -> Self {
        self.no_output = val;
        self
    }

    pub fn error_cycle(mut self, val: bool) -> Self {
        self.error_cycle = val;
        self
    }

    pub fn headers(mut self, val: bool) -> Self {
        self.headers = val;
        self
    }

    pub fn compile(self) -> Result<(), ::failure::Error> {
        for in_file in self.in_files {
            let mut out_file = in_file.with_extension("rs");

            if let Some(ref ofile) = self.out_file {
                if ofile.is_dir() {
                    out_file = ofile.join(out_file.file_name().unwrap());
                } else {
                    out_file = ofile.into();
                }
            }

            let config = Config {
                in_file: in_file,
                out_file: out_file,
                import_search_path: self.include_paths.clone(),
                single_module: self.single_module,
                no_output: self.no_output,
                error_cycle: self.error_cycle,
                headers: self.headers,
            };

            FileDescriptor::write_proto(&config).context(format!(
                "Could not convert {} into {}",
                config.in_file.display(),
                config.out_file.display()
            ))?;
        }
        Ok(())
    }
}
