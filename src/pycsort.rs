mod lib;

use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use eframe::egui::{CtxRef, FontData, FontDefinitions, FontFamily};
use rfd::FileDialog;

use self::lib::{get_file_metadata, get_files};

pub const SPACING: f32 = 15.0;

pub struct PycSort {
    pub files: Vec<PathBuf>,
    pub folder: String,
    pub num_files: usize,
    pub progress: f32,
    pub status: String,
}

impl PycSort {
    pub fn new() -> PycSort {
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
        }
    }

    pub fn font_config(&self, ctx: &CtxRef) {
        // Get Font
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "Inter".to_string(),
            FontData::from_static(include_bytes!("../assets/fonts/Inter-VariableFont_slnt,wght.ttf")),
        );

        // Set Font
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 35.),
        );

        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 16.),
        );

        // Load Fonts
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Inter".to_string());

        ctx.set_fonts(font_def);
    }

    pub fn change_folder(&mut self) {
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

    pub fn rescan_folder(&mut self) {
        let updated_files = get_files(&self.folder);
        self.num_files = updated_files.len();
        self.files = updated_files;
        self.progress = 0.0;
    }

    pub fn sort_files(&mut self) {
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
            self.status = "Please delete or rename the existing `sorted` folder and rescan before sorting again".to_string();
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
