mod common;

use markdown_fmt::rewrite_markdown;

#[test]
fn check_markdown_formatting() {
    let mut errors = 0;

    for file in std::fs::read_dir("tests/source")
        .unwrap()
        .map(Result::unwrap)
    {
        let filename = file.file_name();
        let input = std::fs::read_to_string(file.path()).unwrap();
        let formatted_input = rewrite_markdown(&input).unwrap();
        let target_file = format!("tests/target/{}", filename.to_str().unwrap());
        let expected_output = std::fs::read_to_string(target_file).unwrap();

        if formatted_input != expected_output {
            errors += 1;
        }
    }

    assert_eq!(errors, 0, "there should be no formatting error");
}

#[test]
fn idempotence_test() {
    let mut errors = 0;

    for file in std::fs::read_dir("tests/target")
        .unwrap()
        .map(Result::unwrap)
    {
        let input = std::fs::read_to_string(file.path()).unwrap();
        let formatted_input = rewrite_markdown(&input).unwrap();

        if formatted_input != input {
            errors += 1;
        }
    }

    assert_eq!(errors, 0, "formatting should not change in target files");
}
