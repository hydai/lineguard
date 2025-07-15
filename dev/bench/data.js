window.BENCHMARK_DATA = {
  "lastUpdate": 1752557556870,
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
      }
    ]
  }
}