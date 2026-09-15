use reqwest::blocking::Client;
use std::time::Duration;

use crate::errors::RhostmanResult;
use crate::hosts_file::HostsLine;

/// Fetches a raw text file from `url` and parses its lines as hosts-file
/// entries, keeping only lines that parse as a real `Entry` (comments and
/// blanks in the remote file are dropped).
pub fn fetch_entries(url: &str) -> RhostmanResult<Vec<HostsLine>> {
    let client = Client::builder()
        .user_agent(concat!("rhostman/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(15))
        .build()?;
    let body = client.get(url).send()?.error_for_status()?.text()?;
    Ok(parse_body(&body))
}

/// Fetches `url` and flattens it into a plain list of hostnames, for use by
/// tracked sources (which normalize every entry to an `0.0.0.0` blocking
/// line regardless of what IP, if any, the remote file used).
pub fn fetch_blocklist_domains(url: &str) -> RhostmanResult<Vec<String>> {
    Ok(fetch_entries(url)?
        .into_iter()
        .flat_map(|line| match line {
            HostsLine::Entry { hostnames, .. } => hostnames,
            _ => vec![],
        })
        .collect())
}

fn parse_body(body: &str) -> Vec<HostsLine> {
    body.lines()
        .map(HostsLine::parse)
        .filter(|line| matches!(line, HostsLine::Entry { .. }))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_body_keeps_only_entries() {
        let body = "# a comment\n\n0.0.0.0 ads.example.com\n127.0.0.1 localhost\nnot a hosts line\n";
        let entries = parse_body(body);
        assert_eq!(entries.len(), 2);
        assert!(
            matches!(&entries[0], HostsLine::Entry { hostnames, .. } if hostnames == &["ads.example.com".to_string()])
        );
        assert!(matches!(&entries[1], HostsLine::Entry { hostnames, .. } if hostnames == &["localhost".to_string()]));
    }
}
