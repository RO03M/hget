/**
 * // are for comments
 * # is for disabling something
 */

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedLine {
    pub content: String,
    pub is_enabled: bool,
}

// returns the unhashed line and returns if it is enabled
pub fn parse_line(line: &str) -> ParsedLine {
    let is_disabled = line.trim().starts_with('#');
    let line_wihout_hash = line.trim().trim_start_matches("#").trim().to_string();

    return ParsedLine {
        content: line_wihout_hash,
        is_enabled: !is_disabled,
    };
}

#[cfg(test)]
mod tests {
    use crate::line::*;

    #[test]
    fn test_strip_hash() {
        assert_eq!(
            parse_line(" ## &active=false "),
            ParsedLine {
                content: "&active=false".to_string(),
                is_enabled: false
            }
        );
        assert_eq!(
            parse_line("&active=false "),
            ParsedLine {
                content: "&active=false".to_string(),
                is_enabled: true
            }
        );
        assert_eq!(
            parse_line("#&active=false "),
            ParsedLine {
                content: "&active=false".to_string(),
                is_enabled: false
            }
        );
    }
}
