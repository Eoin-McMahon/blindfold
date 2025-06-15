use io::Result as IOResult;
use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
};

pub struct TemplateOutput;

impl TemplateOutput {
    pub fn write_list<T: Write>(&self, templates: Vec<String>, mut writer: T) -> IOResult<()> {
        for template in templates {
            writeln!(writer, "{}", template)?;
        }
        Ok(())
    }

    pub fn write_table<T: Write>(&self, templates: Vec<String>, mut writer: T) -> IOResult<()> {
        let col_width = 20;

        // Get terminal width or default to 80 columns
        let term_width = term_size::dimensions().map(|(w, _h)| w).unwrap_or(80);

        // Compute columns that fit in terminal width
        let columns = std::cmp::max(1, term_width / col_width);

        for chunk in templates.chunks(columns) {
            let line = chunk
                .iter()
                .map(|s| format!("{:<width$}", s, width = col_width))
                .collect::<Vec<_>>()
                .join("");

            writeln!(writer, "{}", line)?;
        }
        Ok(())
    }
}

pub trait Output {
    fn write(&self, contents: Vec<String>, append: bool, output_path: &Path) -> IOResult<()>;
}

pub struct FileOutput;

impl Output for FileOutput {
    fn write(&self, contents: Vec<String>, append: bool, output_path: &Path) -> IOResult<()> {
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
}
