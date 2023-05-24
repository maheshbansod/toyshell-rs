# toyshell

A toyshell that is a re-implementation of the toyshell project in C (https://github.com/maheshbansod/toyshell)  
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

### ChangeDirectory
Change the current working directory

### Exit
Exit the shell

