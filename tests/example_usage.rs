// Example usage of the test utilities for output capture and verification
// This demonstrates how to use the utilities created in task 1

mod test_utils;

use lineguard::checker::IssueType;
use test_utils::{
    TestFile, TestableGitHubReporter, TestableHumanReporter, TestableJsonReporter,
    TestableReporter, create_sample_check_result, create_sample_issue,
    setup_integration_test_environment, verification, with_test_files,
};

/// Example: Testing JSON reporter output with proper verification
#[test]
fn example_json_reporter_test() {
    let reporter = TestableJsonReporter;

    // Create test data using utilities
    let issue1 = create_sample_issue(
        IssueType::MissingNewline,
        None,
        "Missing newline at end of file",
    );
    let issue2 = create_sample_issue(IssueType::TrailingSpace, Some(15), "Trailing spaces found");

    let results = vec![
        create_sample_check_result("src/main.rs", vec![issue1]),
        create_sample_check_result("src/lib.rs", vec![issue2]),
    ];

    // Capture output using testable reporter
    let output = reporter.report_to_string(&results);

    // Verify output using verification utilities
    assert!(verification::is_valid_json(&output));
    assert!(verification::verify_json_output(&output, 2, 2)); // 2 files, 2 issues
    assert_eq!(verification::count_files_in_json(&output), Some(2));
    assert_eq!(verification::count_issues_in_json(&output), Some(2));

    // Verify specific content
    assert!(output.contains("src/main.rs"));
    assert!(output.contains("src/lib.rs"));
    assert!(output.contains("missing_newline"));
    assert!(output.contains("trailing_space"));
}

/// Example: Testing GitHub Actions reporter output
#[test]
fn example_github_reporter_test() {
    let reporter = TestableGitHubReporter;

    let issue = create_sample_issue(
        IssueType::MultipleNewlines,
        Some(100),
        "Multiple trailing newlines",
    );
    let results = vec![create_sample_check_result("docs/README.md", vec![issue])];

    let output = reporter.report_to_string(&results);

    // Verify GitHub Actions annotation format
    assert!(verification::verify_github_output(
        &output,
        &[
            "::error file=docs/README.md,line=100",
            "Multiple trailing newlines"
        ]
    ));

    // Verify exact format
    assert_eq!(
        output.trim(),
        "::error file=docs/README.md,line=100::Multiple trailing newlines"
    );
}

/// Example: Testing human-readable reporter output
#[test]
fn example_human_reporter_test() {
    let reporter = TestableHumanReporter { use_color: false };

    let issues = vec![
        create_sample_issue(IssueType::TrailingSpace, Some(5), "Trailing space"),
        create_sample_issue(IssueType::MissingNewline, None, "Missing newline"),
    ];
    let results = vec![create_sample_check_result("config.toml", issues)];

    let output = reporter.report_to_string(&results);

    // Verify human-readable patterns
    assert!(verification::verify_human_output(
        &output,
        &[
            "✗ config.toml",
            "Line 5: Trailing space",
            "Missing newline",
            "Found 2 issues in 1 files"
        ]
    ));
}

/// Example: Using test file utilities for integration testing
#[test]
fn example_test_file_utilities() {
    // Create test files with specific content
    let test_files = vec![
        TestFile::with_issues("bad.txt", "content without newline"),
        TestFile::clean("good.txt", "content with proper ending"),
        TestFile::new("mixed.txt", "line 1\nline 2 \nline 3\n", true), // Has trailing space
    ];

    // Setup test environment
    let (_temp_dir, file_paths) = setup_integration_test_environment(&test_files);

    // Verify files were created correctly
    assert_eq!(file_paths.len(), 3);
    for path in &file_paths {
        assert!(path.exists());
    }

    // Verify file contents
    let bad_content = std::fs::read_to_string(&file_paths[0]).unwrap();
    assert_eq!(bad_content, "content without newline");
    assert!(!bad_content.ends_with('\n'));

    let good_content = std::fs::read_to_string(&file_paths[1]).unwrap();
    assert_eq!(good_content, "content with proper ending\n");
    assert!(good_content.ends_with('\n'));
}

/// Example: Using with_test_files for simpler test scenarios
#[test]
fn example_with_test_files() {
    with_test_files(
        &[
            ("test1.txt", "content1"),
            ("test2.txt", "content2\n"),
            ("test3.txt", "content3 "), // trailing space
        ],
        |_temp_dir, paths| {
            assert_eq!(paths.len(), 3);

            // All files should exist
            for path in paths {
                assert!(path.exists());
            }

            // Verify specific content
            let content3 = std::fs::read_to_string(&paths[2]).unwrap();
            assert!(content3.ends_with(' '));
        },
    );
}

/// Example: Comprehensive reporter testing with error scenarios
#[test]
fn example_comprehensive_reporter_test() {
    let json_reporter = TestableJsonReporter;
    let github_reporter = TestableGitHubReporter;
    let human_reporter = TestableHumanReporter { use_color: true };

    // Create complex test scenario
    let results = vec![
        create_sample_check_result(
            "src/main.rs",
            vec![create_sample_issue(
                IssueType::MissingNewline,
                None,
                "Missing newline at EOF",
            )],
        ),
        create_sample_check_result(
            "src/lib.rs",
            vec![
                create_sample_issue(
                    IssueType::TrailingSpace,
                    Some(10),
                    "Trailing space on line 10",
                ),
                create_sample_issue(
                    IssueType::TrailingSpace,
                    Some(25),
                    "Trailing space on line 25",
                ),
            ],
        ),
        create_sample_check_result(
            "tests/mod.rs",
            vec![create_sample_issue(
                IssueType::MultipleNewlines,
                Some(50),
                "Multiple trailing newlines",
            )],
        ),
        create_sample_check_result("README.md", vec![]), // Clean file
    ];

    // Test all reporters
    let json_output = json_reporter.report_to_string(&results);
    let github_output = github_reporter.report_to_string(&results);
    let human_output = human_reporter.report_to_string(&results);

    // Comprehensive JSON verification
    assert!(verification::is_valid_json(&json_output));
    assert!(verification::verify_json_output(&json_output, 4, 4)); // 4 files, 4 issues total
    assert_eq!(verification::count_files_in_json(&json_output), Some(4));
    assert_eq!(verification::count_issues_in_json(&json_output), Some(4));

    // GitHub output should contain all error annotations
    let expected_github_patterns = &[
        "::error file=src/main.rs::Missing newline at EOF",
        "::error file=src/lib.rs,line=10::Trailing space on line 10",
        "::error file=src/lib.rs,line=25::Trailing space on line 25",
        "::error file=tests/mod.rs,line=50::Multiple trailing newlines",
    ];
    assert!(verification::verify_github_output(
        &github_output,
        expected_github_patterns
    ));

    // Human output should show summary and details
    let expected_human_patterns = &[
        "Found 4 issues in 3 files",
        "Files checked: 4",
        "src/main.rs",
        "src/lib.rs",
        "tests/mod.rs",
    ];
    assert!(verification::verify_human_output(
        &human_output,
        expected_human_patterns
    ));

    // Clean file should not appear in human output (only files with issues are shown)
    assert!(!human_output.contains("README.md"));
}
