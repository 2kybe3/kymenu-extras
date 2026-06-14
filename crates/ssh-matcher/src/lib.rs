#[derive(Debug, Clone, PartialEq, Eq)]
struct Classes {
    negated: bool,
    items: Vec<Class>,
}

impl Classes {
    fn matches(&self, c: char) -> bool {
        for item in &self.items {
            match item {
                Class::Single(x) => {
                    if *x == c {
                        return true;
                    }
                }
                Class::Range(start, end) => {
                    if *start <= c && c <= *end {
                        return true;
                    }
                }
            }
        }

        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pattern {
    Star,
    Question,
    Char(char),
    Class(Classes),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Class {
    Single(char),
    Range(char, char),
}

pub struct Matcher {
    ast: Vec<Pattern>,
    negated: bool,
}

impl Matcher {
    pub fn new(pattern: &str) -> Matcher {
        let mut negated = false;
        let mut p = pattern;

        if let Some(stripped) = pattern.strip_prefix('!') {
            negated = true;
            p = stripped;
        }

        let mut chars = p.chars().peekable();
        let mut out = Vec::new();

        while let Some(c) = chars.next() {
            match c {
                '*' => out.push(Pattern::Star),
                '?' => out.push(Pattern::Question),

                '[' => {
                    let mut items = Vec::new();
                    let mut negated = false;

                    if let Some(&'!') = chars.peek() {
                        chars.next();
                        negated = true;
                    }

                    while let Some(&next) = chars.peek() {
                        if next == ']' {
                            chars.next();
                            break;
                        }

                        let start = chars.next().unwrap();

                        if let Some(&'-') = chars.peek() {
                            chars.next();
                            let end = chars.next().unwrap();
                            items.push(Class::Range(start, end));
                        } else {
                            items.push(Class::Single(start));
                        }
                    }

                    out.push(Pattern::Class(Classes { negated, items }));
                }

                _ => out.push(Pattern::Char(c)),
            }
        }

        Self { ast: out, negated }
    }

    pub fn matches(&self, text: &str) -> bool {
        let chars: Vec<char> = text.chars().collect();
        let result = Self::inner_matches(&self.ast, &chars);

        if self.negated { !result } else { result }
    }

    fn inner_matches(pattern: &[Pattern], text: &[char]) -> bool {
        match pattern.split_first() {
            Some((p, rest)) => match p {
                Pattern::Char(c) => {
                    if text.first() == Some(c) {
                        Self::inner_matches(rest, &text[1..])
                    } else {
                        false
                    }
                }

                Pattern::Question => {
                    if text.is_empty() {
                        false
                    } else {
                        Self::inner_matches(rest, &text[1..])
                    }
                }

                Pattern::Star => {
                    if Self::inner_matches(rest, text) {
                        return true;
                    }

                    if let Some((_, rest_text)) = text.split_first() {
                        Self::inner_matches(pattern, rest_text)
                    } else {
                        false
                    }
                }

                Pattern::Class(class) => {
                    if let Some(tc) = text.first() {
                        let matched = class.matches(*tc);
                        let ok = if class.negated { !matched } else { matched };

                        if ok {
                            Self::inner_matches(rest, &text[1..])
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            },
            None => text.is_empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_glob() {
        assert!(Matcher::new("test*").matches("test1"));
        assert!(Matcher::new("test*").matches("test11"));
        assert!(Matcher::new("test*test").matches("testhitest"));
        assert!(!Matcher::new("test*test").matches("testhi"));

        assert!(Matcher::new("test?").matches("test1"));
        assert!(Matcher::new("test??").matches("test12"));
        assert!(Matcher::new("test??b?").matches("test12b4"));
        assert!(!Matcher::new("test??b?").matches("test1234"));
        assert!(!Matcher::new("test?").matches("test12"));

        assert!(!Matcher::new("test[1-9]").matches("test0"));
        assert!(Matcher::new("test[1-9]").matches("test1"));
        assert!(Matcher::new("test[1-9]").matches("test9"));

        assert!(!Matcher::new("test").matches("test0"));

        assert!(Matcher::new("[abc][defg]").matches("ad"));
        assert!(!Matcher::new("[abc][defg]").matches("da"));

        assert!(Matcher::new("[abc][!defg]").matches("aa"));
        assert!(!Matcher::new("[abc][!defg]").matches("ad"));
    }
}
