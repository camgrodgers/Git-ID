# Git ID

Manage emails and names for git.

### Summary

This is a CLI tool for storing the git identity info user.name and user.email, and setting the info in repos individually.
It should be of some use to programmers who need to manage multiple identities across multiple git repos.
From a security point of view, it has the advantage of making it easier to manage multiple identities,
 but the drawback of storing them all in a single file, which will allow anyone who reads it to 
 infer that all of the identities in the file are the same person.

Gitid stores ids in a .gitid dotfile in the home directory.

##### Example:
```
[u gitid]$ gitid list
Git IDs:
0. Email: bobby@humans.com
   Name : Bobby
[u gitid]$ gitid add -e lizzie@lizards.net -n Lizzie
Git IDs:
0. Email: bobby@humans.com
   Name : Bobby
1. Email: lizzie@lizards.net
   Name : Lizzie
[u gitid]$ gitid set 0 
[u gitid]$ gitid remove 1
Git IDs:
0. Email: bobby@humans.com
   Name : Bobby
```

### Installation

```
cargo install gitid
```

### Usage
```
USAGE:
    gitid [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       Add an email and name to the .gitid dotfile.
    help      Prints this message or the help of the given subcommand(s)
    list      Lists the user.emails and user.names stored in the .gitid dotfile.
    remove    Remove the specified user id.
    set       Set the user.email and user.name of the current git repo to the one specified. EG: gitid -s 3

---

	gitid-add 
Add an email and name to the .gitid dotfile.

USAGE:
    gitid add --email <email> --name <name>

---

	gitid-remove
Remove the specified user id.

USAGE:
    gitid remove <number>

ARGS:
    <number>

---

gitid-set
Set the user.email and user.name of the current git repo to the one specified. EG: gitid -s 3

USAGE:
    gitid set <number>

ARGS:
    <number>
```
