# Migrating `dvb-si` 3.x -> 4.0

4.0 is a deliberate API break that separates one wire section from a complete
logical table.

In 3.x, many public names used the table name (`Nit`, `Sdt`, `Sit`, `AnyTable`)
even though the parser returned one PSI/SI section. In 4.0, section parsers are
named `*Section`, the dynamic dispatcher is `AnyTableSection`, and the
`collect` module builds complete multi-section tables.

There are no compatibility aliases. Update imports and pattern matches directly.

## 1. Section parser types are now `*Section`

Every one-section parser uses a `Section` suffix and Rust CamelCase acronyms.

```rust
// 3.x
use dvb_si::tables::nit::Nit;
use dvb_si::tables::sdt::Sdt;
use dvb_si::tables::sit::Sit;

let nit = Nit::parse(section_bytes)?;

// 4.0
use dvb_si::tables::nit::NitSection;
use dvb_si::tables::sdt::SdtSection;
use dvb_si::tables::sit::SitSection;

let nit = NitSection::parse(section_bytes)?;
```

Common renames:

| 3.x | 4.0 |
|---|---|
| `Ait` | `AitSection` |
| `Bat` | `BatSection` |
| `Cat` | `CatSection` |
| `Cit` | `CitSection` |
| `Container` | `ContainerSection` |
| `Dit` | `DitSection` |
| `Eit` | `EitSection` |
| `Int` | `IntSection` |
| `MpeFec` | `MpeFecSection` |
| `MpeIfec` | `MpeIfecSection` |
| `Nit` | `NitSection` |
| `Pat` | `PatSection` |
| `Pmt` | `PmtSection` |
| `Rct` | `RctSection` |
| `Rnt` | `RntSection` |
| `Rst` | `RstSection` |
| `Sat` | `SatSection` |
| `Sdt` | `SdtSection` |
| `Sit` | `SitSection` |
| `St` | `StSection` |
| `Tdt` | `TdtSection` |
| `Tot` | `TotSection` |
| `Tsdt` | `TsdtSection` |
| `Unt` | `UntSection` |

`MpeDatagramSection`, `DsmccSection`, `ProtectionMessageSection`, and
`DownloadableFontInfoSection` already had section-shaped names.

## 2. `AnyTable` is now `AnyTableSection`

The dynamic dispatcher parses exactly one complete section. Rename the enum and
its variants.

```rust
// 3.x
use dvb_si::tables::AnyTable;

match AnyTable::parse(section_bytes)? {
    AnyTable::Sdt(sdt) => { /* one SDT section */ }
    AnyTable::Unknown { table_id, raw } => { /* unchanged */ }
    _ => {}
}

// 4.0
use dvb_si::tables::AnyTableSection;

match AnyTableSection::parse(section_bytes)? {
    AnyTableSection::SdtSection(sdt) => { /* one SDT section */ }
    AnyTableSection::Unknown { table_id, raw } => { /* unchanged */ }
    _ => {}
}
```

`parse_as` moved with the enum:

```rust
let mpe = AnyTableSection::parse_as::<MpeDatagramSection>(section_bytes)?;
```

If you serialize the dispatcher enum with serde, variant keys now include the
section suffix. For example, `AnyTableSection::PatSection` serializes as
`{"patSection": ...}`.

## 3. `SectionEvent::table()` is now `table_section()`

Demux still emits changed sections, not complete logical tables.

```rust
// 3.x
if let Ok(AnyTable::Pat(pat)) = event.table() {
    let _ = pat;
}

// 4.0
if let Ok(AnyTableSection::PatSection(pat)) = event.table_section() {
    let _ = pat;
}
```

Feed `event.bytes()` to a collector when you need the complete logical table.

## 4. Multi-section tables: use `collect`

Every long-form table with `section_number` and `last_section_number` can be
assembled with `SectionSetCollector`. The collector owns the original section
bytes so parsed views can keep borrowing from them. It validates the long-form
section CRC before retaining bytes.

```rust
use dvb_si::collect::SectionSetCollector;
use dvb_si::tables::pat::PatSection;

let mut collector = SectionSetCollector::new();

if let Some(complete) = collector.push_section(section_bytes)? {
    let pat = complete.table::<PatSection>()?;

    for section in pat.sections() {
        for entry in &section.entries {
            println!("program {} -> pid {}", entry.program_number, entry.pid);
        }
    }
}
```

When the PID is part of your stream identity, include it in the key:

```rust
if let Some(complete) =
    collector.push_section_with_pid(Some(event.pid().value()), event.bytes())?
{
    let sections = complete.table::<NitSection>()?;
    let _ = sections;
}
```

The generic complete view works for all long-form PSI/SI section parsers. It
does not invent a synthetic one-section table; it preserves the actual section
sequence in section-number order.

In a demux loop, feed the event bytes after matching the section's `table_id`.
Collector errors are section-scoped: log/drop that input section and keep
feeding later sections unless your application chooses strict failure.

```rust
use dvb_si::collect::SectionSetCollector;
use dvb_si::tables::nit::{self, NitSection};

let mut nits = SectionSetCollector::new();

for event in demux.feed(&packet) {
    match event.table_id() {
        nit::TABLE_ID_ACTUAL | nit::TABLE_ID_OTHER => {
            let collected = match nits.push_section_with_pid(
                Some(event.pid().value()),
                event.bytes(),
            ) {
                Ok(collected) => collected,
                Err(error) => {
                    eprintln!("dropping bad SI section: {error}");
                    continue;
                }
            };
            if let Some(complete) = collected {
                let logical = complete.nit()?;
                let sections = complete.table::<NitSection>()?;
                let _ = (logical, sections);
            }
        }
        _ => {}
    }
}
```

## 5. Complete logical table helpers

Some tables are more useful after flattening entries across all sections. 4.0
adds helpers for those logical views:

```rust
if let Some(complete) = collector.push_section(section_bytes)? {
    let nit = complete.nit()?;

    for ts in &nit.transport_streams {
        for descriptor in ts.descriptors.descriptors() {
            println!("{descriptor:?}");
        }
    }
}
```

Available helper views:

| Helper | Output |
|---|---|
| `CompleteSectionSet::nit()` | `CompleteNit` |
| `CompleteSectionSet::bat()` | `CompleteBat` |
| `CompleteSectionSet::sdt()` | `CompleteSdt` |
| `CompleteSectionSet::eit()` | `CompleteEit` for one EIT table_id |

Each helper has a `_with_registry` variant for private descriptor registries.

Descriptor loops in complete views use `ParsedDescriptorLoop`:

```rust
let typed = service.descriptors.descriptors(); // &[Result<AnyDescriptor>]
let raw = service.descriptors.raw();           // original DescriptorLoop
```

This keeps descriptors real and typed without losing access to the original
wire bytes.

## 6. EIT schedule collection spans `last_table_id`

EIT schedule is not complete when one table_id has all its sections. It is
complete when every schedule table_id through `last_table_id` has completed.
Use `EitCollector` for all EIT sections. EIT schedule sub-tables version
independently, so `CompleteEitSchedule` exposes per-table versions rather than
a single schedule-wide version.

```rust
use dvb_si::collect::{CompletedEit, EitCollector};

let mut eits = EitCollector::new();

let collected = match eits.push_section_with_pid(Some(event.pid().value()), event.bytes()) {
    Ok(collected) => collected,
    Err(error) => {
        eprintln!("dropping bad EIT section: {error}");
        None
    }
};
if let Some(done) = collected {
    match done {
        CompletedEit::PresentFollowing(set) => {
            let tables = set.eit()?;
            let _ = tables;
        }
        CompletedEit::Schedule(schedule) => {
            for (table_id, version) in schedule.table_versions() {
                println!("EIT schedule 0x{table_id:02X} version {version}");
            }
            for table in schedule.tables()? {
                println!("EIT schedule table_id 0x{:02X}", table.table_id);
            }
        }
    }
}
```

For long-running EPG collection, prune state on your own retention boundary:

```rust
eits.retain_logical(|key| key.transport_stream_id == current_tsid);
// or drop everything:
eits.clear();
```

## 7. `TableId` variants use CamelCase

`TableId` is still the typed enum for raw `table_id` byte values. Its variants
now follow Rust CamelCase and do not use the `Section` suffix.

```rust
// 3.x
TableId::PAT;
TableId::MPE_FEC;

// 4.0
TableId::Pat;
TableId::MpeFec;
```

Long descriptive variants were already close to the new style and remain
semantic names:

```rust
TableId::NetworkInformationActual;
TableId::ServiceDescriptionOther;
TableId::EventInformationPfActual;
```

This keeps the byte-value enum distinct from parser types such as
`PatSection` and `NitSection`.

## 8. Serialization boundary

`*Section` types still implement `Serialize` to their one-section wire format.
Complete logical views do not serialize to a fabricated single section. Keep
the collected `CompleteSectionSet` if you need the original section bytes:

```rust
for bytes in complete.section_bytes() {
    writer.write_all(bytes)?;
}
```

## Checklist

1. Replace parser imports (`Nit` -> `NitSection`, `Sit` -> `SitSection`, etc.).
2. Replace `AnyTable` with `AnyTableSection` and update variant names.
3. Replace `event.table()` with `event.table_section()`.
4. Replace all-caps `TableId` variants with CamelCase variants.
5. Add `SectionSetCollector` anywhere code previously assumed one section was
   the complete logical table.
6. Route EIT sections through `EitCollector` if schedule completeness matters.

The 3.1 guide still applies for older code that has not yet moved to
`DescriptorLoop`, Serialize-only serde, typed SIT service loops, or `yoke`.
See [MIGRATION-3.1.md](MIGRATION-3.1.md) first if you are upgrading from 1.x or
2.x.
