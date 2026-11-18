//! Reading a shell line as the commands it runs: each simple command as its words, with what only runs
//! another command (sudo, env, timeout, xargs, ...) stripped, so a rule can look at the program itself.
//! Taken from devlab's guard (linux-phone-oco/lab-tools), where it was first written and proven; devlab
//! keeps only the rule of its own device. Here it learned here-documents: their body is data, not commands
//! (devlab's copy refused a file written through a quoted here-document whose text named pip).

/// Strip what only runs another command: assignments, sudo, env, timeout, nice, nohup, exec, command,
/// time, xargs, stdbuf.
pub fn unwrapped(words: &[String]) -> Vec<String> {
    let mut w: Vec<String> = words.to_vec();
    loop {
        let Some(first) = w.first().cloned() else {
            return w;
        };
        let program = first.rsplit('/').next().unwrap_or(&first).to_string();
        let skip = if is_assignment(&first) {
            1
        } else {
            match program.as_str() {
                "sudo" | "doas" => flags_then(
                    &w,
                    1,
                    &["-u", "-g", "-C", "-D", "-h", "-p", "-U", "-r", "-t"],
                ),
                "env" => {
                    let mut k = flags_then(&w, 1, &["-u", "-C", "-S"]);
                    while k < w.len() && is_assignment(&w[k]) {
                        k += 1;
                    }
                    k
                }
                "timeout" => flags_then(&w, 1, &["-s", "-k", "--signal", "--kill-after"]) + 1,
                "nice" => flags_then(&w, 1, &["-n"]),
                "nohup" | "exec" | "command" | "time" | "builtin" => flags_then(&w, 1, &[]),
                "xargs" => flags_then(&w, 1, &["-I", "-n", "-P", "-L", "-d", "-a", "-E", "-s"]),
                "stdbuf" => flags_then(&w, 1, &[]),
                _ => return w,
            }
        };
        if skip == 0 || skip > w.len() {
            return Vec::new();
        }
        w.drain(..skip);
    }
}

fn flags_then(w: &[String], mut k: usize, with_value: &[&str]) -> usize {
    while k < w.len() && w[k].starts_with('-') {
        if w[k] == "--" {
            return k + 1;
        }
        k += if with_value.contains(&w[k].as_str()) {
            2
        } else {
            1
        };
    }
    k
}

fn is_assignment(word: &str) -> bool {
    match word.split_once('=') {
        Some((name, _)) => {
            !name.is_empty()
                && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                && !name.starts_with(|c: char| c.is_ascii_digit())
        }
        None => false,
    }
}

/// The simple commands of a shell line, each as its words: split at ; & | && || newlines, parentheses,
/// braces, and command substitutions (whose insides are commands of their own); quotes are kept together.
pub fn simple_commands(line: &str) -> Vec<Vec<String>> {
    let mut all = Vec::new();
    let mut words: Vec<String> = Vec::new();
    let mut word = String::new();
    let mut in_word = false;
    // here-documents begun on this line, read from the next: (delimiter, quoted, <<- strips tabs)
    let mut pending: Vec<(String, bool, bool)> = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    let end_word = |words: &mut Vec<String>, word: &mut String, in_word: &mut bool| {
        if *in_word {
            words.push(std::mem::take(word));
            *in_word = false;
        }
    };
    let end_command = |all: &mut Vec<Vec<String>>, words: &mut Vec<String>| {
        if !words.is_empty() {
            all.push(std::mem::take(words));
        }
    };
    while i < chars.len() {
        let c = chars[i];
        match c {
            '\'' => {
                in_word = true;
                i += 1;
                while i < chars.len() && chars[i] != '\'' {
                    word.push(chars[i]);
                    i += 1;
                }
            }
            '"' => {
                in_word = true;
                i += 1;
                let mut inner = String::new();
                while i < chars.len() && chars[i] != '"' {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        i += 1;
                    }
                    inner.push(chars[i]);
                    i += 1;
                }
                // a substitution inside double quotes is a command too
                if inner.contains("$(") || inner.contains('`') {
                    all.extend(
                        substitutions(&inner)
                            .iter()
                            .flat_map(|s| simple_commands(s)),
                    );
                }
                word.push_str(&inner);
            }
            '\\' if i + 1 < chars.len() => {
                if chars[i + 1] != '\n' {
                    in_word = true;
                    word.push(chars[i + 1]);
                }
                i += 1;
            }
            '#' if !in_word => {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                continue;
            }
            ' ' | '\t' => end_word(&mut words, &mut word, &mut in_word),
            // a here-document (<<WORD, <<-WORD, <<'WORD'): its body, from the next line to the line that is
            // WORD, is data given to the command - not commands. Unquoted, its substitutions still run.
            '<' if chars.get(i + 1) == Some(&'<')
                && chars.get(i + 2) != Some(&'<')
                && (i == 0 || chars[i - 1] != '<') =>
            {
                end_word(&mut words, &mut word, &mut in_word);
                i += 2;
                let strip_tabs = chars.get(i) == Some(&'-');
                if strip_tabs {
                    i += 1;
                }
                while matches!(chars.get(i), Some(' ' | '\t')) {
                    i += 1;
                }
                let (mut delimiter, mut quoted) = (String::new(), false);
                while let Some(&d) = chars.get(i) {
                    match d {
                        '\'' | '"' => {
                            quoted = true;
                            i += 1;
                            while i < chars.len() && chars[i] != d {
                                delimiter.push(chars[i]);
                                i += 1;
                            }
                        }
                        '\\' => {
                            quoted = true;
                            i += 1;
                            if let Some(&n) = chars.get(i) {
                                delimiter.push(n);
                            }
                        }
                        ' ' | '\t' | '\n' | ';' | '&' | '|' | '(' | ')' | '<' | '>' => break,
                        _ => delimiter.push(d),
                    }
                    i += 1;
                }
                if !delimiter.is_empty() {
                    pending.push((delimiter, quoted, strip_tabs));
                }
                continue;
            }
            '\n' if !pending.is_empty() => {
                end_word(&mut words, &mut word, &mut in_word);
                end_command(&mut all, &mut words);
                i += 1;
                for (delimiter, quoted, strip_tabs) in std::mem::take(&mut pending) {
                    let mut body = String::new();
                    while i < chars.len() {
                        let end = chars[i..]
                            .iter()
                            .position(|&c| c == '\n')
                            .map_or(chars.len(), |p| i + p);
                        let text: String = chars[i..end].iter().collect();
                        i = end + 1;
                        let text = if strip_tabs {
                            text.trim_start_matches('\t')
                        } else {
                            &text
                        };
                        if text == delimiter {
                            break;
                        }
                        body.push_str(text);
                        body.push('\n');
                    }
                    if !quoted {
                        all.extend(substitutions(&body).iter().flat_map(|s| simple_commands(s)));
                    }
                }
                continue;
            }
            ';' | '&' | '|' | '\n' | '(' | ')' | '`' | '{' | '}'
                if !(c == '{' || c == '}') || !in_word =>
            {
                if c == '(' && in_word && word.ends_with('$') {
                    word.pop();
                }
                end_word(&mut words, &mut word, &mut in_word);
                end_command(&mut all, &mut words);
            }
            _ => {
                in_word = true;
                word.push(c);
            }
        }
        i += 1;
    }
    end_word(&mut words, &mut word, &mut in_word);
    end_command(&mut all, &mut words);
    all
}

/// The insides of $(...) and `...` in a text.
fn substitutions(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("$(") {
        let after = &rest[start + 2..];
        let mut depth = 1;
        let mut end = after.len();
        for (k, ch) in after.char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = k;
                        break;
                    }
                }
                _ => {}
            }
        }
        found.push(after[..end].to_string());
        rest = &after[end.min(after.len())..];
    }
    let mut parts = text.split('`');
    parts.next();
    found.extend(parts.step_by(2).map(str::to_string));
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    fn programs(line: &str) -> Vec<String> {
        simple_commands(line)
            .iter()
            .filter_map(|w| unwrapped(w).first().cloned())
            .collect()
    }

    #[test]
    fn a_quoted_here_document_is_data() {
        let p = programs("cat > f <<'EOF'\nthen python3 x\n`pip` y\nEOF\necho ok");
        assert_eq!(p, ["cat", "echo"], "{p:?}");
        let p = programs("cat <<\"END\" | tee f\n$(pip install x)\nEND\n");
        assert_eq!(p, ["cat", "tee"], "{p:?}");
    }

    #[test]
    fn an_unquoted_here_documents_substitutions_run() {
        assert!(programs("cat <<EOF\nversion $(python3 -V)\nEOF").contains(&"python3".to_string()));
        assert!(!programs("cat <<EOF\nplain python3 text\nEOF").contains(&"python3".to_string()));
    }

    #[test]
    fn what_follows_a_here_document_is_read() {
        let p = programs("cat <<-EOF\n\tpip\n\tEOF\npip install x");
        assert_eq!(p, ["cat", "pip"], "{p:?}");
        let p = programs("a <<A; b <<B\nx\nA\ny\nB\nc");
        assert_eq!(p, ["a", "b", "c"], "{p:?}");
    }

    #[test]
    fn a_here_string_is_not_a_here_document() {
        assert_eq!(programs("cat <<< 'python3'\nls"), ["cat", "ls"]);
    }
}
