use lineeditor::Color;
use lineeditor::Completer;
use lineeditor::Highlighter;
use lineeditor::Hinter;
use lineeditor::KeyModifiers;
use lineeditor::LineEditor;
use lineeditor::Span;
use lineeditor::StringPrompt;
use lineeditor::Suggestion;
use lineeditor::event::LineEditorEvent;
use lineeditor::keybindings::KeyCombination;
use lineeditor::style::Style;
use lineeditor::styled_buffer::StyledBuffer;

const GITQL_RESERVED_KEYWORDS: [&str; 57] = [
    "do",
    "set",
    "select",
    "distinct",
    "from",
    "where",
    "limit",
    "offset",
    "order",
    "using",
    "case",
    "when",
    "then",
    "else",
    "end",
    "between",
    "in",
    "is",
    "on",
    "not",
    "like",
    "glob",
    "describe",
    "show",
    "regexp",
    "into",
    "outfile",
    "dumpfile",
    "lines",
    "fields",
    "enclosed",
    "terminated",
    "join",
    "left",
    "right",
    "cross",
    "inner",
    "outer",
    "group",
    "by",
    "having",
    "with",
    "rollup",
    "div",
    "mod",
    "or",
    "and",
    "xor",
    "true",
    "false",
    "null",
    "infinity",
    "nan",
    "as",
    "asc",
    "desc",
    "array",
];

#[derive(Default)]
pub struct GitQLHinter {}

impl Hinter for GitQLHinter {
    fn hint(&self, buffer: &mut StyledBuffer) -> Option<StyledBuffer> {
        if let Some(keyword) = buffer.last_alphabetic_keyword() {
            let keyword_lower = keyword.to_lowercase();
            for word in GITQL_RESERVED_KEYWORDS {
                if word.starts_with(&keyword_lower) {
                    let hint = &word[keyword.len()..];
                    let mut styled_buffer = StyledBuffer::default();
                    let mut style = Style::default();
                    style.set_foreground_color(Color::DarkGrey);
                    styled_buffer.insert_styled_string(hint, style);
                    return Some(styled_buffer);
                }
            }
        }
        None
    }
}

pub struct FixedCompleter;

impl Completer for FixedCompleter {
    fn complete(&self, input: &StyledBuffer) -> Vec<Suggestion> {
        let mut suggestions: Vec<Suggestion> = vec![];
        if input.position() != input.len() {
            return suggestions;
        }

        if let Some(keyword) = input.last_alphabetic_keyword() {
            for reserved_keyword in GITQL_RESERVED_KEYWORDS {
                if reserved_keyword.starts_with(&keyword) {
                    let suggestion = Suggestion {
                        content: StyledBuffer::from(reserved_keyword),
                        span: Span {
                            start: input.len() - keyword.len(),
                            end: input.len(),
                        },
                    };
                    suggestions.push(suggestion);
                }
            }
        }
        suggestions
    }
}

#[derive(Default)]
pub struct GitQLHighlighter;

impl Highlighter for GitQLHighlighter {
    fn highlight(&self, buffer: &mut StyledBuffer) {
        let lines = buffer.buffer().clone();
        let mut i: usize = 0;

        let mut keyword_style = Style::default();
        keyword_style.set_foreground_color(Color::Magenta);

        let mut string_style = Style::default();
        string_style.set_foreground_color(Color::Yellow);

        loop {
            if i >= lines.len() {
                break;
            }

            // Highlight String literal
            if lines[i] == '"' {
                buffer.style_char(i, string_style.clone());
                i += 1;

                while i < lines.len() && lines[i] != '"' {
                    buffer.style_char(i, string_style.clone());
                    i += 1;
                }

                if i < lines.len() && lines[i] == '"' {
                    buffer.style_char(i, string_style.clone());
                    i += 1;
                }

                continue;
            }

            // Highlight reserved keyword
            if lines[i].is_alphabetic() {
                let start = i;
                let mut keyword = String::new();
                while i < lines.len() && (lines[i].is_alphanumeric() || lines[i] == '_') {
                    keyword.push(lines[i]);
                    i += 1;
                }

                keyword = keyword.to_lowercase();
                if GITQL_RESERVED_KEYWORDS.contains(&keyword.as_str()) {
                    buffer.style_range(start, i, keyword_style.clone())
                }
                continue;
            }

            i += 1;
        }
    }
}

#[derive(Default)]
pub struct MatchingBracketsHighlighter;

impl Highlighter for MatchingBracketsHighlighter {
    fn highlight(&self, buffer: &mut StyledBuffer) {
        let colors = [Color::Red, Color::Blue, Color::Yellow, Color::Green];
        let mut brackets_stack: Vec<Color> = vec![];
        let mut current_color_index = 0;

        let lines = buffer.buffer().clone();
        let mut i: usize = 0;
        loop {
            if i >= lines.len() {
                break;
            }

            if lines[i] == '"' {
                i += 1;
                while i < lines.len() && lines[i] != '"' {
                    i += 1;
                }

                if i < lines.len() {
                    i += 1;
                }
                continue;
            }

            if lines[i] == '(' || lines[i] == '<' || lines[i] == '[' || lines[i] == '{' {
                if current_color_index >= colors.len() {
                    current_color_index = 0;
                }

                let color = colors[current_color_index];
                current_color_index += 1;

                brackets_stack.push(color);

                let mut style = Style::default();
                style.set_foreground_color(color);
                buffer.style_char(i, style);
                i += 1;
                continue;
            }

            if lines[i] == ')' || lines[i] == '>' || lines[i] == ']' || lines[i] == '}' {
                let color = if brackets_stack.is_empty() {
                    colors[0]
                } else {
                    brackets_stack.pop().unwrap()
                };

                let mut style = Style::default();
                style.set_foreground_color(color);
                buffer.style_char(i, style);

                i += 1;
                continue;
            }
            i += 1;
        }
    }
}

pub fn create_new_line_editor() -> LineEditor {
    let prompt = StringPrompt::new("gitql > ".to_string());
    let mut line_editor = LineEditor::new(Box::new(prompt));

    let mut style = Style::default();
    style.set_background_color(lineeditor::Color::Cyan);
    line_editor.set_visual_selection_style(Some(style));

    line_editor.add_highlighter(Box::<GitQLHighlighter>::default());
    line_editor.add_highlighter(Box::<MatchingBracketsHighlighter>::default());
    line_editor.add_hinter(Box::<GitQLHinter>::default());
    line_editor.set_completer(Box::new(FixedCompleter));

    let bindings = line_editor.keybinding();
    bindings.register_common_control_bindings();
    bindings.register_common_navigation_bindings();
    bindings.register_common_edit_bindings();
    bindings.register_common_selection_bindings();
    bindings.register_binding(
        KeyCombination {
            key_kind: lineeditor::KeyEventKind::Press,
            modifier: KeyModifiers::NONE,
            key_code: lineeditor::KeyCode::Tab,
        },
        LineEditorEvent::ToggleAutoComplete,
    );

    line_editor
}
