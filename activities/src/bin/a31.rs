// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
trait Cost {
    fn get_cost(&self) -> f64;
}
struct Carpet {
    area: f64
}
impl Cost for Carpet {
   fn get_cost(&self) -> f64 {
       self.area * 10.0
   } 
}

struct Tile {
    area: f64,
}
impl Cost for Tile {
    fn get_cost(&self) -> f64 {
        self.area * 15.0
    }
}

struct Wood {
    area: f64,
}
impl Cost for Wood {
    fn get_cost(&self) -> f64 {
        self.area * 20.0
    }
}
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

fn main() {
    let wood = &Wood {area: 10.0};
    let tile = &Tile {area: 60.};
    let carpet = &Carpet {area: 50.0};
    let materials: Vec<&dyn Cost> = vec![wood, tile, carpet];
    let cost: f64 = materials.iter().map(|&x| x.get_cost()).sum();
    println!("{:?}", cost);

}
