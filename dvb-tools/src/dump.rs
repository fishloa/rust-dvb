//! `dvb-tools dump <file.ts> [--json]` — SI section dump.
//!
//! Drives [`SiDemux`] over an aligned 188-byte `.ts` capture and prints one
//! line per emitted (changed) section. With `--json`, the decoded typed table
//! for each section is pretty-printed via `serde_json`.
//!
//! Behaviour is identical to the former `dvb-si/examples/si_dump.rs`.
use std::process::ExitCode;

use dvb_si::demux::SiDemux;
use dvb_si::tables::AnyTableSection;

use crate::util::{for_each_packet, read_file};

/// Label for an `AnyTableSection` — the macro-generated `name()`, with the
/// `table_id` appended for unknowns.
fn table_name(table: &AnyTableSection<'_>) -> String {
    match table {
        AnyTableSection::Unknown { table_id, .. } => format!("UNKNOWN(0x{table_id:02X})"),
        t => t.name().to_string(),
    }
}

/// `dvb-tools dump <file.ts> [--json]` — returns success on a clean run.
pub fn run(args: &[String]) -> ExitCode {
    let mut path: Option<String> = None;
    let mut json = false;
    for arg in args {
        match arg.as_str() {
            "--json" => json = true,
            "-h" | "--help" => {
                eprintln!("usage: dvb-tools dump <file.ts> [--json]");
                return ExitCode::SUCCESS;
            }
            other if other.starts_with('-') => {
                eprintln!("dvb-tools dump: unknown option {other}");
                return ExitCode::FAILURE;
            }
            other => path = Some(other.to_string()),
        }
    }

    let Some(path) = path else {
        eprintln!("usage: dvb-tools dump <file.ts> [--json]");
        return ExitCode::FAILURE;
    };

    let data = match read_file(&path, "dvb-tools dump") {
        Ok(d) => d,
        Err(code) => return code,
    };

    let mut demux = SiDemux::builder().build();
    for packet in for_each_packet(&data) {
        for event in demux.feed(&packet) {
            match event.table_section() {
                Ok(table) => {
                    if json {
                        match serde_json::to_string_pretty(&table) {
                            Ok(s) => println!("{s}"),
                            Err(e) => {
                                eprintln!("dvb-tools dump: serialize {}: {e}", event.pid());
                            }
                        }
                    } else {
                        let name = table_name(&table);
                        match event.version() {
                            Some(v) => println!(
                                "pid={} {name} v{v} sn={}",
                                event.pid(),
                                event.section_number().unwrap_or(0)
                            ),
                            None => println!("pid={} {name}", event.pid()),
                        }
                        match &table {
                            AnyTableSection::PmtSection(pmt) => {
                                for st in &pmt.streams {
                                    println!(
                                        "    es pid=0x{:04X} stream_type={}",
                                        st.elementary_pid,
                                        st.stream_type.name()
                                    );
                                }
                            }
                            AnyTableSection::SdtSection(sdt) => {
                                for s in &sdt.services {
                                    println!(
                                        "    service 0x{:04X} running_status={}",
                                        s.service_id,
                                        s.running_status.name()
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Err(e) => eprintln!("pid={} parse error: {e}", event.pid()),
            }
        }
    }

    let s = demux.stats();
    eprintln!(
        "-- packets={} sections={} emitted={} suppressed={} crc_failures={} malformed={}",
        s.packets,
        s.sections_completed,
        s.emitted,
        s.suppressed,
        s.crc_failures,
        s.malformed_packets
    );
    ExitCode::SUCCESS
}
