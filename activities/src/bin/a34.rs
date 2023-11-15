// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state
#[allow(dead_code)]
#[derive(Debug)]
struct Luggage<State> {
    id: i32,
    state: State,
}
impl <State> Luggage<State> {
    fn transform<NextState> (self, state: NextState) -> Luggage<NextState> {
        Luggage { id: self.id, state }
    }
}

#[derive(Debug)]
struct CheckIn;
impl Luggage<CheckIn> {
    pub fn new(id: i32) -> Luggage<CheckIn> {
        Luggage {
            id,
            state: CheckIn,
        }
    }
    pub fn load(self) -> Luggage<OnLoading> {
        self.transform(OnLoading)
    }
}

#[derive(Debug)]
struct OnLoading;
impl Luggage<OnLoading> {
    pub fn unload(self) -> Luggage<OffLoading> {
        self.transform(OffLoading)
    }
}

#[derive(Debug)]
struct OffLoading;
impl Luggage<OffLoading> {
    pub fn await_pickup(self) -> Luggage<AwaitingPickup> {
        self.transform(AwaitingPickup)
    }
}

#[derive(Debug)]
struct AwaitingPickup;
impl Luggage<AwaitingPickup> {
    pub fn pickup(self) -> Luggage<EndCustody> {
        self.transform(EndCustody)
    }
}

#[derive(Debug)]
struct EndCustody;

fn main() {
    let luggage = Luggage::new(1).load().unload().await_pickup().pickup();
    println!("{:?}", luggage);
}
