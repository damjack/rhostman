### Another command line tool for managing your hosts file
[![CircleCI](https://circleci.com/gh/damjack/rhostman/tree/main.svg?style=svg)](https://circleci.com/gh/damjack/rhostman/tree/main)

- add/remove/disable entries by exact host, or in bulk by domain
- import a raw hosts-format file from a URL
- track a remote raw hosts-format file by name, and update/remove just its block later

#### How to install
```bash
$ cargo install rhostman
```

No local Rust toolchain is required to build or test this project: everything runs through Docker.
```bash
$ docker compose run --rm cli cargo build
$ docker compose run --rm cli cargo test
```

#### Usage
```bash
$ rhostman add <IP> <HOSTS>... [-c COMMENT]
$ rhostman remove (--host <HOST> | --domain <DOMAIN>)
$ rhostman disable (--host <HOST> | --domain <DOMAIN>)
$ rhostman import <URL>
$ rhostman backup <OUTPUT>
$ rhostman track add <NAME> <URL>
$ rhostman track update [NAME]
$ rhostman track remove <NAME>
$ rhostman track list
```

`--host` matches a single entry by its exact IP or hostname. `--domain` matches every entry whose
hostname equals the given domain, or is a subdomain of it.

`import` fetches a raw hosts-format file once and merges its entries in; it doesn't remember where
they came from. `track add` does the same fetch, but also remembers the source under `NAME` and
wraps its entries in a marker block, so `track update`/`track remove` can later refresh or delete
just that block without touching anything else. Domains from a tracked source are written as
`0.0.0.0 <domain>` entries.

#### Options
```bash
    -h --help                    show this help message and exit
    --version                    show version and exit
    -p --path=PATH               location of the hosts file (default: /etc/hosts)
```

`track` subcommands also accept `--config=PATH` (or the `RHOSTMAN_CONFIG` environment variable) to
override the tracked-sources config file. By default, this is `/etc/rhostman/sources.toml` when
running as root (e.g. under `sudo`, since editing `/etc/hosts` normally requires it), or
`~/.config/rhostman/sources.toml` otherwise. If a tracked source can't be found, check whether it
was registered under the other of these two locations.
