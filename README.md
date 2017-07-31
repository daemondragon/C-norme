# Goal

The goal of this program is to create a tool that will check EPITA C norme.
The non respect of this will result with a 0 mark, so having an automated tool to check it
is really useful.

The respect of this goal mean that their must not have **ANY** false negative (No errors were found where it should have):
if a goal passe all test it respect the rule. That mean that this norme-checker is allowed to have some false positive
(The checker detect an error when it shouldn't).

Some [norme] rules might depend on more than one [checker] rule to work.
Some [checker] rules might depend on another to detect all errors.

An extract of this norme can be found [here](http://tsunanet.net/~tsuna/codingstyle/codingstyle.pdf).
*Note that this is an old norme, and new rules might have been added*

## Features

* Usable in command line.
* Check all content of a directory at once, or only some file.
* Pointing the exact place where errors happened.
* Checking a file will display all current errors, it doesn't stop at the first rule that found errors.

## What will **NOT** be done

* Detecting if a file have been included more than once. In this case, it will be processed has many time as mentionned.
* Detecting if code langage is in english and no other langage is used.
* Detecting unused function (use your compiler).
* Detecting if composite names are separated with underscores.

## Global progression

- [x] Basic rule trait.
- [x] Reading a file and checking its norme.
- [x] Reading all contents of a directory.
- [ ] Multiprocessing rules check (useful for big project).
- [ ] Correcting file to match norme.

## Rule implementation progression

### Global
- [x] No more than 80 characters per line (including newline character).
- [x] Space instead of tab.
- [x] No trailing whitespace.
- [x] Closing brace must appear on the same column at the corresponding opening brace.
- [x] The text between brace must be indented by 4 spaces (could be 2, but 4 is choosen for personnal preference).
- [x] All braces must be on their own line.
- [x] The goto statement must not be used.

### Comment
- [x] Multiline comments delimiters must appear on their on line.
- [x] Multiline comments intermediary line must start with **

### Preprocessor and macro
- [x] Preprocessor directive must appear on the first column.
- [x] Preprocessor directives following '#if' and '#ifdef' must be indented by one character.
- [x] '#else' and 'endif' must be followed by a comment describing the corresponding initial condition.
- [x] When macro must span over multiple lines, escape line break ('\\') must be aligned.
- [x] Macro names must be entirely captitalized.
- [x] Macro arguments must be capitalized.
- [x] All #include directive must appear at the start of the file.
- [x] System header must appear before local one. In header (.h) file only.
- [x] Headers must be protected against multiple inclusions. *See official norme for more info*.

### Functions
- [ ] Function's body must not contain more than 25 lines (excluding comments and blank line).
- [ ] Function's subpart have to be separated one blank line maximum (No two following blank line, excluding comments).
- [ ] Function must not have more than four arguments.
- [ ] There must be at most five exported functions per source file.
- [ ] There must be at most ten function per source file.

### Enumeration
- [ ] Enumerations values must be entirely capitalized.
- [ ] Enumerations values must appear on their own line.

### Naming convention
- [ ] Global variables must start with 'g_'.

### Typedef
- [ ] Structures names must be prefixed by 's_'.
- [ ] Unions names must be prefixed by 'u_'.
- [ ] Enumerations names must be prefixed by 'e_'.
- [ ] If the type already have 'e_', 'f_', 's_', 't_', 'u_' as prefix, it has to be keep.

More rules to be added later...
