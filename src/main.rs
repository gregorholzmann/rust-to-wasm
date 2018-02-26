
#[derive(Debug)]
enum Direction { North, South, East, West }

fn is_north(dir: Direction) -> bool {
    match dir {
        Direction::North => true,
        _ => false,
    }
}
fn waddup() {
    println!("WADDUP");
}

fn main() {
    let points = Direction::South;
    println!("{:?}", points);
    let compass = is_north(points);
    println!("{}", compass);
    waddup();
}
