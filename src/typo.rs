use crate::log::log_error;
use std::io::Write;
use strsim::levenshtein;

/// Checks the given user inputs against available templates.
/// Logs all typos and suggestions using your logger functions.
/// Returns `true` if any typos were found, else `false`.
pub fn check_typos<T: Write>(
    handle: &mut T,
    user_inputs: &[&str],
    available_templates: &[String],
) -> bool {
    let mut found_typos = false;

    for input in user_inputs {
        if !available_templates.iter().any(|t| t == input) {
            found_typos = true;
            let suggestion = find_closest_match(input, available_templates);
            match suggestion {
                Some(s) => {
                    let _ = log_error(
                        format!("Did not recognize '{}' – perhaps you meant '{}'", input, s),
                        handle,
                    );
                }
                None => {
                    let _ = log_error(format!("Did not recognize '{}'", input), handle);
                }
            }
        }
    }

    found_typos
}

/// Find closest string match between input and candidates
fn find_closest_match<'a>(input: &str, candidates: &'a [String]) -> Option<&'a str> {
    candidates
        .iter()
        .map(|c| (levenshtein(input, c), c))
        .filter(|(dist, _)| *dist <= 3) // Only allow small typo margin
        .min_by_key(|(dist, _)| *dist)
        .map(|(_, c)| c.as_str())
}
