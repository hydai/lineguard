window.BENCHMARK_DATA = {
  "lastUpdate": 1752275636851,
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
      }
    ]
  }
}