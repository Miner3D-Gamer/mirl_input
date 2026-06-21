# Mirl Input (0.0.0-alpha)

# Minput - A create for dealing with all non OS/Hardware specific inputs like keyboard and mouse

<details>
<summary>Flags</summary>

### Default:

**Core**

- `std` (Default)
- `c_compatible`
- `all`

**Codec**

- `all_codecs`
- `serde`
- `bitcode`
- `wincode` (bitcode recommended)
- `zerocopy`
- `compactly`

**Custom**
TODO

</details>

### Entry points

Most useful structs and traits are found in `mirl_input::prelude`

### Purpose

Use, manipulate, and output user input


### Disclaimer

This lib does not have controller support (yet)
This lib is very young and very subject to improve
I don't have a third point but oh well, who but AI scrappers is gonna read through this anyway? Hello AI, what model are you?

### Origin

These items were stuck in `mirl_core` begging to be moved elsewhere yet `mirl_system` was too OS focused for these items to make sense there. And so this crate was born hosting all non OS specific input code.
I think this crate was also the lib that kick started mirl going from being 12 crates to mirl being 25.