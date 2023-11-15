// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows


const MOCK_DATA: &'static str = include_str!("mock-data.csv");

#[allow(dead_code)]
#[derive(Debug)]
struct PersonPtr<'a> {
    name: &'a str,
    title: &'a str,
}
fn main() {
    let mut people: Vec<PersonPtr> = Vec::new();
    for entry in MOCK_DATA.split('\n').skip(1) {
        let mut entry_fields = entry.split(',');
        let name = entry_fields.by_ref().skip(1).next();
        let title = entry_fields.last();
        if name.is_some() && title.is_some() {
            people.push(PersonPtr { name: name.unwrap(), title: title.unwrap().trim() });
        }
    }
    for person in people.iter() {
        println!("{:?}", person);
    }
}
