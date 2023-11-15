// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles
#![allow(dead_code)]
#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn main() {
    //let tile = Tile::Brick(BrickStyle::Gray);
    //let water_9 = Tile::Water(Pressure(11));
    let gold = Tile::Treasure(TreasureChest { content: TreasureItem::Gold, amount: 99}); 
    match gold {
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
        Tile::Brick(
            color @ BrickStyle::Red 
            | color @ BrickStyle::Gray) => {
                println!("The brick color is {:?}", color)
            },
        Tile::Brick(other) => println!("{:?} brick", other),
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
        Tile::Water(Pressure(amount @ 0..=9)) => println!("Water pressure level: {}", amount),
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
        Tile::Water(_) => println!("High water pressure!"),
        Tile::Grass | Tile::Sand | Tile::Dirt => println!("Ground tile"),
        Tile::Treasure(
            TreasureChest {
                content: TreasureItem::Gold,
                amount
            }) if amount >= 100 => println!("lots of gold!"),
        _ => (),
    }
}