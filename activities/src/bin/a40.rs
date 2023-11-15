#![allow(dead_code)]
use std::cell::RefCell;

// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented

#[derive(Debug)]
struct Rental {
    vehicule_type: VehicleType,
    vin: u16,
    vehicle_status: VehicleStatus,
}

#[derive(Debug)]
enum VehicleStatus {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}

#[derive(Debug)]
enum VehicleType {
    Car,
    Motorcycle,
    Truck,
}
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

struct Corporate{
    rentals: RefCell<Vec<Rental>>,
}

struct StoreFront<'a> {
    rentals: &'a RefCell<Vec<Rental>>,
}

fn main() {
    let corporate = Corporate { rentals: RefCell::new(Vec::new())};
    let store = StoreFront { rentals: &corporate.rentals};
    let first_rental = Rental {
        vehicule_type: VehicleType::Car,
        vin: 20,
        vehicle_status: VehicleStatus::Rented,
    };
    let new_model = Rental {
        vehicule_type: VehicleType::Truck,
        vin: 1,
        vehicle_status: VehicleStatus::Available,
    };
    {
        store.rentals.borrow_mut().push(first_rental);
        corporate.rentals.borrow_mut().push(new_model);
        store.rentals.borrow().iter().map(|v| println!("{:?}",v)).for_each(drop);
    }
}
