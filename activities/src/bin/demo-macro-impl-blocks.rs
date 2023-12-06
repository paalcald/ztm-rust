#[derive(Clone, Copy, Debug)]
struct Volume(usize);
macro_rules! impl_reagent_container {
    ($container:ty : $volume:literal) => {
        impl ReagentContainer for $container {
            fn max_volume(&self) -> Volume {
                Volume($volume)
            }
            fn current_volume(&self) -> Volume {
                self.current_volume
            }
        }
    };
}
trait ReagentContainer {
    fn max_volume(&self) -> Volume;
    fn current_volume(&self) -> Volume;
}

struct TallFlask {
    current_volume: Volume,
}
impl_reagent_container!(TallFlask: 50);

struct TestTube {
    current_volume: Volume,
}
impl_reagent_container!(TestTube: 25);

struct Pipette {
    current_volume: Volume,
}
impl_reagent_container!(Pipette: 75);

struct OtherTube {
    current_volume: Volume,
    max_volume: Volume,
}

impl ReagentContainer for OtherTube {
    fn max_volume(&self) -> Volume {
        self.max_volume
    }
    fn current_volume(&self) -> Volume {
        self.current_volume
    }
}

fn main() {
    let pipette = Pipette{current_volume: Volume(30)};
    println!("Pippete max volume is {:?}", pipette.max_volume());
}
