/**
 * // are for comments
 * # is for disabling something
 */

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedLine {
    pub content: String,
    pub is_enabled: bool,
    pub is_user_comment: bool,
}

// returns the unhashed line and returns if it is enabled
pub fn parse_line(line: &str) -> ParsedLine {
    let trimmed = line.trim_start();
    let is_disabled = trimmed.starts_with('#');
    let is_user_comment = trimmed
        .trim_start_matches('#')
        .trim_start()
        .starts_with("//");
    
    let line_wihout_hash = line
        .trim()
        .trim_start_matches("#")
        .trim_start()
        .trim_start_matches("//")
        .trim_start()
        .trim_start_matches("/")
        .trim().to_string();

    return ParsedLine {
        content: line_wihout_hash,
        is_enabled: !is_disabled,
        is_user_comment: is_user_comment
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
                is_enabled: false,
                is_user_comment: false,
            }
        );
        assert_eq!(
            parse_line("&active=false "),
            ParsedLine {
                content: "&active=false".to_string(),
                is_enabled: true,
                is_user_comment: false,
            }
        );
        assert_eq!(
            parse_line("#&active=false "),
            ParsedLine {
                content: "&active=false".to_string(),
                is_enabled: false,
                is_user_comment: false,
            }
        );
    }

    #[test]
    fn test_user_comment() {
        assert_eq!(
            parse_line("// this is a comment"),
            ParsedLine {
                content: "this is a comment".to_string(),
                is_enabled: true,
                is_user_comment: true,
            }
        );
        assert_eq!(
            parse_line("  // indented comment "),
            ParsedLine {
                content: "indented comment".to_string(),
                is_enabled: true,
                is_user_comment: true,
            }
        );
        assert_eq!(
            parse_line("&active=true"),
            ParsedLine {
                content: "&active=true".to_string(),
                is_enabled: true,
                is_user_comment: false,
            }
        );
        assert_eq!(
            parse_line("/ single slash"),
            ParsedLine {
                content: "single slash".to_string(),
                is_enabled: true,
                is_user_comment: false,
            }
        );
    }

    #[test]
    fn test_disabled_user_comment() {
        assert_eq!(
            parse_line("#// disabled comment"),
            ParsedLine {
                content: "disabled comment".to_string(),
                is_enabled: false,
                is_user_comment: true,
            }
        );
        assert_eq!(
            parse_line("  #// indented disabled comment "),
            ParsedLine {
                content: "indented disabled comment".to_string(),
                is_enabled: false,
                is_user_comment: true,
            }
        );
        assert_eq!(
            parse_line("# // space between "),
            ParsedLine {
                content: "space between".to_string(),
                is_enabled: false,
                is_user_comment: true,
            }
        );
    }
}
