use std::env;
use std::fs;

const ELF_MAGIC: &[u8] = b"\x7fELF";

const SIG_PAYLOAD_BEGIN: &[u8] = b"VIRUS_PAYLOAD_BEGIN_MARKER";
const SIG_SUFFIX: &[u8] = b"fdgffdgsdhgtrhtshs";
const SIG_WELCOME: &[u8] = b"Welcome to HNU CSAPP!";
const SIG_INFECT_FN: &[u8] = b"infect_fdgffdgsdhgtrhtshs";

fn usage() {
    eprintln!(
        "USAGE: {} [--verbose] <path>\n",
        env::args().next().unwrap_or_else(|| "check_hnu_csapp_infection".to_string())
    );
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || haystack.len() < needle.len() {
        return false;
    }
    haystack.windows(needle.len()).any(|w| w == needle)
}

fn is_elf(data: &[u8]) -> bool {
    data.len() >= 4 && &data[..4] == ELF_MAGIC
}

#[derive(Default, Clone)]
struct Signals {
    elf: bool,
    r1_payload_marker: bool,
    r2_suffix_and_welcome: bool,
    r3_suffix_and_infect: bool,
}

impl Signals {
    fn collect(data: &[u8]) -> Self {
        let mut s = Signals::default();
        s.elf = is_elf(data);
        if !s.elf {
            return s;
        }
        s.r1_payload_marker = contains(data, SIG_PAYLOAD_BEGIN);
        let has_suffix = contains(data, SIG_SUFFIX);
        s.r2_suffix_and_welcome = has_suffix && contains(data, SIG_WELCOME);
        s.r3_suffix_and_infect = has_suffix && contains(data, SIG_INFECT_FN);
        s
    }

    fn infected(&self) -> bool {
        self.elf && (self.r1_payload_marker || self.r2_suffix_and_welcome || self.r3_suffix_and_infect)
    }
}

fn main() {
    let mut verbose = false;
    let mut path: Option<String> = None;
    for a in env::args().skip(1) {
        match a.as_str() {
            "-v" | "--verbose" => verbose = true,
            _ => {
                if path.is_some() {
                    usage();
                    std::process::exit(1);
                }
                path = Some(a);
            }
        }
    }

    let Some(path) = path else {
        usage();
        std::process::exit(1);
    };

    let data = match fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to read: {path}: {e}");
            std::process::exit(1);
        }
    };

    let sig = Signals::collect(&data);
    if verbose {
        eprintln!("File: {path}");
        eprintln!("  ELF: {}", sig.elf);
        eprintln!("  R1 VIRUS_PAYLOAD_BEGIN_MARKER: {}", sig.r1_payload_marker);
        eprintln!(
            "  R2 fdgffdgsdhgtrhtshs + Welcome…: {}",
            sig.r2_suffix_and_welcome
        );
        eprintln!(
            "  R3 fdgffdgsdhgtrhtshs + infect_…: {}",
            sig.r3_suffix_and_infect
        );
    }

    if sig.infected() {
        println!("INFECTED");
    } else if !sig.elf {
        println!("NOT_INFECTED (not ELF)");
    } else {
        println!("NOT_INFECTED");
    }
}
