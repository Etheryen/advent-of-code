use std::{fmt::Display, time::Instant};

struct OptionsDisplay(Vec<Option<u64>>);

impl Display for OptionsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.0 {
            match item {
                Some(id) => write!(f, "{}", id)?,
                None => write!(f, ".")?,
            };
        }

        Ok(())
    }
}

struct FilesDisplay(Vec<u64>);

impl Display for FilesDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for id in &self.0 {
            write!(f, "{}", id)?;
        }

        Ok(())
    }
}

fn to_disk_items(line: &str) -> Vec<Option<u64>> {
    let mut curr_file_id = 0;
    let mut disk_items = Vec::new();

    let mut is_file = true;

    for character in line.chars() {
        if !character.is_ascii_digit() {
            break;
        }

        let size = character.to_digit(10).unwrap();
        if size == 0 {
            is_file = !is_file;
            continue;
        }

        if is_file {
            for _ in 0..size {
                disk_items.push(Some(curr_file_id));
            }
            curr_file_id += 1;
        } else {
            for _ in 0..size {
                disk_items.push(None);
            }
        }

        is_file = !is_file;
    }

    disk_items
}

fn is_finished(files: &Vec<u64>, disk_items: &Vec<Option<u64>>) -> bool {
    let mut files_clone = files.clone();
    files_clone.sort();

    let mut disk_items_clone = disk_items
        .clone()
        .into_iter()
        .filter_map(|item| item)
        .collect::<Vec<_>>();
    disk_items_clone.sort();

    files_clone == disk_items_clone
}

fn to_packed_files(disk_items: Vec<Option<u64>>) -> Vec<u64> {
    let mut files = Vec::new();

    let mut rev_files_iter = disk_items
        .clone()
        .into_iter()
        .enumerate()
        .filter_map(|(index, item)| item.map(|id| (index, id)))
        .rev();

    let mut stop_index = disk_items.len() - 1;

    let disk_items_clone = disk_items.clone();

    for (index, item) in disk_items.into_iter().enumerate() {
        if is_finished(&files, &disk_items_clone) {
            break;
        }
        if let Some(id) = item {
            files.push(id);
            continue;
        }

        let (new_stop_index, id) = rev_files_iter.next().unwrap();
        stop_index = new_stop_index;
        files.push(id);
    }

    files
}

fn to_checksum(files: Vec<u64>) -> u64 {
    files
        .into_iter()
        .enumerate()
        .map(|(index, id)| index as u64 * id)
        .sum()
}

fn solve(line: &str) -> u64 {
    let disk_items = to_disk_items(line);
    println!("before    - {}", OptionsDisplay(disk_items.clone()));
    let files = to_packed_files(disk_items);
    println!("after     - {}", FilesDisplay(files.clone()));
    print!("should be - ");
    println!("0099811188827773336446555566");

    to_checksum(files)
}

fn main() {
    let lines = include_str!("input.txt");
    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
