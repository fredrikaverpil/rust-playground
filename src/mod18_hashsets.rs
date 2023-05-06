use std::collections::HashSet;

// like a set in Python

// HashSet is unordered, contains unique elements only

pub fn main() {
    let mut greeks = HashSet::new();

    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks); // {"delta", "gamma"}

    greeks.insert("delta"); // will not add it again

    let added_vega = greeks.insert("vega"); // will add it
    if added_vega {
        println!("we added vega!");
    }

    if !greeks.contains("kappa") {
        println!("we don't have kappa");
    }

    let removed = greeks.remove("delta"); // will remove delta
    if removed {
        println!("we removed delta");
    }

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // is_subset: returns true if the set is a subset of the specified set
    println!(
        "is {:?} a subset of {:?}? {}",
        _2_8,
        _1_10,
        _2_8.is_subset(&_1_10)
    ); // true

    // disjoint = no common elements
    println!(
        "is {:?} and {:?} disjoint? {}",
        _1_5,
        _6_10,
        _1_5.is_disjoint(&_6_10)
    ); // true

    // union: returns a new set with all the elements in both sets
    println!(
        "items in either {:?} or {:?} are {:?}",
        _2_8,
        _6_10,
        _2_8.union(&_6_10)
    ); // {2, 3, 4, 5, 6, 7, 8, 9, 10}

    // difference: returns a set of all the elements that are in the set and not in the specified set
    println!(
        "items in {:?} that are not in {:?} are {:?}",
        _1_10,
        _2_8,
        _1_10.difference(&_2_8)
    ); // {1, 9, 10}

    // intersection: returns a set of all the elements common to the set and the specified set
    println!(
        "items in {:?} that are also in {:?} are {:?}",
        _1_10,
        _2_8,
        _1_10.intersection(&_2_8)
    ); // {2, 3, 4, 5, 6, 7, 8}

    // symmetric_difference: returns a set of all the elements that are in one of either set, but not both
    println!(
        "items in {:?} or {:?} but not both are {:?}",
        _1_10,
        _2_8,
        _1_10.symmetric_difference(&_2_8)
    ); // {1, 9, 10, 3, 4, 5, 6, 7}

    // is_superset: returns true if the set is a superset of the specified set
    println!(
        "is {:?} a superset of {:?}? {}",
        _1_10,
        _2_8,
        _1_10.is_superset(&_2_8)
    ); // true
}
