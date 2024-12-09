use std::{fmt::Display, time::Instant, vec};

#[derive(Clone, Debug)]
struct DiskSpace {
    size: u64,
    space: Vec<u64>,
}

impl DiskSpace {
    fn available_space(&self) -> u64 {
        self.size - self.space.len() as u64
    }
}

struct DiskDisplay(Vec<DiskSpace>);

impl Display for DiskDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.0 {
            for i in 0..item.size {
                match item.space.get(i as usize) {
                    Some(id) => write!(f, "{}", id)?,
                    None => write!(f, ".")?,
                };
            }
        }

        Ok(())
    }
}

fn to_disk(line: &str) -> Vec<DiskSpace> {
    let mut curr_file_id = 0;
    let mut disk_items = Vec::new();

    let mut is_file = true;

    for character in line.chars() {
        if !character.is_ascii_digit() {
            break;
        }

        let size = character.to_digit(10).unwrap() as u64;
        if size == 0 {
            is_file = !is_file;
            continue;
        }

        if is_file {
            disk_items.push(DiskSpace {
                size,
                space: vec![curr_file_id; size as usize],
            });
            curr_file_id += 1;
        } else {
            disk_items.push(DiskSpace {
                size,
                space: vec![],
            });
        }

        is_file = !is_file;
    }

    disk_items
}

fn move_if_can(disk: &mut Vec<DiskSpace>, space_to_move: DiskSpace, space_to_move_index: usize) {
    for (move_index, disk_space) in disk.clone().iter().enumerate() {
        if space_to_move_index == move_index {
            break;
        }
        if disk_space.available_space() >= space_to_move.size {
            for _ in 0..space_to_move.size {
                disk[move_index].space.push(space_to_move.clone().space[0]);
            }
            disk[space_to_move_index].space = vec![];
            break;
        }
    }
}

fn pack_disk(disk: &mut Vec<DiskSpace>) {
    let rev_disk_file_iter = disk
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_, disk_space)| disk_space.space.len() as u64 == disk_space.size)
        .rev();

    for (index, disk_space_to_move) in rev_disk_file_iter {
        move_if_can(disk, disk_space_to_move, index);
    }
}

fn to_checksum(disk: Vec<DiskSpace>) -> u64 {
    let mut checksum = 0;
    let mut index = 0;

    for disk_space in disk {
        for i in 0..disk_space.size as usize {
            if let Some(id) = disk_space.space.get(i) {
                checksum += index as u64 * id;
            }
            index += 1;
        }
    }

    checksum
}

fn solve(line: &str) -> u64 {
    let mut disk = to_disk(line);
    // println!("before    - {}", DiskDisplay(disk.clone()));
    pack_disk(&mut disk);
    // println!("after     - {}", DiskDisplay(disk.clone()));
    // print!("should be - ");
    // println!("00992111777.44.333....5555.6666.....8888..");

    to_checksum(disk)
}

fn main() {
    let lines = include_str!("input.txt");
    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
