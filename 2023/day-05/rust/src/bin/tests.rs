use day_05::{MapRange, SeedRange, Transform};

fn main() {}

#[test]
fn it_works() {
    let hello = "hello";
    assert_eq!(hello.len(), 5);
}

#[test]
fn transform_range() {
    let seed_range = SeedRange {
        start: 55,
        length: 13,
    };
    let map_range = MapRange {
        destination_start: 52,
        source_start: 50,
        length: 48,
    };
    let transformed = seed_range.transformed_binary(&map_range);
    assert_eq!(
        transformed,
        SeedRange {
            start: 57,
            length: 13
        }
    )
}
