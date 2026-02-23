//Import crates used!
use std::fs;
use std::path::Path;

//Main function that runs at compile time
fn main() {
    //Define path varibles for the sources and destination folder of the screenshots
    let desktop = "/Users/jarrodlambert/Desktop";
    let screenshots_folder = "/Users/jarrodlambert/Desktop/Screenshots/";

    //Iterate through each of the files in the desktop folder
    for x in fs::read_dir(desktop).expect("Could Not Read Desktop") {
        //Assign current file to a varible
        let file = x.expect("Could Not Read File");

        //Check file exists and convert file name to a string
        if let Some(file_name_str) = file.file_name().to_str() {
            //Check if the file matches our screenshot format
            if file_name_str.starts_with("Screenshot") && file_name_str.ends_with(".png") {
                //If the file matches out screenshot format then copy it to our new folder
                // and delete it from the old folder
                println!("{}", file_name_str);
                let new_file_path = Path::new(screenshots_folder).join(file_name_str);
                println!("{:?}", new_file_path);
                fs::copy(file.path(), new_file_path).expect("Failed to copy file");
                fs::remove_file(file.path()).expect("Failed to remove file");
            }
        }
        // println!("{:?}", file);
    }
}
