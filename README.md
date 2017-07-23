#Goal

The goal of this program is to create a tool that will check EPITA C norme.
The non respect of this will result with a 0 mark, so having an automated tool to check it
is really useful.

An extract of this norme can be found [here](http://tsunanet.net/~tsuna/codingstyle/codingstyle.pdf).

#Features

* Usable in command line.
* Check all content of a directory at once, or only some file.
* Pointing the exact place where errors happened.


#What will **NOT** be done

* Detecting if a file have been included more than once. In this case, it will be processed has many
	time as mentionned.
* Detecting if code langage is in english and no other langage is used.
* Detecting unused function (use your compiler).
* Detecting if composite names are seprated with underscores.

#Global progression

- [ ] Basic rule trait.
- [ ] Reading a file and checking its norme.
- [ ] Reading all contents of a directory.

#Rule implementation progression

###Global
- [ ] No more than 80 characters per line (including newline character).
- [ ] Space instead of tab.
- [ ] No trailing whitespace.

###Macro
- [ ] Macro names must be entirely captitalized.
- [ ] Macro arguments must be capitalized.
