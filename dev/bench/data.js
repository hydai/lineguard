window.BENCHMARK_DATA = {
  "lastUpdate": 1769324726859,
  "repoUrl": "https://github.com/hydai/lineguard",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "distinct": true,
          "id": "8ad6568b07638473c4a3cf58df001144483e0e61",
          "message": "fix: add git user configuration in benchmark workflow\n\n- Configure git user.name and user.email for GitHub Actions bot\n- Prevents \"Author identity unknown\" error when creating gh-pages branch\n\n🤖 Generated with [Claude Code](https://claude.ai/code)\n\nCo-Authored-By: Claude <noreply@anthropic.com>",
          "timestamp": "2025-07-12T06:14:19+08:00",
          "tree_id": "1d9f780667ada8b00396bee7f3411a667bc61d96",
          "url": "https://github.com/hydai/lineguard/commit/8ad6568b07638473c4a3cf58df001144483e0e61"
        },
        "date": 1752272198653,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0013332274322092224,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.0013063631200256741,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.0009778145773968927,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.0018261996326366563,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.005002390938202236,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "distinct": true,
          "id": "41df0f51e86666aa9dcc9593254380454b8669a9",
          "message": "chore(version): bump to v0.1.1\n\nSigned-off-by: hydai <z54981220@gmail.com>",
          "timestamp": "2025-07-12T06:29:42+08:00",
          "tree_id": "4d6eaaef2ca49b8a8f994068570bdca8817dfaf8",
          "url": "https://github.com/hydai/lineguard/commit/41df0f51e86666aa9dcc9593254380454b8669a9"
        },
        "date": 1752273120367,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.001334727462768362,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.0013282277418985712,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.0009766852344474395,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.0018293818798352536,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.00499782435798507,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "distinct": true,
          "id": "36a96823ca05b9b476b092333f43a895056ffd63",
          "message": "feat: add verbose output for git range information\n\nWhen using --from and --to flags with --verbose, lineguard now displays:\n- The git commit range being checked\n- The number of changed files\n- A list of all changed files in the range\n\nThis helps users understand which files are being checked when using\ngit range filtering.\n\n🤖 Generated with [Claude Code](https://claude.ai/code)\n\nCo-Authored-By: Claude <noreply@anthropic.com>",
          "timestamp": "2025-07-12T07:11:09+08:00",
          "tree_id": "6ce5560ab6bf009690d81a4c56835b031178a429",
          "url": "https://github.com/hydai/lineguard/commit/36a96823ca05b9b476b092333f43a895056ffd63"
        },
        "date": 1752275576018,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0013268042492810468,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.001322751037692311,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.0009592457361105221,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.0018462910797485356,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.005045486110188678,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "distinct": true,
          "id": "98ef19ac3c7e00daa5b538c2c9d9a19c08844904",
          "message": "chore(version): bump to v0.1.2\n\nSigned-off-by: hydai <z54981220@gmail.com>",
          "timestamp": "2025-07-12T07:12:39+08:00",
          "tree_id": "adc125b6cc514bd9c22edd0647434d63d1e45dad",
          "url": "https://github.com/hydai/lineguard/commit/98ef19ac3c7e00daa5b538c2c9d9a19c08844904"
        },
        "date": 1752275636206,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0013243070874631052,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.0013056224165183751,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.000965014068167538,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.0018203332540597257,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.0049678762890167,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "distinct": true,
          "id": "7efbe84aaf7bfb4aaa05dff432b0e3649fc7013d",
          "message": "fix: resolve benchmark CI failure due to non-zero exit codes\n\n- Remove test files with lint issues that caused exit code 1\n- Add --ignore-failure flag to hyperfine for robustness\n\n🤖 Generated with [Claude Code](https://claude.ai/code)\n\nCo-Authored-By: Claude <noreply@anthropic.com>",
          "timestamp": "2025-07-12T12:47:47+08:00",
          "tree_id": "1d96a89b6cbe555cb450bc96accb47bb3342f411",
          "url": "https://github.com/hydai/lineguard/commit/7efbe84aaf7bfb4aaa05dff432b0e3649fc7013d"
        },
        "date": 1752295754382,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0018894703006151292,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.005944497243636364,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.0571906778614815,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.07731343539219511,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07928680250857142,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "distinct": true,
          "id": "29fca6756490f3ed51f0de6ddca5033a03498b85",
          "message": "Improve error messages and test assertions based on reviewer feedback\n\n- Enhanced git error messages to include stderr output for better debugging\n- Updated test assertions to use .code(1) instead of .failure() when issues are found\n- Added better error context in test helper functions with .with_context()\n- Maintained .failure() for actual error conditions (invalid refs, missing repos)\n\nThis provides more specific and helpful error information for both users and developers.\n\nSigned-off-by: hydai <z54981220@gmail.com>",
          "timestamp": "2025-07-13T15:17:19+08:00",
          "tree_id": "0f6a62f7747a8a265444797f40e2bcd88e6cf404",
          "url": "https://github.com/hydai/lineguard/commit/29fca6756490f3ed51f0de6ddca5033a03498b85"
        },
        "date": 1752391114073,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.001907066064775088,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.006074990415643566,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.058256450508301895,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.07699856778555557,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07952673595435898,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "distinct": true,
          "id": "13c4c41df11a316d961282493ae802d932a7a07e",
          "message": "chore(version): bump to v0.1.4\n\nSigned-off-by: hydai <z54981220@gmail.com>",
          "timestamp": "2025-07-13T15:18:11+08:00",
          "tree_id": "282b2c25898aa23325b77b24eb29d9601425ab25",
          "url": "https://github.com/hydai/lineguard/commit/13c4c41df11a316d961282493ae802d932a7a07e"
        },
        "date": 1752391210320,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0019214410093064075,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.005842621946637747,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.05705907499259261,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.07572762465,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07846487975897434,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "distinct": true,
          "id": "02f43522e3d24b8e8ef1c5e4d4fbfd5c93164457",
          "message": "docs: add shell command alternatives to README\n\nAdd comprehensive shell command alternatives for users who prefer not to install LineGuard:\n- Basic file checking with find and grep\n- Automatic fixing of issues\n- Git integration examples\n- Advanced usage with GNU parallel\n- Shell function for .bashrc/.zshrc\n- Feature comparison table\n\n🤖 Generated with [Claude Code](https://claude.ai/code)\n\nCo-Authored-By: Claude <noreply@anthropic.com>",
          "timestamp": "2025-07-13T21:45:07+08:00",
          "tree_id": "f1ad4d79b9a3bc18141ec41f3d3a44447cacb6fb",
          "url": "https://github.com/hydai/lineguard/commit/02f43522e3d24b8e8ef1c5e4d4fbfd5c93164457"
        },
        "date": 1752414396072,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.001949726795698924,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.0059507363208791235,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.05747028146296295,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.07705007677777778,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.0792765894871795,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c16199d1f9717b5706174d13f00846ea641e1e60",
          "message": "ci(deps): bump actions/configure-pages from 4 to 5 (#12)\n\nBumps [actions/configure-pages](https://github.com/actions/configure-pages) from 4 to 5.\n- [Release notes](https://github.com/actions/configure-pages/releases)\n- [Commits](https://github.com/actions/configure-pages/compare/v4...v5)\n\n---\nupdated-dependencies:\n- dependency-name: actions/configure-pages\n  dependency-version: '5'\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-07-14T12:01:36+08:00",
          "tree_id": "632861877428f6534415215ea938ba83cfc78fe0",
          "url": "https://github.com/hydai/lineguard/commit/c16199d1f9717b5706174d13f00846ea641e1e60"
        },
        "date": 1752465779580,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0018990030263192194,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.00586698295094092,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.057409546521818185,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.07470471464,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07877236478615383,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ef01c1225b767f80d9154e587f870a1f72626d6d",
          "message": "fix: apply lineguard to its own repo (#13)\n\nSigned-off-by: hydai <z54981220@gmail.com>",
          "timestamp": "2025-07-14T19:50:26+08:00",
          "tree_id": "3c11bd953098f62856ec54729db5e1d41743474a",
          "url": "https://github.com/hydai/lineguard/commit/ef01c1225b767f80d9154e587f870a1f72626d6d"
        },
        "date": 1752493901621,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.001885654746765476,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.005854921919783077,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.056423526208888894,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.07626727560378378,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.08041786627714288,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2da3ca01b863abe563bd08195fc338056bf5f0d2",
          "message": "ci: optimize GitHub Actions workflows to reduce usage (#14)\n\n- Remove beta/nightly Rust testing (reduce test matrix from 9 to 3 jobs)\n- Add path filtering to skip runs for documentation-only changes\n- Prevent duplicate runs when PRs are merged (push/PR overlap)\n- Add concurrency control to cancel outdated workflow runs\n- Restrict benchmark runs to master push, manual trigger, or PRs with 'benchmark' label\n- Move coverage job to master-only execution\n- Add caching for cargo-audit and cargo-tarpaulin tools\n- Fix cargo upgrade command for newer cargo-edit versions\n- Limit size-check job to PRs only\n\nThese optimizations should reduce GitHub Actions usage by ~60-70% while\nmaintaining necessary CI/CD quality checks.\n\n🤖 Generated with [Claude Code](https://claude.ai/code)\n\nCo-authored-by: Claude <noreply@anthropic.com>",
          "timestamp": "2025-07-15T13:31:25+08:00",
          "tree_id": "176d19992b01816a7cafbb6034d524a119c3661f",
          "url": "https://github.com/hydai/lineguard/commit/2da3ca01b863abe563bd08195fc338056bf5f0d2"
        },
        "date": 1752557556617,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0018906322710699227,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.005865893790023313,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.05716697898000002,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.07519095250439024,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07785223658256409,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cd26e5cde4f48a67ca2b5122047d97308a824ef8",
          "message": "chore(coverage): improve the coverage (#15)\n\n* chore(coverage): improve the coverage\n\nSigned-off-by: hydai <z54981220@gmail.com>\n\n* chore(lineguard): apply lineguard again\n\nSigned-off-by: hydai <z54981220@gmail.com>\n\n* fix: the edge cases\n\nSigned-off-by: hydai <z54981220@gmail.com>\n\n* fix: the error on windows\n\nSigned-off-by: hydai <z54981220@gmail.com>\n\n* fix: apply the gemini review comments\n\nSigned-off-by: hydai <z54981220@gmail.com>\n\n---------\n\nSigned-off-by: hydai <z54981220@gmail.com>",
          "timestamp": "2025-07-16T03:10:28+08:00",
          "tree_id": "f36033ac80071bbb661adc91ee29b1668d998e75",
          "url": "https://github.com/hydai/lineguard/commit/cd26e5cde4f48a67ca2b5122047d97308a824ef8"
        },
        "date": 1752606770965,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.001887565528987854,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.00579639609868132,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.05558521459818182,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.07352463811170734,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07697618600162161,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8b12b80647cf706595dad824929cd09d34435bfa",
          "message": "fix(ci): update existing PR comment instead of creating new ones for binary size report (#16)\n\n🤖 Generated with [Claude Code](https://claude.ai/code)\n\nCo-authored-by: Claude <noreply@anthropic.com>",
          "timestamp": "2025-07-16T03:15:45+08:00",
          "tree_id": "b23fc3d30e7f9e1224ae18fbc7f2ac5f883961f6",
          "url": "https://github.com/hydai/lineguard/commit/8b12b80647cf706595dad824929cd09d34435bfa"
        },
        "date": 1752607022820,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0018764783768600688,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.005800593262939862,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.055296727612727276,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.07515886530585368,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07667548316857144,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "distinct": true,
          "id": "d4f1e291b59128771f62cc1066b51a5ac4668761",
          "message": "fix: correct MockOutput example in CONTRIBUTING.md\n\nFix MockOutput usage example to match actual API:\n- Remove non-existent set_color_support() method\n- Fix buffer assertion: use get_output() instead of direct buffer access\n- Add missing Color import statement\n- Use contains_colored() method for colored output verification\n- Correct write_line() behavior: produces [\"Hello\", \"\\n\"] not [\"Hello\\n\"]\n\nThe example now accurately demonstrates MockOutput's actual API.\n\n🤖 Generated with [Claude Code](https://claude.ai/code)\n\nCo-Authored-By: Claude <noreply@anthropic.com>",
          "timestamp": "2025-07-17T15:05:44+08:00",
          "tree_id": "b6fc764741519d19c686f3d839c0d85d1a1b22ad",
          "url": "https://github.com/hydai/lineguard/commit/d4f1e291b59128771f62cc1066b51a5ac4668761"
        },
        "date": 1752736020887,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0019066062010158205,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.005974001913362832,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.056844390086666675,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.07495592063951219,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.0788517850353846,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7077b08545abf3c57557f9f30410104b6fdf26f1",
          "message": "chore(deps): bump serde_json from 1.0.140 to 1.0.141 (#18)\n\nBumps [serde_json](https://github.com/serde-rs/json) from 1.0.140 to 1.0.141.\n- [Release notes](https://github.com/serde-rs/json/releases)\n- [Commits](https://github.com/serde-rs/json/compare/v1.0.140...v1.0.141)\n\n---\nupdated-dependencies:\n- dependency-name: serde_json\n  dependency-version: 1.0.141\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-07-22T10:34:41+08:00",
          "tree_id": "7adc2059cf3daaf79f3bddbd53250fc6b2e6a7e0",
          "url": "https://github.com/hydai/lineguard/commit/7077b08545abf3c57557f9f30410104b6fdf26f1"
        },
        "date": 1753151809407,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.001907651478448567,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.005966534575555558,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.05745306613259261,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.07776031406777778,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07899889104,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e00e19a9c98eace5904fd670f7ed7c1c6216a5a0",
          "message": "chore(deps): bump toml from 0.9.2 to 0.9.4 (#19)\n\nBumps [toml](https://github.com/toml-rs/toml) from 0.9.2 to 0.9.4.\n- [Commits](https://github.com/toml-rs/toml/compare/toml-v0.9.2...toml-v0.9.4)\n\n---\nupdated-dependencies:\n- dependency-name: toml\n  dependency-version: 0.9.4\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-08-04T16:11:35+08:00",
          "tree_id": "98a577daac35cfa161561b922fc0fefa78094e7d",
          "url": "https://github.com/hydai/lineguard/commit/e00e19a9c98eace5904fd670f7ed7c1c6216a5a0"
        },
        "date": 1754295167536,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.001898998361847915,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.00586876333010941,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.05710044630727272,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.07630844546780488,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07873193262102565,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b0e80abe65d22a6e38f400621bb4973b363a5ea3",
          "message": "chore(deps): bump serde_json from 1.0.141 to 1.0.142 (#20)\n\nBumps [serde_json](https://github.com/serde-rs/json) from 1.0.141 to 1.0.142.\n- [Release notes](https://github.com/serde-rs/json/releases)\n- [Commits](https://github.com/serde-rs/json/compare/v1.0.141...v1.0.142)\n\n---\nupdated-dependencies:\n- dependency-name: serde_json\n  dependency-version: 1.0.142\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-08-04T16:34:47+08:00",
          "tree_id": "fd3abaad09b3f3cef0ce61c3266ef916c417c02d",
          "url": "https://github.com/hydai/lineguard/commit/b0e80abe65d22a6e38f400621bb4973b363a5ea3"
        },
        "date": 1754296613959,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0019075730683134596,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.005940348934490238,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.057068976320833346,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.07751997407499998,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07940233685555557,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c68d33c15efd4cf33e58b4e58bb93cb04ba0d939",
          "message": "chore(deps): bump clap from 4.5.41 to 4.5.42 (#21)\n\nBumps [clap](https://github.com/clap-rs/clap) from 4.5.41 to 4.5.42.\n- [Release notes](https://github.com/clap-rs/clap/releases)\n- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.41...clap_complete-v4.5.42)\n\n---\nupdated-dependencies:\n- dependency-name: clap\n  dependency-version: 4.5.42\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-08-04T16:42:01+08:00",
          "tree_id": "23cc29e88a2c493568639329d27aacb794259030",
          "url": "https://github.com/hydai/lineguard/commit/c68d33c15efd4cf33e58b4e58bb93cb04ba0d939"
        },
        "date": 1754297034890,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0019025071662222216,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.006026221301785714,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.058290083233333345,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.07705338695555555,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07995711565000002,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "distinct": true,
          "id": "91e4cf5161ed26e346226c851f7afb7b038de832",
          "message": "chore: apply cargo fmt and clippy\n\nSigned-off-by: hydai <z54981220@gmail.com>",
          "timestamp": "2025-09-12T16:16:38+08:00",
          "tree_id": "9d7fd9ec77a6d86ebf24d8295a686f830b622a5a",
          "url": "https://github.com/hydai/lineguard/commit/91e4cf5161ed26e346226c851f7afb7b038de832"
        },
        "date": 1757665079328,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.001905797412173915,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.005874794426117137,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.05739459022181818,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.06064447960521739,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07719380868102567,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7289ca26577bcd493c3d65182666ca7ba83b59f0",
          "message": "ci(deps): bump actions/checkout from 4 to 5 (#24)\n\nBumps [actions/checkout](https://github.com/actions/checkout) from 4 to 5.\n- [Release notes](https://github.com/actions/checkout/releases)\n- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/actions/checkout/compare/v4...v5)\n\n---\nupdated-dependencies:\n- dependency-name: actions/checkout\n  dependency-version: '5'\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-09-12T16:48:10+08:00",
          "tree_id": "aff8620a14256d18ebd335bb705d1bbc257307f2",
          "url": "https://github.com/hydai/lineguard/commit/7289ca26577bcd493c3d65182666ca7ba83b59f0"
        },
        "date": 1757666974011,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0019085587185714264,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.005853742782299355,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.05619073427636363,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.06051526746,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.0777853268345946,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "distinct": true,
          "id": "2f9533ae440d2a7bb9962c2eee677d56cb965e15",
          "message": "chore(version): bump to v0.1.5\n\nSigned-off-by: hydai <z54981220@gmail.com>",
          "timestamp": "2025-09-12T16:52:14+08:00",
          "tree_id": "aa05bc3519e09898c0644d582c3f99810796555d",
          "url": "https://github.com/hydai/lineguard/commit/2f9533ae440d2a7bb9962c2eee677d56cb965e15"
        },
        "date": 1757667233858,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0019006136640303374,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.006158519999115052,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.058226709675000016,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.06115040077999998,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07786350591538461,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d6cc99b85831992da740ec35a2b85699f161b211",
          "message": "chore: update dependencies (#63)\n\nUpdate all dependencies to latest compatible versions:\n- clap 4.5.47 -> 4.5.54\n- serde 1.0.219 -> 1.0.228\n- regex 1.11.2 -> 1.12.2\n- anyhow 1.0.99 -> 1.0.100\n- tempfile 3.22.0 -> 3.24.0\n- and many more transitive dependencies\n\nPin assert_cmd to ~2.0 to avoid breaking API changes in 2.1+ that\ndeprecate Command::cargo_bin() in favor of escargot crate.\n\n🤖 Generated with [Claude Code](https://claude.com/claude-code)\n\nCo-authored-by: Claude Opus 4.5 <noreply@anthropic.com>",
          "timestamp": "2026-01-05T23:59:58+08:00",
          "tree_id": "adadf4e90aa91cf9a1f39b8fc73c77776f7d51c7",
          "url": "https://github.com/hydai/lineguard/commit/d6cc99b85831992da740ec35a2b85699f161b211"
        },
        "date": 1767628872109,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.001897237027675763,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.005906066213913046,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.05766723399555556,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.061295674179999995,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07775337327333331,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "65d7d8b4b4d8959414260280d1a9c14d0b44c302",
          "message": "ci(deps): bump actions/checkout from 5 to 6 (#55)\n\nBumps [actions/checkout](https://github.com/actions/checkout) from 5 to 6.\n- [Release notes](https://github.com/actions/checkout/releases)\n- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/actions/checkout/compare/v5...v6)\n\n---\nupdated-dependencies:\n- dependency-name: actions/checkout\n  dependency-version: '6'\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-01-06T00:14:08+08:00",
          "tree_id": "2944fe1bf10795465d87b7384e62930b0dc1640d",
          "url": "https://github.com/hydai/lineguard/commit/65d7d8b4b4d8959414260280d1a9c14d0b44c302"
        },
        "date": 1767629715758,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0018944464567796574,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.00589227654424779,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.05643472695999999,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.06236930963636364,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07814480551282052,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bb5d60791cf6db5d1588b7cc8ccb1d229721e1d3",
          "message": "ci(deps): bump actions/cache from 4 to 5 (#56)\n\nBumps [actions/cache](https://github.com/actions/cache) from 4 to 5.\n- [Release notes](https://github.com/actions/cache/releases)\n- [Changelog](https://github.com/actions/cache/blob/main/RELEASES.md)\n- [Commits](https://github.com/actions/cache/compare/v4...v5)\n\n---\nupdated-dependencies:\n- dependency-name: actions/cache\n  dependency-version: '5'\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-01-06T00:18:01+08:00",
          "tree_id": "aa25e4b02a51fa701f4ccb97d9db62fc8aae12cb",
          "url": "https://github.com/hydai/lineguard/commit/bb5d60791cf6db5d1588b7cc8ccb1d229721e1d3"
        },
        "date": 1767629943044,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0013999153268750009,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.006156172042666666,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.05739225010792453,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.06014110820153846,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07569896640216218,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6542c13f28472b3b03f486901d541e55aa9d52e4",
          "message": "chore(deps): bump assert_cmd from 2.0 to 2.1.1 (#64)\n\n- Update assert_cmd dependency to 2.1.1\n- Replace deprecated Command::cargo_bin() with cargo_bin_cmd! macro\n- Replace deprecated cargo_bin() function with cargo_bin! macro\n\n🤖 Generated with [Claude Code](https://claude.com/claude-code)\n\nCo-authored-by: Claude Opus 4.5 <noreply@anthropic.com>",
          "timestamp": "2026-01-06T00:31:54+08:00",
          "tree_id": "a3a208dfd107b098707fda71d2c1392be24726d8",
          "url": "https://github.com/hydai/lineguard/commit/6542c13f28472b3b03f486901d541e55aa9d52e4"
        },
        "date": 1767630785827,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.00188661119730897,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.005879706526710233,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.05799146386222223,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.06274943811999999,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07762692897333333,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b1e4490f487c74f396215163d5e5c2cde744f29b",
          "message": "fix: normalize paths in is_ignored() to handle relative path prefixes (#62)\n\n- Normalize input paths by stripping ./ prefix and resolving .. components\n- Ensure consistent path matching regardless of how paths are specified\n- Use matches_path for filename-only patterns (safer than matches())\n- Add comprehensive tests for path normalization edge cases\n\n🤖 Generated with [Claude Code](https://claude.com/claude-code)\n\nCo-authored-by: Claude Opus 4.5 <noreply@anthropic.com>",
          "timestamp": "2026-01-06T05:54:52+08:00",
          "tree_id": "8ddab56e523f888689226a12b667c2e4e98f9bcd",
          "url": "https://github.com/hydai/lineguard/commit/b1e4490f487c74f396215163d5e5c2cde744f29b"
        },
        "date": 1767650165611,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0020755795441791017,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.007063242238190719,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.06359540764666669,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.06711587870608696,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.08371199202864865,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "z54981220@gmail.com",
            "name": "hydai",
            "username": "hydai"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7edbdef75f4c10e1b05d597ac39a37ec94774e61",
          "message": "chore(version): bump to v0.1.6 (#65)\n\n🤖 Generated with [Claude Code](https://claude.com/claude-code)\n\nCo-authored-by: Claude Opus 4.5 <noreply@anthropic.com>",
          "timestamp": "2026-01-06T06:03:38+08:00",
          "tree_id": "ac75877f4ec055e080ae6b1088897f96f4e2865a",
          "url": "https://github.com/hydai/lineguard/commit/7edbdef75f4c10e1b05d597ac39a37ec94774e61"
        },
        "date": 1767650685830,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0018900287232855926,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.00598038264048998,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.05691379754962963,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.06324970248818182,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.07849699285589745,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bff400918b5e4f1d480012bffc7ee84a80627e43",
          "message": "chore(deps): bump serde_json from 1.0.148 to 1.0.149 (#67)\n\nBumps [serde_json](https://github.com/serde-rs/json) from 1.0.148 to 1.0.149.\n- [Release notes](https://github.com/serde-rs/json/releases)\n- [Commits](https://github.com/serde-rs/json/compare/v1.0.148...v1.0.149)\n\n---\nupdated-dependencies:\n- dependency-name: serde_json\n  dependency-version: 1.0.149\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-01-25T15:04:00+08:00",
          "tree_id": "2f85888dffecf51784cff787dbf28f7c09393fb7",
          "url": "https://github.com/hydai/lineguard/commit/bff400918b5e4f1d480012bffc7ee84a80627e43"
        },
        "date": 1769324726533,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Small files (100x1KB)",
            "value": 0.0019032149821110117,
            "unit": "seconds"
          },
          {
            "name": "Medium files (100x100KB)",
            "value": 0.006134405662247194,
            "unit": "seconds"
          },
          {
            "name": "Large files (10x10MB)",
            "value": 0.05825888469921568,
            "unit": "seconds"
          },
          {
            "name": "Recursive scan",
            "value": 0.06498658766000001,
            "unit": "seconds"
          },
          {
            "name": "Glob pattern",
            "value": 0.08116061079157896,
            "unit": "seconds"
          }
        ]
      }
    ]
  }
}