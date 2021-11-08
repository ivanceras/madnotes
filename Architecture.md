# Architecture

Uses markdown as the umbrella text. All the other content such as scripts will be implemented
as a markdown plugin.

# Depedencies
Uses `ultron` for the code editor.
Uses `ipfs` for storing, referencing and retrieving of files
Uses `rune` scripting language for processing the files
Uses `datafusion` for processing raw csv files to aggregate and compute data visualization input.

# Functionality
- Publishing madnotes will also published the files it referenced.
    For security requirement, all files that is being used and referenced in a madnotes should
    reside inside the workspace of the madnotes.

# Feature
- Security
    - scripts on madnotes should not be run agains a local files, the files should exist
     on a universally accessible location such as ipfs
