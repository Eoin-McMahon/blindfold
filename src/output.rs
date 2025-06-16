use io::Result as IOResult;
use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
};

use crate::log::log_info;

pub trait Output {
    fn write_gitignore(
        &self,
        contents: Vec<String>,
        append: bool,
        output_path: &Path,
    ) -> IOResult<()>;
    fn write_list<T: Write>(&self, templates: Vec<String>, writer: T) -> IOResult<()>;
    fn write_table<T: Write>(&self, templates: Vec<String>, writer: T) -> IOResult<()>;
}

pub struct FileOutput;

impl Output for FileOutput {
    fn write_gitignore(
        &self,
        contents: Vec<String>,
        append: bool,
        output_path: &Path,
    ) -> IOResult<()> {
        let mut file = match append {
            false => OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(output_path)?,

            true => OpenOptions::new().append(true).open(output_path)?,
        };

        for line in contents {
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }

    fn write_list<T: Write>(&self, templates: Vec<String>, mut writer: T) -> IOResult<()> {
        for template in templates {
            log_info(template, &mut writer)?;
        }
        Ok(())
    }

    fn write_table<T: Write>(
        &self,
        templates: Vec<String>,
        mut writer: T,
    ) -> Result<(), std::io::Error> {
        let col_width = 20;
        let term_width = term_size::dimensions().map(|(w, _h)| w).unwrap_or(80);
        let columns = std::cmp::max(1, term_width / col_width);

        for chunk in templates.chunks(columns) {
            let line = chunk
                .iter()
                .map(|s| format!("{:<width$}", s, width = col_width))
                .collect::<Vec<_>>()
                .join("");

            log_info(line, &mut writer)?;
        }

        Ok(())
    }
}
