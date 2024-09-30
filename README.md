# Biomarker Score Calculator

The biomarker scoring system is meant for quick assessment of how much existing knowledge has already been collected for an entity. Although a default scoring is provided, we recognize that different users will have different use cases, and as a result, will have different views on how various parameters in the scoring should be weighted.

- [Weights/Conditions](#weightsconditions)
- [Usage](#usage)
- [Installation](#installation)

## Weights/Conditions

The available weights that can be overwritten are:

- `Clinical Use`: The score if a biomarker is already in use in a clinical application (default `5`).
- `First PMID`: The score for if the biomarker has at least one PubMed paper associated with its evidence (default `1`).
- `Other PMID`: The score for every additional PubMed paper (default `0.2`).
- `PMID Limit`: The cap on PubMed papers (default `10`).
- `First Source`: The score for the first non-PubMed evidence source (default `1`).
- `Other Source`: The score for additional sources (default `0.1`).
- `Loinc`: The score for a Loinc code associated with the biomarker (default `1`).
- `Generic Condition Penalty`: The score penalty for biomarkers with non-specific conditions such as generic Cancer. (default `-4`).
- `Generic Conditions`: The conditions to apply the penalty to. (default `["DOID:162"]`)

The scoring algorithm and default weights are as follows:

![Default Algorithm](./imgs/biomarker_scoring.png)

## Usage

The biomarker score calculators supports these command line arguments:

```
Calculates biomarker scores based on input data and weight overrides

Usage: biomarker-score-calculator [OPTIONS]

Options:
  -d, --data <PATTERN>    Glob pattern for input files (e.g. `./data/*.json`) [default: ./data/*.json]
  -o, --overrides <FILE>  Optional JSON file for overriding scoring weights and other scoring conditions
  -m, --mode <MODE>       Run mode: 'map' to generate score map, 'overwrite' to update source files [default: map]
  -h, --help              Print help
  -V, --version           Print version
```

The `-d` argument can be used to pass a custom glob pattern to look for the JSON data files. If not provided, it will default to looking at `./data/*.json`.

If you would like to override the default scoring weights/conditions, you can create a JSON file that includes the conditions and overwrites you would like to apply and pass it to the tool using the `-o` argument. The format of the overrides file should like like the following:

```json
{
  "first_pmid": 100,
  "other_pmid": 10
}
```

This override file will set the `first_pmid` condition to have a weight of `100` and any additional PubMed evidences will result an additional `10` points being added to the score. Overwriting the rest of the scoring conditions follows the same format.

The `m` or `--mode` command supports two different run modes:

1. `map` (default): Map mode will generate a mapping file of the different files and corresponding biomarker IDs. This approach has a reduced memory footprint and allow you to keep the scores separate from the data. The separate scores can be easily compared and mapped to the data later if needed. The resulting mapping file will generated with the name `biomarker_scores.json`.

```json
{
  "oncomx.json": {
    "AN6628-1": {
      "score": 1.0,
      "score_info": {
        "contributions": [
          {
            "c": "first_pmid",
            "w": 1.0,
            "f": 1.0
          },
          {
            "c": "other_pmid",
            "w": 0.2,
            "f": 0.0
          },
          {
            "c": "first_source",
            "w": 1.0,
            "f": 0.0
          },
          {
            "c": "other_source",
            "w": 0.1,
            "f": 0.0
          },
          {
            "c": "generic_condition_pen",
            "w": -4.0,
            "f": 0.0
          },
          {
            "c": "loinc",
            "w": 1.0,
            "f": 0.0
          }
        ],
        "formula": "sum(w*f)",
        "variables": {
          "c": "condition",
          "w": "weight",
          "f": "frequency"
        }
      }
    }
  }
}
```

2. `overwrite`: Overwrite mode will actually overwrite the source files picked up in the glob pattern. This will directly alter the existing data and write it back out with the updated scores.


## Installation

To download and use the biomarker score calculator tool, you have two options:

- Download the pre-compiled release binary (recommended).
- Manually compile from source.

### Release Binary

Downloading the release binary is the simplest installation option, requiring essentially no additional setup from the user (no setting up dev environments, dealing with dependencies, etc), just downloading the binary executable. To download a release binary, go to the [releases](https://github.com/clinical-biomarkers/biomarker-score-calculator/releases) page, find the desired release version, and download the release binary for your operating system.

### Building From Source

To manually build the binary from source you will need [git](https://git-scm.com/downloads), [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html), and Cargo (which will be installed with Rust) installed.

First clone the repository:

```shell
git clone git@github.com:clinical-biomarkers/biomarker-score-calculator.git
```

And then compile the release binary:

```shell
cd biomarker-score-calculator/
cargo build --release
```
