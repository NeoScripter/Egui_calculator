#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{
    egui::{self, Event, ViewportCommand},
};
use egui::{FontFamily, FontId, RichText, TextStyle, Color32};

fn set_global_text_color_to_white(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // Set the global text color override to white
    style.visuals.override_text_color = Some(egui::Color32::WHITE);
        let white_stroke = egui::Stroke::new(2.0, egui::Color32::WHITE); // You can adjust the width (2.0) as needed
    style.visuals.widgets.active.bg_stroke = white_stroke;
    style.visuals.widgets.hovered.bg_stroke = white_stroke;
    style.visuals.widgets.noninteractive.bg_stroke = white_stroke;

    // Set the text cursor color
    style.visuals.text_cursor = white_stroke;

    // Apply the modified style back to the context
    ctx.set_style(style);
}
fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (TextStyle::Body, FontId::new(35.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(32.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
fn custom_window_frame(ctx: &egui::Context, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::*;

    let panel_frame = egui::Frame {
        fill: Color32::from_rgb(16, 20, 26),
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, title_bar_rect, title);

        // Add the contents:
        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}
fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
    } else if title_bar_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui);
        });
    });
}
fn close_maximize_minimize(ui: &mut egui::Ui) {
    use egui::{Button, RichText};

    let button_height = 20.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)));
    if close_response.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    if is_maximized {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)));
        if maximized_response.clicked() {
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(false));
        }
    } else {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)));
        if maximized_response.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
        }
    }

    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)));
    if minimized_response.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}
struct Calculator {
    user_input: String,
    equal_pressed: bool,
    button_click: u32,
}
impl Calculator {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_text_styles(&cc.egui_ctx);
        set_global_text_color_to_white(&cc.egui_ctx);
        Calculator {
            user_input: String::new(),
            equal_pressed: false,
            button_click: 0,
        }
    }
    fn check_input(&mut self, digit: &str) {
        let values = ['-', '+', '*', '/', '.'];
        let digit_char = digit.chars().next().unwrap();
        let last = self.user_input.chars().last();
        let sec_to_last = self.user_input.chars().rev().nth(1);
        let len = self.user_input.len();
        match digit_char {
            '1'..='9' => match last {
                Some(last_char) => match last_char {
                    ')' => {},
                    _ => self.user_input.push(digit_char),
                }
                None => self.user_input.push(digit_char),
            },
            '-' if len == 0 => self.user_input.push(digit_char),
            '-' => match sec_to_last {
                Some(second_last_char) => match last {
                    Some(last_char) => match last_char {
                        '.' => {},
                        _ if values.contains(&second_last_char) && values.contains(&last_char) => {},
                        _ => self.user_input.push(digit_char),
                    },
                    None => self.user_input.push(digit_char),
                },
                None => self.user_input.push(digit_char),
            }
            '+'|'*'|'/' if len > 0 => match last {
                Some(last_char) => match last_char {
                    '-'|'+'|'*'|'/'|'.' => {},
                    _ => self.user_input.push(digit_char),
                }
                None => self.user_input.push(digit_char),
            },
            '0' => match last {
                Some(last_char) => match last_char {
                    ')' => {},
                    _ => self.user_input.push(digit_char),
                }
                None => {},
            },
            '.' => match last {
                Some(last_char) => match last_char {
                    '0'..='9' => self.user_input.push(digit_char),
                    _ => {},
                }
                None => {},
            },
            _ => {},
        }
    }    
    fn is_input_valid(&self) -> bool {
        let values = ['-', '+', '*', '/', '.'];
        let op_br = self.user_input.chars().filter(|&c| c == '(').count();
        let cl_br = self.user_input.chars().filter(|&c| c == ')').count();
        let len = self.user_input.len();
        let sec_to_last = self.user_input.chars().rev().nth(1);
        let th_to_last = self.user_input.chars().rev().nth(2);
        match self.user_input.chars().last() {
            Some(last_char) => {
                match last_char {
                    '1'..='9' => sec_to_last.map_or(true, |c| c != ')'),
                    '-' => match th_to_last {
                        Some(third_to_last) => match sec_to_last {
                            Some(second_last_char) => match second_last_char {
                                '.' => false,
                                _ if values.contains(&third_to_last) && values.contains(&second_last_char) => false,
                                _ => true,
                            }
                            None => true,
                        }
                        None => true,
                    },
                    '(' => sec_to_last
                             .map(|c| !c.is_digit(10) && c != ')' && c != '.')
                             .unwrap_or(true),
                    ')' => sec_to_last
                             .map(|c| (c.is_digit(10) || c == ')') && c != '.' && cl_br <= op_br)
                             .unwrap_or(false),
                    '0' => match sec_to_last {
                            Some(second_last_char) => match second_last_char {
                                '-' if len > 1 => true,
                                ')' => false,
                                _ => true,
                            },
                            None => true,
                        },
                    '.' => match sec_to_last {
                        Some(second_last_char) => match second_last_char {
                            '0'..='9' => true,
                            _ => false,
                        },
                        None => false,
                    }
                    _ if values.contains(&last_char) => sec_to_last
                        .map(|c| c.is_digit(10) || c == ')')
                        .unwrap_or(false),
                    _ => false,
                }
            },
            None => true,
        }
    }    
    fn open_bracket(&mut self) {
        if self.user_input.is_empty() {
            self.user_input.push('(');
        } else {
            if let Some(last_char) = self.user_input.chars().last() {
                match last_char {
                    '.'|')'|'0'..='9' => {},
                    _ => self.user_input.push('('),
                }
            }
        }
    }
    fn close_bracket(&mut self) {
        let op_br = self.user_input.chars().filter(|&c| c == '(').count();
        let cl_br = self.user_input.chars().filter(|&c| c == ')').count();
        if let Some(last_char) = self.user_input.chars().last() {
            match last_char {
                ')'|'0'..='9' if cl_br < op_br => self.user_input.push(')'),
                _ => {},
            }
        }
    }
    fn correct_input(&self) -> bool {
        let op_br = self.user_input.chars().filter(|&c| c == '(').count();
        let cl_br = self.user_input.chars().filter(|&c| c == ')').count();
        if cl_br == op_br {
            if let Some(last_char) = self.user_input.chars().last() {
                if last_char.is_digit(10) || last_char == ')' {
                    return true
                } else {
                    return false
                }
            }
            false
        } else {
            false
        }

    }
}
impl Default for Calculator {
    fn default() -> Self {
        Self {
            user_input: String::new(),
            equal_pressed: false,
            button_click: 0,
        }
    }
}

impl eframe::App for Calculator {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.0);
        custom_window_frame(ctx, "", |ui| {
                ui.add_space(30.0);
                let id = egui::Id::new("text_edit").with(self.button_click);
                ui.style_mut().visuals.extreme_bg_color = egui::Color32::BLACK; // Set the desired dark background color
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.user_input)
                        .desired_width(370.0)
                        .id(id)
                );
                response.request_focus();
                ctx.input(|i| {
                    for event in i.events.iter() {
                        if let Event::Text(_text) = event {
                            if let Some(_last_char) = self.user_input.chars().last() {
                                if !self.is_input_valid() {
                                    self.user_input.pop();
                                }
                            }
                        }
                    }
                });
                let contents = [
                    "CE", "CLR", "(", ")",
                    "7", "8", "9", "/",
                    "4", "5", "6", "*",
                    "1", "2", "3", "-",
                    "0", ".", "=", "+"
                ]; // This array holds the content for each cell
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    // Add space to the left of the grid
                    ui.add_space(10.0);
                egui::Grid::new("grid").show(ui, |ui| {
                    let button_size = [82.0, 40.0]; // Set the desired size for the buttons

                    for (index, content) in contents.iter().enumerate() {
                        let button = egui::Button::new(*content)
                        .fill(Color32::from_rgb(17, 41, 58));
                        if ui.add_sized(button_size, button).clicked() {
                            match *content {
                                "=" => {
                                    if self.correct_input() {
                                        self.equal_pressed = true;
                                    }
                                },
                                "CE" => {self.user_input.pop();},
                                "CLR" => {self.user_input.clear();},
                                "(" => {self.open_bracket();
                                        self.button_click += 1;
                                },
                                ")" => {self.close_bracket();
                                    self.button_click += 1;
                            },
                                _ => {
                                    self.check_input(*content);
                                    self.button_click += 1;
                                },
                            }
                        }
                
                        if (index + 1) % 4 == 0 {
                            ui.end_row(); // End the row after every 4th item
                        }
                    }
                });
            });
                if (ui.input(|i| i.key_pressed(egui::Key::Enter)) && self.correct_input()) || self.equal_pressed {
                    if let Ok(result) = calculate(&self.user_input) {
                            self.user_input = format!("{}", result);
                    }
                    self.equal_pressed = false;
                }
            });
    }    
}

fn calculate(expr: &str) -> Result<f64, String> {
    meval::eval_str(expr)
        .map_err(|e| format!("Calculation error: {:?}", e))
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()>  {  
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false) // Hide the OS-specific "chrome" around the window
            .with_inner_size([500.0, 500.0])
            .with_min_inner_size([500.0, 500.0])
            .with_transparent(true), // To have rounded corners we need transparency
    
        ..Default::default()
    };
    eframe::run_native(
        "Rust-calculator", 
        options,
        Box::new(|cc| Box::new(Calculator::new(cc))),
    )
}  
#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    let web_options = eframe::WebOptions::default();

    let web_runner = eframe::WebRunner::new(); // Create a WebRunner instance

    wasm_bindgen_futures::spawn_local(async move {
        web_runner.start(
            "canvas",
            web_options,
            Box::new(|cc| Box::new(Calculator::new(cc))) // Replace Calculator::new(cc) with the appropriate initialization
        )
        .await
        .expect("failed to start eframe");
    });
}

