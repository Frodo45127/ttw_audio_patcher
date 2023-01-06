use std::{path::PathBuf, collections::{HashSet, hash_map::DefaultHasher}, hash::Hash, io::Read, env::args, process::exit};

use walkdir::WalkDir;

fn main() {

    // Args:
    // - 0: program.
    // - 1: folder with voices in english
    // - 2: folder with voices in another language
    let args = args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Error: 2 args required but you provided {}.", args.len() - 1);
        println!("The program should be executed like this: ttw_audio_patcher.exe 'path_of_the_english_voice_files' 'path_of_the_non_english_voice_files'");
        exit(1);
    }

    let voice_folder_english = PathBuf::from(&args[1]);
    let voice_folder_non_english = PathBuf::from(&args[2]);

    if !voice_folder_english.is_dir() {
        println!("Invalid english voice folder.");
        exit(1);
    }

    if !voice_folder_non_english.is_dir() {
        println!("Invalid non-english voice folder.");
        exit(1);
    }

    // TODO: unhardcode this.
    //let voice_folder_non_english = PathBuf::from("C:/Users/frodo/AppData/Local/ModOrganizer/TTW/mods/TTW 3.3 Español (Voces)");
    //let voice_folder_english = PathBuf::from("C:/Users/frodo/AppData/Local/ModOrganizer/TTW/mods/TTW Voices (English)");

    // Cache all the file paths. Otherwise this takes ages.
    let english_paths = WalkDir::new(&voice_folder_english).follow_links(false)
        .into_iter()
        .map(|result| result.map(|entry| entry.path().to_path_buf()))
        .collect::<Vec<_>>();

    let non_english_paths = WalkDir::new(&voice_folder_non_english).follow_links(false)
        .into_iter()
        .filter_map(|entry| entry.ok()
            .map(|entry| entry.path().to_path_buf()))
        .filter(|path| path.is_file())
        .map(|path| (path.file_name().unwrap().to_string_lossy().to_string(), path))
        .collect::<Vec<_>>();

    let names = non_english_paths.iter().map(|(file_name, _)| file_name).collect::<HashSet<_>>();

    let mut move_count = 0;
    let mut not_found = 0;

    for (index, english_path) in english_paths.iter().enumerate() {
        if index % 500 == 0 {
            println!("Files processed: {}.", index);
        }

        match english_path {
            Ok(english_path) => {
                if english_path.is_file() {
                    let file_name = english_path.file_name().unwrap().to_string_lossy().to_string();
                    let mut file_moved = false;

                    // Do not even do a pass if it's a file we don't have in our language.
                    if names.contains(&file_name) {
                        let stripped_yes = english_path.strip_prefix(&voice_folder_english).unwrap();

                        let paths_matching = non_english_paths.iter().filter(|(non_english_file_name, _)| non_english_file_name == &file_name).collect::<Vec<_>>();
                        let exact_match = paths_matching.iter().find(|(_, path)| stripped_yes == path.strip_prefix(&voice_folder_non_english).unwrap());

                        // Simple move: only one match.
                        if exact_match.is_none() && paths_matching.len() == 1 {
                            let non_english_path =  &paths_matching[0].1;
                            let stripped_non = non_english_path.strip_prefix(&voice_folder_non_english).unwrap();

                            if stripped_yes != stripped_non {
                                let new_path = voice_folder_non_english.join(&stripped_yes);
                                let new_path_parent = new_path.parent().unwrap();

                                println!("Copying: {} to {}", non_english_path.display(), new_path_parent.display());

                                file_moved = true;
                                move_count += 1;

                                if !new_path_parent.is_dir() {
                                    std::fs::create_dir_all(&new_path_parent).unwrap();
                                }

                                std::fs::copy(non_english_path, new_path).unwrap();
                            } else {
                                //println!("Already on correct path, not moving.");
                            }
                        }

                        // Multiple matches.
                        else if exact_match.is_none() && paths_matching.len() > 1 {
                            let mut found = false;
                            for (_, non_english_path) in &paths_matching {
                                let stripped_non = non_english_path.strip_prefix(&voice_folder_non_english).unwrap();
                                let non_english_path_parent = stripped_non.parent().unwrap();
                                let english_path_parent = stripped_yes.parent().unwrap();

                                // If path began with the other one, it's a small move. Ignore the esm, as there are about 1000 files with just the esm changed.
                                let yes_parent_no_esm = english_path_parent.to_string_lossy().replace("\\", "/").split('/').collect::<Vec<_>>()[3..].join("/");
                                let non_parent_no_esm = non_english_path_parent.to_string_lossy().replace("\\", "/").split('/').collect::<Vec<_>>()[3..].join("/");

                                // Equal lenght: only the ESM has changed.
                                // +2 lenght: moved from folderX to folderXdc.
                                if yes_parent_no_esm.starts_with(&non_parent_no_esm) && (yes_parent_no_esm.len() == non_parent_no_esm.len() + 2 || yes_parent_no_esm.len() == non_parent_no_esm.len()) {
                                    let new_path = voice_folder_non_english.join(&stripped_yes);
                                    let new_path_parent = new_path.parent().unwrap();

                                    println!("Copying: {} to {}", non_english_path.display(), new_path_parent.display());

                                    file_moved = true;
                                    move_count += 1;

                                    if !new_path_parent.is_dir() {
                                       std::fs::create_dir_all(&new_path_parent).unwrap();
                                    }

                                    std::fs::copy(non_english_path, new_path).unwrap();
                                    found = true;
                                    break;
                                }
                            }

                            // These ones left are files that match multiple files, but we have no idea which one is the correct.
                            // So we hash them and see if we can get them by hash.
                            if !found {

                                let mut data = vec![];
                                let mut hasher = DefaultHasher::new();
                                let mut file = std::fs::File::open(&english_path).unwrap();
                                file.read_to_end(&mut data).unwrap();
                                let non_hash = data.hash(&mut hasher);

                                for (_, non_english_path) in &paths_matching {
                                    let mut data = vec![];
                                    if non_english_path.is_file() {
                                        let mut file = std::fs::File::open(non_english_path).unwrap();
                                        file.read_to_end(&mut data).unwrap();
                                        let hash = data.hash(&mut hasher);

                                        // If hashes match, we COPY the file. Not move it.
                                        if hash == non_hash {
                                            let new_path = voice_folder_non_english.join(&stripped_yes);
                                            let new_path_parent = new_path.parent().unwrap();

                                            println!("File matched through hash: {} with {}.", non_english_path.display(), stripped_yes.display());

                                            file_moved = true;
                                            move_count += 1;
                                            found = true;

                                            if !new_path_parent.is_dir() {
                                                std::fs::create_dir_all(&new_path_parent).unwrap();
                                            }

                                            std::fs::copy(non_english_path, new_path).unwrap();
                                            break;
                                        }
                                    }
                                }

                                // If still hasn't been found, it's a weird one.
                                if !found {
                                    println!("Path matched no success: {}", english_path.display());
                                    println!("Fail: {}", stripped_yes.display());
                                    not_found += 1;
                                }
                            }
                        }
                    }

                    if !file_moved {
                        //println!("Match not found for {}", english_path.display());
                    }
                }
            }
            Err(error) => println!("Path failed: {}", error.path().unwrap().display()),
        }
    }

    println!("Moved/copied files: {} of {}, total english paths: {}, not found: {}", move_count, non_english_paths.len(), english_paths.len(), not_found);
}
