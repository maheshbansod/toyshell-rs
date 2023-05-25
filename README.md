# toyshell

A toyshell that is a re-implementation of the toyshell project which was originally in C (https://github.com/maheshbansod/toyshell)  
I'm gonna try to implement all the pre-defined commands that are in there as close as possible I guess. Or maybe not. idk.

## Built-in commands

### List
The list command is used to list files.
Syntax:
```
list [option] [...directories]
```
| Option | Description | Symbol |
| :---   |  :---:      | :---:  |
| All | (default) list all files and directories | a |
| Files | list only files | f |
| Count | Count files/directories | c |

Example:
```
$ list f . /
```

### Typeline
This command lists the contents of a file with line numbers.
Syntax:
```
typeline [a|+<n>|-<n>] <filename>
```
| Option | Description | Symbol |
| :---   |  :---:      | :---:  |
| All | Show all lines | a |
| FromStart {n} | `n` lines from the beginning | +<n> |
| FromEnd {n} | `n` lines from the end | -<n> |

Example:
```
$ typeline +4 src/main.rs
```

### Count
This command counts the number of words, characters, or lines in a file.
Syntax
```
count [c|w|l] <filename>
```
| Option | Description | Symbol |
| :---   |  :---:      | :---:  |
| Characters | Count characters | c |
| Words | Count words | w |
| Lines | Count lines | l |

Example:
```
$ count l src/main.rs
```

### Search
This command searches for a pattern within a file  

Syntax
```
search <f|a|c> <filepath> <pattern>
```
| Option | Description | Symbol |
| :---   |  :---:      | :---:  |
| First | First occurrence | f |
| All | All occurrences | a |
| Count | Count occurrences | c |

Example:
```
$ search a src/main.rs let
```

### ChangeDirectory
Change the current working directory

### Exit
Exit the shell

