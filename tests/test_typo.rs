use blindfold::typo::check_typos;
use std::io::Cursor;

#[test]
fn test_no_typos() {
    let available = vec![
        "rust".to_string(),
        "python".to_string(),
        "javascript".to_string(),
    ];
    let inputs = vec!["rust", "python"];

    let mut buffer = Cursor::new(Vec::new());
    let result = check_typos(&mut buffer, &inputs, &available);
    assert_eq!(result, false); // no typos found
}
