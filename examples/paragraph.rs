//! ## Demo
//!
//! `Demo` shows how to use tui-realm in a real case

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use std::time::Duration;

use tui_realm_stdlib::Paragraph;
use tuirealm::command::CmdResult;
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextSpan};
use tuirealm::terminal::TerminalBridge;
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent},
    Application, Component, Event, EventListenerCfg, MockComponent, NoUserEvent, Update,
};
// tui
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout};

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    ParagraphAlfa,
    ParagraphBeta,
}

struct Model {
    quit: bool,   // Becomes true when the user presses <ESC>
    redraw: bool, // Tells whether to refresh the UI; performance optimization
    app: Application<Id, Msg, NoUserEvent>,
}

impl Default for Model {
    fn default() -> Self {
        // Setup app
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default().default_input_listener(Duration::from_millis(10)),
        );
        assert!(app
            .mount(
                Id::ParagraphAlfa,
                Box::new(ParagraphAlfa::default()),
                vec![]
            )
            .is_ok());
        assert!(app
            .mount(
                Id::ParagraphBeta,
                Box::new(ParagraphBeta::default()),
                vec![]
            )
            .is_ok());
        // We need to give focus to input then
        assert!(app.active(&Id::ParagraphAlfa).is_ok());
        Self {
            quit: false,
            redraw: true,
            app,
        }
    }
}

impl Model {
    fn view(&mut self, terminal: &mut TerminalBridge) {
        let _ = terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(6),
                        Constraint::Length(6),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            self.app.view(&Id::ParagraphAlfa, f, chunks[0]);
            self.app.view(&Id::ParagraphBeta, f, chunks[1]);
        });
    }
}

fn main() {
    let mut model = Model::default();
    let mut terminal = TerminalBridge::new().expect("Cannot create terminal bridge");
    let _ = terminal.enable_raw_mode();
    let _ = terminal.enter_alternate_screen();
    // Now we use the Model struct to keep track of some states

    // let's loop until quit is true
    while !model.quit {
        // Tick
        if let Ok(messages) = model.app.tick(PollStrategy::Once) {
            for msg in messages.into_iter() {
                let mut msg = Some(msg);
                while msg.is_some() {
                    msg = model.update(msg);
                }
            }
        }
        // Redraw
        if model.redraw {
            model.view(&mut terminal);
            model.redraw = false;
        }
    }
    // Terminate terminal
    let _ = terminal.leave_alternate_screen();
    let _ = terminal.disable_raw_mode();
    let _ = terminal.clear_screen();
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.redraw = true;
        match msg.unwrap_or(Msg::None) {
            Msg::AppClose => {
                self.quit = true;
                None
            }
            Msg::None => None,
        }
    }
}

#[derive(MockComponent)]
struct ParagraphAlfa {
    component: Paragraph,
}

impl Default for ParagraphAlfa {
    fn default() -> Self {
        Self {
            component: Paragraph::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Yellow),
                )
                .foreground(Color::Yellow)
                .background(Color::Black)
                .title("Lorem ipsum (wrap)", Alignment::Center)
                .wrap(true)
                .text(&[
                    TextSpan::new("Lorem ipsum dolor sit amet,").underlined().fg(Color::Green),
                    TextSpan::from("consectetur adipiscing elit. Praesent mauris est, vehicula et imperdiet sed, tincidunt sed est. Sed sed dui odio. Etiam nunc neque, sodales ut ex nec, tincidunt malesuada eros. Sed quis eros non felis sodales accumsan in ac risus"),
                    TextSpan::from("                       Duis augue diam, tempor vitae posuere et, tempus mattis ligula.")
                ])
        }
    }
}

impl Component<Msg, NoUserEvent> for ParagraphAlfa {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

#[derive(MockComponent)]
struct ParagraphBeta {
    component: Paragraph,
}

impl Default for ParagraphBeta {
    fn default() -> Self {
        Self {
            component: Paragraph::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Cyan),
                )
                .foreground(Color::Cyan)
                .background(Color::Black)
                .title("Lorem ipsum (no wrap)", Alignment::Center)
                .wrap(false)
                .text(&[
                    TextSpan::new("Lorem ipsum dolor sit amet,").underlined().fg(Color::Green),
                    TextSpan::from("consectetur adipiscing elit. Praesent mauris est, vehicula et imperdiet sed, tincidunt sed est. Sed sed dui odio. Etiam nunc neque, sodales ut ex nec, tincidunt malesuada eros. Sed quis eros non felis sodales accumsan in ac risus"),
                    TextSpan::from("                                        Duis augue diam, tempor vitae posuere et, tempus mattis ligula.")
                ])
        }
    }
}

impl Component<Msg, NoUserEvent> for ParagraphBeta {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
