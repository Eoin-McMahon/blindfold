use std::{
    fs::OpenOptions,
    io::{self, ErrorKind, Write},
    path::Path,
};
use term_size;

pub struct TemplateOutput;

impl TemplateOutput {
    pub fn write_list(&self, templates: Vec<String>) {
        for template in templates {
            println!("{}", template);
        }
    }

    pub fn write_table(&self, templates: Vec<String>) {
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
            println!("{}", line);
        }
    }
}

pub trait Output {
    fn write(&self, contents: Vec<String>, append: bool, output_path: &Path) -> io::Result<()>;
}

pub struct FileOutput;

impl Output for FileOutput {
    fn write(&self, contents: Vec<String>, append: bool, output_path: &Path) -> io::Result<()> {
        let mut file = match append {
            false => OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(output_path)?,

            true => OpenOptions::new()
                .write(true)
                .append(true)
                .open(output_path)
                .map_err(|e| {
                    if e.kind() == ErrorKind::NotFound {
                        io::Error::new(
                            ErrorKind::NotFound,
                            format!(
                                "Unable to append to '{}' as it was not found",
                                output_path.display()
                            ),
                        )
                    } else {
                        e
                    }
                })?,
        };

        for line in contents {
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }
}
