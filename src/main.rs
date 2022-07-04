/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use iced::{
    button, window, Alignment, Button, Column, Element, Length, ProgressBar, Row, Sandbox,
    Settings, Text,
};
use pycsort_rs::{get_file_metadata, get_files};
use rfd::FileDialog;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

pub fn main() -> iced::Result {
    PycSort::run(Settings {
        window: window::Settings {
            size: (625, 420),
            resizable: false,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct PycSort {
    files: Vec<PathBuf>,
    folder: String,
    num_files: usize,
    progress: f32,
    status: String,
    change_btn: button::State,
    sort_btn: button::State,
    rescan_btn: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ChangePressed,
    SortPressed,
    RescanPressed,
}

impl Sandbox for PycSort {
    type Message = Message;

    fn new() -> Self {
        let home_folder = home::home_dir().expect("Could not find home");
        let folder = home_folder
            .as_os_str()
            .to_str()
            .expect("Could not parse home folder")
            .to_string()
            + "/Pictures";
        let files = get_files(&String::from(&folder));

        PycSort {
            num_files: files.len(),
            progress: 0.0,
            status: "Sorted 0 pictures/videos".to_string(),
            files,
            folder,
            change_btn: button::State::default(),
            sort_btn: button::State::default(),
            rescan_btn: button::State::default(),
        }
    }

    fn title(&self) -> String {
        String::from("PycSort RS")
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::ChangePressed => {
                self.change_folder();
            }
            Message::SortPressed => {
                self.sort_files();
            }
            Message::RescanPressed => {
                self.rescan_folder();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .width(Length::Fill)
            .padding(20)
            .spacing(40)
            .align_items(Alignment::Center)
            .push(
                Row::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Start)
                    .push(Text::new("PycSort RS").size(48)),
            )
            .push(
                Column::new()
                    .align_items(Alignment::Center)
                    .spacing(12)
                    .push(Text::new(
                        "Found ".to_owned()
                            + &self.num_files.to_string()
                            + " pictures/videos in "
                            + &self.folder.split('/').last().unwrap(),
                    ))
                    .push(
                        Button::new(&mut self.change_btn, Text::new("Change Folder"))
                            .on_press(Message::ChangePressed)
                            .padding(12)
                            .style(style::Button::Action),
                    ),
            )
            .push(
                Column::new()
                    .align_items(Alignment::Center)
                    .spacing(12)
                    .push(Text::new(&self.status))
                    .push(
                        Row::new()
                            .spacing(12)
                            .push(
                                Button::new(&mut self.sort_btn, Text::new("Sort Files"))
                                    .on_press(Message::SortPressed)
                                    .padding(12)
                                    .style(style::Button::Action),
                            )
                            .push(
                                Button::new(&mut self.rescan_btn, Text::new("Rescan Folder"))
                                    .on_press(Message::RescanPressed)
                                    .padding(12)
                                    .style(style::Button::Default),
                            ),
                    ),
            )
            .push(ProgressBar::new(0.0..=1.0, self.progress))
            .into()
    }
}

impl PycSort {
    fn change_folder(&mut self) {
        if let Some(path) = FileDialog::new().set_directory(&self.folder).pick_folder() {
            let folder = path
                .as_os_str()
                .to_str()
                .expect("Could not parse path")
                .to_string();

            let updated_files = get_files(&folder);
            self.status = "Sorted 0 pictures/videos".to_string();
            self.folder = folder;
            self.num_files = updated_files.len();
            self.files = updated_files;
        }
    }

    fn rescan_folder(&mut self) {
        let updated_files = get_files(&self.folder);
        self.num_files = updated_files.len();
        self.files = updated_files;
        self.progress = 0.0;
    }

    fn sort_files(&mut self) {
        self.progress = 0.0;

        let mut md_vec: Vec<String> = Vec::new();

        self.status = "Gathering Metadata".to_string();

        self.progress += 1.0 / (self.num_files as f32);

        // Put Metadata (Date Created) into a Vector
        for file in &self.files {
            let file_md = get_file_metadata(file);
            md_vec.push(file_md);
            self.progress += 1.0 / (self.num_files as f32);
        }

        // Reset Progress
        self.progress = 0.0;
        self.status = "Sorting Files".to_string();

        /*
        Check if sorted folder exists
        If it does, prompt user
        */
        if fs::read_dir(String::from(&self.folder) + &"/sorted".to_string()).is_ok() {
            self.status =
                "Please delete or rename the existing `sorted` folder and rescan before sorting again."
                    .to_string();
        } else {
            // Create Sorting Folder
            let sorted_folder = String::from(&self.folder) + "/sorted";
            fs::create_dir(&sorted_folder).expect("Could not create dir");

            // Create Log File
            let mut log_file = fs::File::create(String::from(&sorted_folder) + "/sort_log.txt")
                .expect("Could not create file");
            let mut log_string = String::new();

            // Start Sorting
            self.progress += 1.0 / (self.num_files as f32);
            for (i, _) in md_vec.iter().enumerate().take(self.files.len()) {
                let file = &self.files[i];

                let file_name = file
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .split('/')
                    .last()
                    .unwrap()
                    .to_string();

                let dest_folder = String::from(&sorted_folder) + "/" + &md_vec[i][..7];

                let dest = String::from(&dest_folder) + "/" + &file_name;

                let res = self.copy_file(&dest, &dest_folder, file);
                self.progress += 1.0 / (self.num_files as f32);
                let _ = &log_string.push_str(&res);
                self.status = res;
            }

            log_file
                .write_all(log_string.as_bytes())
                .expect("Could not write to log file");

            self.status = "Sorted ".to_owned() + &self.num_files.to_string() + " pictures/videos";
        }
    }

    fn copy_file(&self, dest: &str, dest_folder: &str, file: &Path) -> String {
        let mut res: String = String::from(file.to_str().unwrap()) + " -> " + dest + "\n";

        if fs::read(dest).is_err() {
            // If file does not exist in sorted folder, copy
            if fs::copy(file, &dest).is_err() {
                // Create All Missing Dirs
                fs::create_dir_all(&dest_folder).expect("Could not create directory");

                // Copy
                fs::copy(file, &dest).expect("Still can't copy");
            };
        } else {
            // Create a new destination (filename + (_parent folder) + .ext)
            let split_dest: Vec<&str> = dest.split('.').collect();
            let split_file: Vec<&str> = file.to_str().unwrap().split('/').collect();
            let new_dest = String::from(split_dest[0])
                + " (_"
                + split_file[split_file.len() - 2]
                + ")."
                + split_dest.last().unwrap();

            if fs::copy(file, &new_dest).is_err() {
                // Create All Missing Dirs
                fs::create_dir_all(&dest_folder).expect("Could not create directory");

                // Copy
                fs::copy(file, &new_dest).expect("Still can't copy");
                res = String::from(file.to_str().unwrap()) + " -> " + &new_dest + "\n";
            }
        }

        res
    }
}

mod style {
    use iced::{button, Background, Color};

    pub enum Button {
        Action,
        Default,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::Action => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.984, 0.573, 0.235))),
                    border_radius: 10.0,
                    text_color: Color::BLACK,
                    ..button::Style::default()
                },
                Button::Default => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.118, 0.161, 0.231))),
                    border_radius: 10.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                },
            }
        }
    }
}
