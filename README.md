### Ahother command line tool for managing your hosts file
[![CircleCI](https://circleci.com/gh/damjack/rhostman/tree/main.svg?style=svg)](https://circleci.com/gh/damjack/rhostman/tree/main)

- add/remove new entry host
- disable single line
- import RAW data file from URLs

#### How to install
```bash
$ cargo install rhostman
```

#### Usage
```bash
$ rhostman add <HOSTS>
$ rhostman backup <BACKUP_FILE>
$ rhostman disable <HOST>
$ rhostman import <URL>
$ rhostman remove <HOST>
```

#### Options
```bash
    -h --help                    show this help message and exit
    --version                    show version and exit
    -p --path=PATH               location of hosts file (attempts to detect default)
```
