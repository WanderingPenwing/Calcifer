#![allow(dead_code)]

pub mod highlighting;
mod syntax;
pub mod themes;

use eframe::egui;
use egui::{text_edit::CCursorRange, text::CCursor};
use highlighting::highlight;
use std::hash::{Hash, Hasher};
pub use syntax::{Syntax, TokenType};
pub use themes::ColorTheme;
use std::cmp::{min, max};
use std::ops::{Bound, RangeBounds};


trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
	fn char_at(&self, index: usize) -> char;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }

    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
	
	fn char_at(&self, index: usize) -> char {
        self.chars().nth(index).unwrap_or('\0') // '\0' is used as the default value
    }
}


#[derive(Clone, Debug, PartialEq)]
/// CodeEditor struct which stores settings for highlighting.
pub struct CodeEditor {
    id: String,
    theme: ColorTheme,
    syntax: Syntax,
    numlines: bool,
    fontsize: f32,
    rows: usize,
    vscroll: bool,
    stick_to_bottom: bool,
    shrink: bool,
}

impl Hash for CodeEditor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.theme.hash(state);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        (self.fontsize as u32).hash(state);
        self.syntax.hash(state);
    }
}

impl Default for CodeEditor {
    fn default() -> CodeEditor {
        CodeEditor {
            id: String::from("Code Editor"),
            theme: ColorTheme::GRUVBOX,
            syntax: Syntax::rust(),
            numlines: true,
            fontsize: 10.0,
            rows: 10,
            vscroll: true,
            stick_to_bottom: false,
            shrink: false,
        }
    }
}

impl CodeEditor {
    pub fn id_source(self, id_source: impl Into<String>) -> Self {
        CodeEditor {
            id: id_source.into(),
            ..self
        }
    }

    /// Minimum number of rows to show.
    ///
    /// **Default: 10**
    pub fn with_rows(self, rows: usize) -> Self {
        CodeEditor { rows, ..self }
    }

    /// Use custom Color Theme
    ///
    /// **Default: Gruvbox**
    pub fn with_theme(self, theme: ColorTheme) -> Self {
        CodeEditor { theme, ..self }
    }

    /// Use custom font size
    ///
    /// **Default: 10.0**
    pub fn with_fontsize(self, fontsize: f32) -> Self {
        CodeEditor { fontsize, ..self }
    }

    #[cfg(feature = "egui")]
    /// Use UI font size
    pub fn with_ui_fontsize(self, ui: &mut egui::Ui) -> Self {
        CodeEditor {
            fontsize: egui::TextStyle::Monospace.resolve(ui.style()).size,
            ..self
        }
    }

    /// Show or hide lines numbering
    ///
    /// **Default: true**
    pub fn with_numlines(self, numlines: bool) -> Self {
        CodeEditor { numlines, ..self }
    }

    /// Use custom syntax for highlighting
    ///
    /// **Default: Rust**
    pub fn with_syntax(self, syntax: Syntax) -> Self {
        CodeEditor { syntax, ..self }
    }

    /// Turn on/off scrolling on the vertical axis.
    ///
    /// **Default: true**
    pub fn vscroll(self, vscroll: bool) -> Self {
        CodeEditor { vscroll, ..self }
    }
    /// Should the containing area shrink if the content is small?
    ///
    /// **Default: false**
    pub fn auto_shrink(self, shrink: bool) -> Self {
        CodeEditor { shrink, ..self }
    }

    /// Stick to bottom
    /// The scroll handle will stick to the bottom position even while the content size
    /// changes dynamically. This can be useful to simulate terminal UIs or log/info scrollers.
    /// The scroll handle remains stuck until user manually changes position. Once "unstuck"
    /// it will remain focused on whatever content viewport the user left it on. If the scroll
    /// handle is dragged to the bottom it will again become stuck and remain there until manually
    /// pulled from the end position.
    ///
    /// **Default: false**
    pub fn stick_to_bottom(self, stick_to_bottom: bool) -> Self {
        CodeEditor {
            stick_to_bottom,
            ..self
        }
    }

    pub fn format(&self, ty: TokenType) -> egui::text::TextFormat {
        let font_id = egui::FontId::monospace(self.fontsize);
        let color = self.theme.type_color(ty);
        egui::text::TextFormat::simple(font_id, color)
    }

    fn numlines_show(&self, ui: &mut egui::Ui, text: &str) {
        let total = if text.ends_with('\n') || text.is_empty() {
            text.lines().count() + 1
        } else {
            text.lines().count()
        }
        .max(self.rows);
        let max_indent = total.to_string().len();
        let mut counter = (1..=total)
            .map(|i| {
                let label = i.to_string();
                format!(
                    "{}{label}",
                    " ".repeat(max_indent.saturating_sub(label.len()))
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        #[allow(clippy::cast_precision_loss)]
        let width = max_indent as f32 * self.fontsize * 0.5;

        let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
            let layout_job = egui::text::LayoutJob::single_section(
                string.to_string(),
                egui::TextFormat::simple(
                    egui::FontId::monospace(self.fontsize),
                    self.theme.type_color(TokenType::Comment(true)),
                ),
            );
            ui.fonts(|f| f.layout_job(layout_job))
        };

        ui.add(
            egui::TextEdit::multiline(&mut counter)
                .id_source(format!("{}_numlines", self.id))
                .font(egui::TextStyle::Monospace)
                .interactive(false)
                .frame(false)
                .desired_rows(self.rows)
                .desired_width(width)
                .layouter(&mut layouter),
        );
    }

    /// Show Code Editor
    pub fn show(&mut self, ui: &mut egui::Ui, text: &mut String, history: &mut Vec<String>, last_cursor: &mut Option<CCursorRange>, vertical_offset: &mut f32, override_cursor: Option<CCursorRange>) {
        //let mut text_edit_output: Option<TextEditOutput> = None;
        let mut code_editor = |ui: &mut egui::Ui| {
            ui.horizontal_top(|h| {
                self.theme.modify_style(h, self.fontsize);
                if self.numlines {
                    self.numlines_show(h, text);
                }
                egui::ScrollArea::horizontal()
                    .id_source(format!("{}_inner_scroll", self.id))
                    .show(h, |ui| {
                        let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
                            let layout_job = highlight(ui.ctx(), self, string);
                            ui.fonts(|f| f.layout_job(layout_job))
                        };

						let previous_text = text.clone();

						let mut output = egui::TextEdit::multiline(text)
                            .id_source(&self.id)
                            .lock_focus(true)
                            .desired_rows(self.rows)
                            .frame(true)
                            .desired_width(if self.shrink { 0.0 } else { f32::MAX })
                            .layouter(&mut layouter)
                            .show(ui);

						let mut get_new_cursor : bool = true;
						let mut extend : isize = 0;

						if output.response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
							println!("line break");
						}

						if output.response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::E) && i.modifiers.ctrl) {
							if let Some(range) = last_cursor {
								(*text, extend) = self.toggle_start_of_line(range.clone(), text.clone(), "//");
								get_new_cursor = false;
							}
						}

						if output.response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Tab)) {
							if let Some(range) = last_cursor {
								if range.primary.index != range.secondary.index {
									(*text, extend) = self.add_start_of_line(range.clone(), previous_text.clone(), "\t");
									get_new_cursor = false;
								}
							}
						}

						if output.response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Tab) && i.modifiers.shift) {
							if let Some(range) = last_cursor {
								if range.primary.index != range.secondary.index {
									(*text, extend) = self.remove_start_of_line(range.clone(), previous_text.clone(), "\t");
									get_new_cursor = false;
								}
							}
						}

						if output.response.has_focus() && ui.input( |i| i.key_pressed(egui::Key::Z) && i.modifiers.ctrl) {
							println!("Ctrl+Z");
							//let current_tab = &mut self.tabs[self.selected_tab.to_index()];
							//if current_tab.history.len() > 1 {
								//current_tab.code = current_tab.history[current_tab.history.len() - 2].clone();
								//current_tab.history.pop();
							//}
						}
					
						

						if override_cursor != None {
							output.response.request_focus();
							output.state.set_ccursor_range(override_cursor);
							output.state.store(ui.ctx(), output.response.id);
						} else if get_new_cursor {
							*last_cursor = output.state.clone().ccursor_range();
						} else {
							if let Some(cursor_range) = last_cursor.clone() {
								let start = min(cursor_range.primary.index, cursor_range.secondary.index);
								let end = max(cursor_range.primary.index, cursor_range.secondary.index);
								let extended = match end as isize + extend {
        							// Check for overflow or negative result
        							value if value < 0 => 0,
        							value => value as usize,
    							};
								let cursor = Some(CCursorRange {
									primary : CCursor::new(start),
									secondary : CCursor::new(max(start, extended)),
								});
								output.state.set_ccursor_range(cursor.clone());
								output.state.store(ui.ctx(), output.response.id);
								*last_cursor = cursor.clone();
							}
						}
						

						//text_edit_output = Some(output);

						if history.len() < 1 {
							history.push(text.clone());
						}
		
						//if &current_tab.code != current_tab.history.last().expect("There should be an history") {
							//current_tab.history.push(current_tab.code.clone());
							//current_tab.saved = false;
							//if current_tab.history.len() > super::HISTORY_LENGTH {
								//current_tab.history.remove(0);
							//}
						//}
                    });
            });
        };
        if self.vscroll {
            let scroll_area = egui::ScrollArea::vertical()
                .id_source(format!("{}_outer_scroll", self.id))
                .stick_to_bottom(self.stick_to_bottom)
				.vertical_scroll_offset(vertical_offset.clone())
                .show(ui, code_editor);
			*vertical_offset = scroll_area.state.offset.y.clone();
        } else {
            code_editor(ui);
        }

        //text_edit_output.expect("TextEditOutput should exist at this point")
    }

	fn toggle_start_of_line(&self, cursor_range : CCursorRange, text : String, head : &str) -> (String, isize) {
		let mut substring = self.get_selection_substring(text.clone(), cursor_range.clone()).clone();
		let mut new_text : String = "".into();
		let extend : isize;
		
		if substring[1].contains(head) {
			extend = - self.delta_char(substring[1].clone(), head);
			substring[1] = substring[1].replace(&format!("\n{}", head), &"\n".to_string());
		} else {
			extend = self.delta_char(substring[1].clone(), head);
			substring[1] = substring[1].replace(&"\n".to_string(), &format!("\n{}", head));
		}
		new_text.push_str(&substring[0].clone());
		new_text.push_str(&substring[1].clone());
		new_text.push_str(&substring[2].clone());

		return (new_text, extend)
	}


	fn add_start_of_line(&self, cursor_range : CCursorRange, text : String, head : &str) -> (String, isize) {
		let mut substring = self.get_selection_substring(text.clone(), cursor_range.clone()).clone();
		let mut new_text : String = "".into();

		let extend : isize = self.delta_char(substring[1].clone(), head);
		substring[1] = substring[1].replace(&"\n".to_string(), &format!("\n{}", head));
		
		new_text.push_str(&substring[0].clone());
		new_text.push_str(&substring[1].clone());
		new_text.push_str(&substring[2].clone());

		return (new_text, extend)
	}

	fn remove_start_of_line(&self, cursor_range : CCursorRange, text : String, head : &str) -> (String, isize) {
		let mut substring = self.get_selection_substring(text.clone(), cursor_range.clone()).clone();
		let mut new_text : String = "".into();
		
		let extend : isize = - self.delta_char(substring[1].clone(), head);
		substring[1] = substring[1].replace(&format!("\n{}", head), &"\n".to_string());
		
		new_text.push_str(&substring[0].clone());
		new_text.push_str(&substring[1].clone());
		new_text.push_str(&substring[2].clone());

		return (new_text, extend)
	}

	fn get_selection_substring(&self, text : String, cursor_range : CCursorRange) -> Vec<String> {
		let start = min(cursor_range.primary.index, cursor_range.secondary.index);
		let end = max(cursor_range.primary.index, cursor_range.secondary.index);

		let mut first_char = max(0, start - 1);

		while first_char > 0 && text.char_at(first_char) != '\n' {
			first_char -= 1;
		}

		let last_char = end;
		
		return vec![text.slice(..first_char).to_string(), text.slice(first_char..last_char).to_string(), text.slice(last_char..).to_string()];
	}

	fn delta_char(&self, text : String, modifier: &str) -> isize {
		(modifier.len() * text.match_indices(&"\n".to_string()).collect::<Vec<_>>().len()) as isize
	}
}
