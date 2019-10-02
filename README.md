# Git ID

Manage emails and names for git.

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
