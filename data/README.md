# Data Directory

This directory houses data that is ingested by taxonate at compile time.

## Updating the Supported Languages

### Prerequisites

You'll need the following software installed on your machine in order to develop
and submit changes to the languages supported by taxonate:

- [CUE](https://cuelang.org/)
- [Prettier](https://prettier.io/)

### Overview

The programming languages that taxonate is able to detect are defined by data in
the _languages.json_ file, which **should not be edited manually**!

To ensure consistency and check for potential conflicts, the language
definitions should be written using CUE (Configure Unify Execute) within the
_languages.cue_ file. Once you've made an edit to the language definitions, you
can evaluate the configuration and export the resulting JSON by running the
custom CUE command:

    $ cue dump

This will emit a pretty version of the JSON data to STDOUT, and will also save a
minified copy, overwriting the existing _languages.json_ file.

To verify that the _languages.cue_ and _languages.json_ files are in sync, you
can validate that they match and adhere to the defined constraints by running:

    $ cue vet languages.json languages.cue --schema output.languages

### Schema

A language definition has four parts that can be seen in this example:

```json
"python": {
  "name": "Python",
  "globs": [
    "*.py",
    "*.pyw"
  ],
  "interpreters": [
    "python",
    "python2",
    "python3"
  ]
}
```

1. `key` is the map key used to identify a language and is what end users will
   specify via the `--language LANGUAGE` command line option. The key must be a
   valid [CUE identifier] and will end up being lowercased in the final JSON
   output.

2. `name` is the language's human friendly display name and should be
   capitalized accordingly, as a proper noun.

3. `globs` is an array containing the common pattern(s) that will match
   filenames belonging to the language.

4. `interpreters` is an array containing the language's executable program
   name(s) that would be specified in a script's [shebang] line as part of the
   interpreter directive.

> NOTE: `globs` and `interpreters` are considered to be filetype "markers" used
> for identification; a minimum of one marker is required for each language
> definition.

[cue identifier]: https://cuelang.org/docs/references/spec/#identifiers
[shebang]: https://en.wikipedia.org/wiki/Shebang_(Unix)
