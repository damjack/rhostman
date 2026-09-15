use crate::errors::RhostmanResult;
use crate::hosts_file::entry::{HostsLine, MarkerBoundary};

/// The full contents of an /etc/hosts file, parsed into structured lines.
pub struct HostsDocument {
    lines: Vec<HostsLine>,
}

impl HostsDocument {
    pub fn parse(content: &str) -> HostsDocument {
        HostsDocument {
            lines: content.lines().map(HostsLine::parse).collect(),
        }
    }

    pub fn render(&self) -> String {
        let mut rendered: String = self
            .lines
            .iter()
            .map(HostsLine::to_line_string)
            .collect::<Vec<_>>()
            .join("\n");
        rendered.push('\n');
        rendered
    }

    fn matches_exact(ip: &str, hostnames: &[String], host: &str) -> bool {
        ip.eq_ignore_ascii_case(host) || hostnames.iter().any(|h| h.eq_ignore_ascii_case(host))
    }

    fn matches_domain(hostname: &str, domain: &str) -> bool {
        let hostname = hostname.to_ascii_lowercase();
        let domain = domain.to_ascii_lowercase();
        hostname == domain || hostname.ends_with(&format!(".{}", domain))
    }

    pub fn find_entry_by_host(&self, host: &str) -> Option<usize> {
        self.lines.iter().position(
            |line| matches!(line, HostsLine::Entry { ip, hostnames, .. } if Self::matches_exact(ip, hostnames, host)),
        )
    }

    pub fn find_entries_by_domain(&self, domain: &str) -> Vec<usize> {
        self.lines
            .iter()
            .enumerate()
            .filter_map(|(i, line)| match line {
                HostsLine::Entry { hostnames, .. } if hostnames.iter().any(|h| Self::matches_domain(h, domain)) => {
                    Some(i)
                }
                _ => None,
            })
            .collect()
    }

    pub fn find_source_block(&self, name: &str) -> Option<(usize, usize)> {
        let start = self.lines.iter().position(
            |line| matches!(line, HostsLine::Marker { name: n, boundary: MarkerBoundary::Start } if n == name),
        )?;
        let end_offset = self.lines[start + 1..].iter().position(
            |line| matches!(line, HostsLine::Marker { name: n, boundary: MarkerBoundary::End } if n == name),
        )?;
        Some((start, start + 1 + end_offset))
    }

    /// Appends a new entry. Idempotent: does nothing if an entry with the
    /// same IP and the exact same set of hostnames already exists.
    pub fn add_entry(&mut self, ip: &str, hostnames: &[String], comment: Option<&str>) -> RhostmanResult<()> {
        let already_present = self.lines.iter().any(|line| {
            matches!(line, HostsLine::Entry { ip: existing_ip, hostnames: existing_hosts, .. }
                if existing_ip == ip && existing_hosts.as_slice() == hostnames)
        });
        if already_present {
            return Ok(());
        }
        self.lines.push(HostsLine::Entry {
            ip: ip.to_string(),
            hostnames: hostnames.to_vec(),
            inline_comment: comment.map(str::to_string),
            disabled: false,
        });
        Ok(())
    }

    /// Comments out the single entry matching `host` exactly. Returns 1 if
    /// an entry was disabled, 0 if no match was found or it was already disabled.
    pub fn disable_by_host(&mut self, host: &str) -> RhostmanResult<usize> {
        for line in self.lines.iter_mut() {
            if let HostsLine::Entry {
                ip,
                hostnames,
                disabled,
                ..
            } = line
            {
                if !*disabled && Self::matches_exact(ip, hostnames, host) {
                    *disabled = true;
                    return Ok(1);
                }
            }
        }
        Ok(0)
    }

    /// Comments out every entry with a hostname matching `domain` (exact or
    /// subdomain). Returns the number of entries disabled.
    pub fn disable_by_domain(&mut self, domain: &str) -> RhostmanResult<usize> {
        let mut count = 0;
        for line in self.lines.iter_mut() {
            if let HostsLine::Entry {
                hostnames, disabled, ..
            } = line
            {
                if !*disabled && hostnames.iter().any(|h| Self::matches_domain(h, domain)) {
                    *disabled = true;
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    /// Removes the single entry matching `host` exactly. Returns 1 if an
    /// entry was removed, 0 if no match was found.
    pub fn remove_by_host(&mut self, host: &str) -> RhostmanResult<usize> {
        let mut removed = false;
        self.lines.retain(|line| {
            if removed {
                return true;
            }
            if let HostsLine::Entry { ip, hostnames, .. } = line {
                if Self::matches_exact(ip, hostnames, host) {
                    removed = true;
                    return false;
                }
            }
            true
        });
        Ok(if removed { 1 } else { 0 })
    }

    /// Removes every entry with a hostname matching `domain` (exact or
    /// subdomain). Returns the number of entries removed.
    pub fn remove_by_domain(&mut self, domain: &str) -> RhostmanResult<usize> {
        let mut count = 0;
        self.lines.retain(|line| {
            if let HostsLine::Entry { hostnames, .. } = line {
                if hostnames.iter().any(|h| Self::matches_domain(h, domain)) {
                    count += 1;
                    return false;
                }
            }
            true
        });
        Ok(count)
    }

    /// Inserts or replaces the marker-delimited block for a tracked source,
    /// leaving every other line untouched. `new_lines` are already-rendered
    /// hosts-file lines (e.g. `"0.0.0.0 example.com"`).
    pub fn replace_source_block(&mut self, name: &str, new_lines: &[String]) -> RhostmanResult<()> {
        let mut block = Vec::with_capacity(new_lines.len() + 2);
        block.push(HostsLine::Marker {
            name: name.to_string(),
            boundary: MarkerBoundary::Start,
        });
        block.extend(new_lines.iter().map(|l| HostsLine::parse(l)));
        block.push(HostsLine::Marker {
            name: name.to_string(),
            boundary: MarkerBoundary::End,
        });

        if let Some((start, end)) = self.find_source_block(name) {
            self.lines.splice(start..=end, block);
        } else {
            if !self.lines.is_empty() && !matches!(self.lines.last(), Some(HostsLine::Blank)) {
                self.lines.push(HostsLine::Blank);
            }
            self.lines.extend(block);
        }
        Ok(())
    }

    /// Removes a tracked source's marker-delimited block, if present.
    /// Returns whether a block was found and removed.
    pub fn remove_source_block(&mut self, name: &str) -> RhostmanResult<bool> {
        match self.find_source_block(name) {
            Some((start, end)) => {
                self.lines.drain(start..=end);
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hosts(strings: &[&str]) -> Vec<String> {
        strings.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn exact_host_match_does_not_false_positive_on_substring() {
        let doc = HostsDocument::parse("1.2.3.4 notexample.com\n5.6.7.8 example.com\n");
        assert!(doc.find_entry_by_host("example.com").is_some());
        let idx = doc.find_entry_by_host("example.com").unwrap();
        assert_eq!(idx, 1);
    }

    #[test]
    fn domain_match_covers_subdomains_but_not_lookalikes() {
        let doc = HostsDocument::parse("0.0.0.0 ads.example.com\n0.0.0.0 example.com\n0.0.0.0 notexample.com\n");
        let matches = doc.find_entries_by_domain("example.com");
        assert_eq!(matches, vec![0, 1]);
    }

    #[test]
    fn add_entry_is_idempotent() {
        let mut doc = HostsDocument::parse("127.0.0.1 localhost\n");
        doc.add_entry("1.2.3.4", &hosts(&["foo.test"]), None).unwrap();
        doc.add_entry("1.2.3.4", &hosts(&["foo.test"]), None).unwrap();
        let rendered = doc.render();
        assert_eq!(rendered.matches("foo.test").count(), 1);
    }

    #[test]
    fn disable_by_host_toggles_exactly_one_and_is_idempotent() {
        let mut doc = HostsDocument::parse("1.2.3.4 foo.test\n1.2.3.4 foo.test\n");
        assert_eq!(doc.disable_by_host("foo.test").unwrap(), 1);
        // second call finds no more *enabled* matches for this exact scan order
        // (first line now disabled), so it disables the second occurrence.
        assert_eq!(doc.disable_by_host("foo.test").unwrap(), 1);
        assert_eq!(doc.disable_by_host("foo.test").unwrap(), 0);
        let rendered = doc.render();
        assert_eq!(rendered.matches("#rhostman-disabled#").count(), 2);
    }

    #[test]
    fn remove_by_domain_removes_all_matches() {
        let mut doc = HostsDocument::parse("0.0.0.0 ads.example.com\n0.0.0.0 tracker.example.com\n1.2.3.4 keep.test\n");
        let removed = doc.remove_by_domain("example.com").unwrap();
        assert_eq!(removed, 2);
        let rendered = doc.render();
        assert!(!rendered.contains("example.com"));
        assert!(rendered.contains("keep.test"));
    }

    #[test]
    fn source_block_round_trips_and_updates_in_place() {
        let mut doc = HostsDocument::parse("127.0.0.1 localhost\n1.2.3.4 manual.test\n");
        doc.replace_source_block(
            "mylist",
            &hosts(&["0.0.0.0 ads.example.com", "0.0.0.0 tracker.example.com"]),
        )
        .unwrap();
        let rendered = doc.render();
        assert!(rendered.contains("# rhostman:mylist:start"));
        assert!(rendered.contains("# rhostman:mylist:end"));
        assert!(rendered.contains("manual.test"));

        doc.replace_source_block("mylist", &hosts(&["0.0.0.0 only.example.com"]))
            .unwrap();
        let rendered = doc.render();
        assert!(rendered.contains("only.example.com"));
        assert!(!rendered.contains("ads.example.com"));
        assert!(rendered.contains("manual.test"), "manual entry must survive an update");
    }

    #[test]
    fn source_block_update_does_not_touch_other_blocks() {
        let mut doc = HostsDocument::parse("");
        doc.replace_source_block("list-a", &hosts(&["0.0.0.0 a.test"])).unwrap();
        doc.replace_source_block("list-b", &hosts(&["0.0.0.0 b.test"])).unwrap();

        doc.replace_source_block("list-a", &hosts(&["0.0.0.0 a2.test"]))
            .unwrap();

        let rendered = doc.render();
        assert!(rendered.contains("a2.test"));
        assert!(!rendered.contains("0.0.0.0 a.test"));
        assert!(
            rendered.contains("b.test"),
            "list-b block must be untouched by a list-a update"
        );
    }

    #[test]
    fn remove_source_block_removes_only_named_block() {
        let mut doc = HostsDocument::parse("");
        doc.replace_source_block("list-a", &hosts(&["0.0.0.0 a.test"])).unwrap();
        doc.replace_source_block("list-b", &hosts(&["0.0.0.0 b.test"])).unwrap();

        let removed = doc.remove_source_block("list-a").unwrap();
        assert!(removed);
        let rendered = doc.render();
        assert!(!rendered.contains("a.test"));
        assert!(rendered.contains("b.test"));

        assert!(!doc.remove_source_block("list-a").unwrap());
    }
}
