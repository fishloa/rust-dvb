# dvb-tools

`dvb-tools` is a small command-line analyzer over the `rust-dvb` library
crates. It absorbs the former `si_dump` / `t2mi_dump` examples (now `dump` and
`t2mi`) plus a few small new utilities (`services`, `epg`, `pids`) under one
binary.

## Subcommands

```text
dvb-tools dump     <file.ts> [--json]                        # SI section dump
dvb-tools services <file.ts>                                # SDT + NIT/LCN tree
dvb-tools epg      <file.ts> [--json]                        # EIT schedule
dvb-tools pids     <file.ts>                                # PID table + bitrate
dvb-tools t2mi     <file> [--pid 0xNNN|raw] [--inner] [--plp N]
                                                          # T2-MI dump / inner-TS
```

## Usage

```console
$ cargo build -p dvb-tools --locked
$ cargo run -p dvb-tools --locked -- dump dvb-si/tests/fixtures/m6-single.ts
pid=0x0000 PROGRAM_ASSOCIATION v0 sn=0
...
-- packets=1264 sections=47 emitted=3 suppressed=44 crc_failures=0 malformed=0

$ cargo run -p dvb-tools --locked -- t2mi dvb-si/tests/fixtures/m6-single.ts --inner \
    > inner.ts && \
  cargo run -p dvb-tools --locked -- dump inner.ts
```
