/*
 *        -=[ N O T E B O X ]=-
 *               v.0.1.0
 *     Copyright © SpielProfessor
 *                2024
 *              ********
 *           (Rust  edition)
 *
 **********************
 *
 * LICENSE:
 *  Copyright 2024 Aaron Vollmar
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 **********************
 *
 * INFO:
 *  This program is the REPLACEMENT for the "C edition" of NoteBox (github.com/spielprofessor/notebox)
 *  which won't receive updates after this programs' release
 *
 **********************
 *
 * ABOUT:
 *  NoteBox is a ToDos App written in Rust (originally in C) with the Iced GUI toolkit (originally raylib/raygui)
 **********************
 *
 * CURRENT FEATURES:
 *  - A GUI
 *  - Adding/editing/removing/(un)checking todos
 *  - Saving ToDos
 **********************
 *
 * TODOS/FUTURE FEATURES:
 *  + Notes view with cards
 *  + A calendar app
 *  + Notifications & Timers of todos
 *  ~ View for all/finished/unfinished todos -> TODO: move to top
 *  + Settings view
 *  + Better styling
 *  + Icon
 */

// -=[ I M P O R T S ]=-
use iced::widget::Container;
use iced::Pixels;
use iced::alignment::{Horizontal, Vertical};
use iced::{Font, Length, Size, Settings};
use iced::widget::{button, checkbox, Column, column, row, text, text_input, container};
use iced::window;
use native_dialog::{FileDialog, MessageDialog, MessageType};
use iced_test::{Todo, save, open};

// Messages/Signals
#[derive(Debug, Clone)]
enum Signal {
    TextInputChanged(String),
    Submit,
    Delete(usize),
    Edit(usize),
    Check(usize),
    TodoTextInputChanged(String, usize),
    Save,
    Load,
    OnModeAll,
    OnModeChecked,
    OnModeUnchecked,
    DeleteFinished
}

// Program state
#[derive(Default)]
pub struct NoteBox {
    text: String,
    todos: Vec<Todo>,
    last_saved_todos: Vec<Todo>,
    save_path: String,
    already_saved: bool,
    mode: usize, // mode 0: todos, mode 1: notes, mode 2: calendar
    current_mode_todo: i32,
}

// Fonts
const MESLO: Font = Font::with_name("MesloLGLDZ Nerd Font Mono");

// Main Program
impl NoteBox {
    // Save todos
    fn save(&mut self) {
        let path = FileDialog::new()
            .add_filter("NoteBox .ron ToDo file", &["ron", "nb", "txt"])
            .add_filter("All files", &["*"])
            .show_save_single_file()
            .unwrap();

        match path {
            Some(path) => {
                save(path.display().to_string(), &mut self.todos);
                self.already_saved=true;
                self.save_path=path.display().to_string();
            },
            None => ()
        };
    }
    // open/load todos
    fn load(&mut self) {

        let path = FileDialog::new()
            .add_filter("NoteBox .ron ToDo file", &["ron", "nb", "txt"])
            .add_filter("All files", &["*"])
            .show_open_single_file()
            .unwrap();

        match path {
            Some(path) => {
                self.todos=open(path.display().to_string());
                self.already_saved=true;
                self.save_path=path.display().to_string();
            },
            None => ()
        };
    }
    // view function: display stuff
    fn view(&self) -> Column<Signal> {
        let mut gui:Column<Signal>=column![
            row![
                button("").on_press(Signal::Save),
                button("󰝰").on_press(Signal::Load),
                Container::new(button("󰩺  Delete finished").on_press(Signal::DeleteFinished)).align_x(Horizontal::Right).width(Length::Fill)
            ],
                text("NoteBox").horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill).size(69)
                    .color([0.5, 0.5, 0.5])
                    .height(Length::Fill)
                    .vertical_alignment(Vertical::Bottom),
                text("Todos. Refined.").horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill).size(20)
                    .color([0.3, 0.3, 0.3])
                    .height(Length::Fill)
                    .vertical_alignment(Vertical::Top)
        ].padding(10);
        let mut scroll_view:Column<Signal> = column![];
        // all the todos
        for i in 0..self.todos.len() {
            if self.current_mode_todo==0 || (self.current_mode_todo==1 && self.todos[i].checked) || (self.current_mode_todo==2 && !self.todos[i].checked) {
                if !self.todos[i].editing {
                    scroll_view = scroll_view.push(
                        row![
                        checkbox("", self.todos[i].checked).on_toggle(move |_| Signal::Check(i)),
                        text(self.todos[i].text.clone()).width(Length::Fill),
                        button("󰲶").on_press(Signal::Edit(i)),
                        button("󰩹").on_press(Signal::Delete(i)),
                    ],
                    );
                } else {
                    scroll_view = scroll_view.push(
                        row![
                        checkbox("", self.todos[i].checked).on_toggle(move |_| Signal::Check(i)),
                        text_input("Insert your todo here", &self.todos[i].text.clone()).on_submit(Signal::Edit(i)).on_input(move |string_input| Signal::TodoTextInputChanged(string_input.clone(), i)).width(Length::Fill),
                        button("").on_press(Signal::Edit(i)),
                        button("󰩹").on_press(Signal::Delete(i)),
                    ],
                    );
                }
            }
        }
        gui=gui.push(scroll_view);
        // different modes for notes
        let mut filter_controls=row![];
        if self.current_mode_todo!=0 {
            filter_controls=filter_controls.push(button("All").on_press(Signal::OnModeAll));
        } else {
            filter_controls=filter_controls.push(button("All"));
        }
        if self.current_mode_todo!=1 {
            filter_controls=filter_controls.push(button("Checked").on_press(Signal::OnModeChecked));
        } else {
            filter_controls=filter_controls.push(button("Checked"));
        }
        if self.current_mode_todo!=2 {
            filter_controls=filter_controls.push(button("Unchecked").on_press(Signal::OnModeUnchecked));
        } else {
            filter_controls=filter_controls.push(button("Unchecked"));
        }
        gui=gui.push(

            Container::new(filter_controls).center_x(Length::Fill).padding(30)
        );
        gui=gui.push(row![
                text_input("Insert your todo", &self.text).on_input(Signal::TextInputChanged).on_submit(Signal::Submit),
                button("Submit").on_press(Signal::Submit),

            ]);
        return gui;
    }



    // update function
    fn update(&mut self, message: Signal) {
        match message {
            Signal::TextInputChanged(str) => self.text=str,
            Signal::Submit => {
                self.todos.push(Todo {
                    text: self.text.clone(),
                    checked: false,
                    editing: false,
                    children: vec![]
                    }
                );
                self.text=String::new();
            },
            Signal::Delete(id) => {self.todos.remove(id);},
            Signal::Edit(id) => {
                self.todos[id].editing=!self.todos[id].editing;
            },
            Signal::Check(id) => {
                self.todos[id].checked=!self.todos[id].checked;
            },
            Signal::TodoTextInputChanged(str, id) => {
                self.todos[id].text=str;
            },
            Signal::Save => self.save(),
            Signal::Load => self.load(),
            Signal::OnModeAll => {
                self.current_mode_todo=0;
            },
            Signal::OnModeChecked => {
                self.current_mode_todo=1;
            },
            Signal::OnModeUnchecked => {
                self.current_mode_todo=2;
            },
            Signal::DeleteFinished => {
                let mut i =0;
                while i<self.todos.len(){
                    if self.todos[i].checked {
                        self.todos.remove(i);
                    }
                    i+=1;
                }
            }
        }
    }


    fn update_controller(&mut self, message: Signal) {
        if self.mode==0 {
            self.update(message);
        }
    }

    fn view_controller(&self) -> Column<Signal> {
        let mut out:Column<Signal> = column![text("lol, youve got a weird mode!")];
        if self.mode==0 {
            out=self.view();
        }
        return out;
    }
}

// main function
fn main(){
    let settings=Settings {
        id: None,
        window: window::Settings {
            size: Size::new(800.0, 450.0),
            position: Default::default(),
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            level: Default::default(),
            icon: None,
            platform_specific: Default::default(),
            exit_on_close_request: true,
        },
        flags: (),
        fonts: vec![],
        default_font: Default::default(),
        default_text_size: Pixels(16.0),
        antialiasing: false,
    };
    let _ = iced::program("NoteBox(.rs): Todos. Refined. v1.0", NoteBox::update_controller, NoteBox::view_controller).settings(settings).font(include_bytes!("../fonts/meslo.ttf").as_slice()).default_font(MESLO).run();
    println!("Closing program!");
}
