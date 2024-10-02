# Biomarker Score Calculator

The biomarker scoring system is meant for quick assessment of how much existing knowledge has already been collected for an entity. Although a default scoring is provided, we recognize that different users will have different use cases, and as a result, will have different views on how various parameters in the scoring should be weighted. This tool allows for quick and portable calculation of biomarker scores and also supports custom scoring rules and weight overrides.

- [Usage](#usage)
- [Installation](#installation)
    - [Release Binary](#release-binary)
    - [Building From Source](#building-from-source)
- [Extensibility](#extensibility)
  - [Weights](#weights)
  - [Custom Rules](#custom-rules)
    - [Field](#field)
    - [Condition](#condition)
    - [Action](#action)
    - [Priority](#priority)

## Usage
```
Calculates biomarker scores based on input data and weight overrides

Usage: biomarker-score-calculator [OPTIONS]

Options:
  -d, --data <PATTERN>    Glob pattern for input files (e.g. `./data/*.json`) [default: ./data/*.json]
  -o, --overrides <FILE>  Optional JSON file for overriding scoring weights and other scoring conditions
  -m, --mode <MODE>       Run mode: 'map' to generate score map, 'overwrite' to update source files [default: map]
  -r, --rules <RULES>     Optional rules file for applying custom scoring logic
  -h, --help              Print help
  -V, --version           Print version
```

The `-d` or `--data` argument can be used to pass a custom glob pattern to look for the JSON data files. If not provided, it will default to looking at `./data/*.json`.

The `-o` or `--overrides` argument can be used to override the default scoring weights. See the [Weights](#weights) section.

The `-r` or `--rules` argument can be used to set custom scoring rules. See the [Custom Rules](#custom-rules) section.

The `m` or `--mode` command supports two different run modes:

1. `map` (default): Map mode will generate a mapping file of the different files and corresponding biomarker IDs. This approach has a reduced memory footprint and allows you to calculate custom scores while leaving the source data unaltered. The separate scores can be easily compared and mapped to the data later if needed. The resulting mapping file will generated with the name `biomarker_scores.json`. The mapping file will have top level keys of the source file names and within each file name object will be the corresponding biomarker IDs and their scoring data. For example:

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

## Extensibility

The biomarker scoring calculator was designed to be completely extensible and customizable without the need to alter the source code. There are two ways the default behaviour of the scoring calculator can be extended and altered.

### Weights

The available default weights that can be overwritten are:

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

If you would like to override the default scoring weights, you can create a JSON file that includes the conditions and overrides you would like to apply and pass it to the tool using the `-o` argument. The format of the overrides file should like like the following:

```json
{
  "first_pmid": 100,
  "other_pmid": 10
}
```

This override file will set the `first_pmid` condition to have a weight of `100` and any additional PubMed evidences will result an additional `10` points being added to the score. Overwriting the rest of the scoring conditions follows the same format.

### Custom Rules

The biomarker score calculator also supports a custom format for creating completely custom rules. This format is much more powerful than simply overriding the default weights using an overrides file. If both a rules file and an overrides file is provided, the overrides will be applied first and then the rules.

Using the `-r` argument you can pass the path to a JSON file which specifies your custom scoring rules. The format specification is as follows:

```json
{
  "rules": [
    {
      "name": "A short, descriptive name for the rule.",
      "condition": {
        "type": "The rule type.",
        "field": "The field to check the condition against",
        "value": "The value for the rule to be applied against."
      },
      "action": {
        "type": "The type of action to take when the rule is applied.",
        "value": "The value for the action."
      },
      "priority": "The action priority in case of rule conflict."
    }
  ]
}
```

The rules are specified in a JSON array with the `"rules"` key at the top level. Each individual rule is an object where the top level keys are `"name"`, `"condition"`, `"action"`, and `"priority"`.

#### Field

The `"field"` key in the condition object specifies which field to check the condition value against. The available fields that can be specified are:

- `BiomarkerID`: The `biomarker_id` value.
- `ComponentEvidenceSourceDatabase`: The `biomarker_component.evidence_source.database` values.
- `ConditionID`: The `condition.id` value.
- `TopEvidenceSourceDatabase`: The `evidence_source.database` values.
- `LoincCode`: The `biomarker_component.specimen.loinc_code` values.

#### Condition

The available conditions that are currently supported are:

- `NonPubmedEvidenceSourceMatch`: This condition allows for matching on non-pubmed evidence sources. For example a value of `"clinvar"` will evaluate to `true` if all the non-pubmed evidence source databases for the biomarker are from `clinvar`. Note the `NonPubmedEvidenceSourceMatch` condition does not require the `"field"` key as it will automatically check against the top level and component level evidence sources.
- `FieldEquals`: This condition allows for checking that a field value(s) equals a certain value. If using on a list field, it will only evaluate to `true` if all the values equal the specified value.
- `FieldAllContains`: This condition allows for checking that a field value(s) contains some substring. If using on a list field, it will only evaluate to `true` if all the values contain the specified value. Both `FieldAllContains` and `FieldSomeContains` are equivalent when using on an individual field.
- `FieldSomeContains`: This condition allows for checking that a field value(s) contains some substring. If using on a list field, it will only evaluate to `true` if any of the values contain the specified value. Both `FieldAllContains` and `FieldSomeContains` are equivalent when using on an individual field.
- `FieldLenGreaterThan`: This condition allows for checking a list field's length is greater than a certain value. Using this condition on non-list fields can have unintended consequences.
- `FieldLenLessThan `: This condition allows for checking a list field's length is less than a certain value. Using this condition on non-list fields can have unintended consequences.
- `FieldLenEqual`: This condition allows for checking a list field's length is equal to a certain value. Using this condition on non-list fields can have unintended consequences.
- `And`: This condition allows for chaining multiple conditions together in a logical AND fashion.
- `Or`: This condition allows for chaining multiple conditions together in a logical OR fashion.

#### Action

The available actions that are currently supported are:

- `SetScore`: Hardcode a score to a certain value if the condition is met.
- `AddToScore`: Add a value to the score if the condition is met.
- `MultiplyScore`: Multiple a value to the score if the condition is met.
- `SubtractScore`: Subtract a value from the score if the condition is met.
- `DivideScore`: Divide the score by a value if the condition is met.

#### Priority

The priority is an integer value that specifies the priority to apply the conditions if multiple rule conditions are met.
