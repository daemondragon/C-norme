# Goal

The goal of this program is to create a tool that will check EPITA C norme.
The non respect of this will result with a 0 mark, so having an automated tool to check it
is really useful.

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
- [ ] Reading all contents of a directory.
- [ ] Multiprocessing rules check (useful for big project).
- [ ] Correcting file to match norme.

## Rule implementation progression

### Global
- [x] No more than 80 characters per line (including newline character).
- [ ] Space instead of tab.
- [ ] No trailing whitespace.
- [ ] Multiline comments delimiters must appear on their on line.

### Preprocessor and macro
- [ ] Preprocessor directive must appear on the first column.
- [ ] Preprocessor directives following '#if' and '#ifdef' must be indented by one character.
- [ ] '#else' and 'endif' must be followed by a comment describing the corresponding initial condition.
- [ ] When macro must span over multiple lines, escape line break ('\') must be aligned.
- [ ] Macro names must be entirely captitalized.
- [ ] Macro arguments must be capitalized.

More rules to be added later...
