### Ahother command line tool for managing your hosts file

- add new entry host
- remove host
- disable single host
- disable hosts with search
- import from URLs
- order all hosts into group

#### How to install
```bash
$ cargo install rhostman
```

#### Usage
```bash
$ rhostman add [-fqbcvq] [--force] [--path=PATH]
                ( [ENTRY ...] | [--input-file=FILE] | [--input-url=URL] )
$ rhostman remove [-qbcvq] ([--address=<address>] [--names=<names>]) [--path=PATH]
                 [--input-file=FILE] [--input-url=URL]
$ rhostman --version
```

#### Options
```bash
    -h --help                    show this help message and exit
    --version                    show version and exit
    --address=ADDRESS            ipv6 or ipv4 address
    --names=NAMES                host names
    -p --path=PATH               location of hosts file (attempts to detect default)
    -u --input-url=URL           url of file containing hosts to import
    -b --backup                  create a backup before each change
```

#### Examples
