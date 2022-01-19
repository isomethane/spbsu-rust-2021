#[cfg(test)]
mod tests {
    use phone_numbers::PhoneEncoder;
    use regex::Regex;
    use std::collections::{HashMap, HashSet};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_sample() {
        run_test("sample");
    }

    #[test]
    fn test_large() {
        run_test("large");
    }

    fn read_output<P: AsRef<Path>>(output: P) -> Vec<(String, String)> {
        let output_lines =
            BufReader::new(File::open(output).expect("Failed to open output file")).lines();

        let mut result = vec![];
        let encoding_regex = Regex::new(r"^([0-9/-]+): (.*)$").unwrap();
        for line in output_lines {
            let line = line.expect("Failed to read output");
            let captures = encoding_regex
                .captures(line.as_str())
                .expect("line does not match pattern");
            let phone_number = &captures[1];
            let phone_encoding = &captures[2];
            result.push((phone_number.to_owned(), phone_encoding.to_owned()));
        }

        result
    }

    fn output_unordered(output: &Vec<(String, String)>) -> HashMap<&String, HashSet<&String>> {
        output.iter().fold(HashMap::new(), |m, (k, v)| {
            let mut m = m;
            if !m.contains_key(k) {
                m.insert(k, HashSet::new());
            }
            m.get_mut(k).unwrap().insert(v);
            m
        })
    }

    fn compare_outputs<P: AsRef<Path>>(expected_output: P, actual_output: P) {
        let expected_output = read_output(expected_output);
        let actual_output = read_output(actual_output);

        assert_eq!(
            expected_output
                .iter()
                .map(|o| { &o.0 })
                .collect::<Vec<&String>>(),
            actual_output
                .iter()
                .map(|o| { &o.0 })
                .collect::<Vec<&String>>()
        );
        assert_eq!(
            output_unordered(&expected_output),
            output_unordered(&actual_output)
        )
    }

    fn run_test(test_name: &str) {
        let test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources/test")
            .join(test_name);

        let dictionary_path = test_path.join("dictionary.txt");
        let input_path = test_path.join("input.txt");
        let expected_output_path = test_path.join("output.txt");
        let actual_output_path = test_path.join("actual_output.txt");

        PhoneEncoder::encode_phones(
            dictionary_path.as_path(),
            input_path.as_path(),
            actual_output_path.as_path(),
        );
        compare_outputs(expected_output_path.as_path(), actual_output_path.as_path());
    }
}
