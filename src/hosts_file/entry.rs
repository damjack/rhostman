const DISABLED_PREFIX: &str = "#rhostman-disabled# ";
const MARKER_PREFIX: &str = "rhostman:";

/// A single line of an /etc/hosts file, in structurally-parsed form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostsLine {
    /// A real mapping entry: `<ip> <host1> <host2> ... [# inline comment]`.
    Entry {
        ip: String,
        hostnames: Vec<String>,
        inline_comment: Option<String>,
        /// Whether rhostman has disabled (commented out) this entry.
        disabled: bool,
    },
    /// A full-line comment that is neither a rhostman marker nor a
    /// rhostman-disabled entry. Stored verbatim for exact round-tripping.
    Comment(String),
    /// A rhostman-managed source block delimiter:
    /// `# rhostman:<name>:start` / `# rhostman:<name>:end`.
    Marker { name: String, boundary: MarkerBoundary },
    /// A blank line, preserved for minimal-diff round-tripping.
    Blank,
    /// Anything that doesn't parse cleanly into the above. Preserved
    /// verbatim so an unusual line never causes data loss.
    Raw(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerBoundary {
    Start,
    End,
}

impl MarkerBoundary {
    fn as_str(self) -> &'static str {
        match self {
            MarkerBoundary::Start => "start",
            MarkerBoundary::End => "end",
        }
    }
}

impl HostsLine {
    pub fn parse(raw_line: &str) -> HostsLine {
        let line = raw_line.trim_end();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return HostsLine::Blank;
        }

        if let Some(rest) = trimmed.strip_prefix(DISABLED_PREFIX) {
            return match parse_entry_body(rest) {
                Some((ip, hostnames, inline_comment)) => HostsLine::Entry {
                    ip,
                    hostnames,
                    inline_comment,
                    disabled: true,
                },
                None => HostsLine::Raw(line.to_string()),
            };
        }

        if let Some(marker) = parse_marker(trimmed) {
            return marker;
        }

        if trimmed.starts_with('#') {
            return HostsLine::Comment(line.to_string());
        }

        match parse_entry_body(trimmed) {
            Some((ip, hostnames, inline_comment)) => HostsLine::Entry {
                ip,
                hostnames,
                inline_comment,
                disabled: false,
            },
            None => HostsLine::Raw(line.to_string()),
        }
    }

    pub fn to_line_string(&self) -> String {
        match self {
            HostsLine::Entry {
                ip,
                hostnames,
                inline_comment,
                disabled,
            } => {
                let mut body = format!("{} {}", ip, hostnames.join(" "));
                if let Some(comment) = inline_comment {
                    body.push_str(&format!(" # {}", comment));
                }
                if *disabled {
                    format!("{}{}", DISABLED_PREFIX, body)
                } else {
                    body
                }
            }
            HostsLine::Comment(text) => text.clone(),
            HostsLine::Marker { name, boundary } => {
                format!("# {}{}:{}", MARKER_PREFIX, name, boundary.as_str())
            }
            HostsLine::Blank => String::new(),
            HostsLine::Raw(text) => text.clone(),
        }
    }
}

fn parse_marker(trimmed: &str) -> Option<HostsLine> {
    let body = trimmed.strip_prefix('#')?.trim_start();
    let rest = body.strip_prefix(MARKER_PREFIX)?;
    let mut parts = rest.splitn(2, ':');
    let name = parts.next()?;
    let boundary_str = parts.next()?.trim();
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        return None;
    }
    let boundary = match boundary_str {
        "start" => MarkerBoundary::Start,
        "end" => MarkerBoundary::End,
        _ => return None,
    };
    Some(HostsLine::Marker {
        name: name.to_string(),
        boundary,
    })
}

fn parse_entry_body(s: &str) -> Option<(String, Vec<String>, Option<String>)> {
    let (main, comment) = match s.split_once('#') {
        Some((m, c)) => (m.trim_end(), Some(c.trim().to_string())),
        None => (s, None),
    };
    let mut tokens = main.split_whitespace();
    let ip = tokens.next()?;
    if !ip.contains('.') && !ip.contains(':') {
        return None;
    }
    let hostnames: Vec<String> = tokens.map(|t| t.to_string()).collect();
    if hostnames.is_empty() {
        return None;
    }
    Some((ip.to_string(), hostnames, comment))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_blank_line() {
        assert_eq!(HostsLine::parse(""), HostsLine::Blank);
        assert_eq!(HostsLine::parse("   "), HostsLine::Blank);
    }

    #[test]
    fn parses_simple_entry() {
        let line = HostsLine::parse("127.0.0.1 localhost");
        assert_eq!(
            line,
            HostsLine::Entry {
                ip: "127.0.0.1".to_string(),
                hostnames: vec!["localhost".to_string()],
                inline_comment: None,
                disabled: false,
            }
        );
        assert_eq!(line.to_line_string(), "127.0.0.1 localhost");
    }

    #[test]
    fn parses_entry_with_multiple_hostnames_and_comment() {
        let line = HostsLine::parse("0.0.0.0 ads.example.com  tracker.example.com # blocklist entry");
        assert_eq!(
            line,
            HostsLine::Entry {
                ip: "0.0.0.0".to_string(),
                hostnames: vec!["ads.example.com".to_string(), "tracker.example.com".to_string()],
                inline_comment: Some("blocklist entry".to_string()),
                disabled: false,
            }
        );
        assert_eq!(
            line.to_line_string(),
            "0.0.0.0 ads.example.com tracker.example.com # blocklist entry"
        );
    }

    #[test]
    fn round_trips_disabled_entry() {
        let original = "#rhostman-disabled# 0.0.0.0 ads.example.com";
        let line = HostsLine::parse(original);
        assert_eq!(
            line,
            HostsLine::Entry {
                ip: "0.0.0.0".to_string(),
                hostnames: vec!["ads.example.com".to_string()],
                inline_comment: None,
                disabled: true,
            }
        );
        assert_eq!(line.to_line_string(), original);
    }

    #[test]
    fn round_trips_marker_lines() {
        let start = HostsLine::parse("# rhostman:my-source:start");
        assert_eq!(
            start,
            HostsLine::Marker {
                name: "my-source".to_string(),
                boundary: MarkerBoundary::Start,
            }
        );
        assert_eq!(start.to_line_string(), "# rhostman:my-source:start");

        let end = HostsLine::parse("#rhostman:my-source:end");
        assert_eq!(
            end,
            HostsLine::Marker {
                name: "my-source".to_string(),
                boundary: MarkerBoundary::End,
            }
        );
    }

    #[test]
    fn plain_comment_is_not_a_marker_or_disabled_entry() {
        let line = HostsLine::parse("# just a note from the admin");
        assert_eq!(line, HostsLine::Comment("# just a note from the admin".to_string()));
    }

    #[test]
    fn line_without_ip_like_token_is_raw() {
        let line = HostsLine::parse("this is not a hosts entry");
        assert_eq!(line, HostsLine::Raw("this is not a hosts entry".to_string()));
    }
}
