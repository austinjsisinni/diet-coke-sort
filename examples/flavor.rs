//! Sort tasting notes by perceived crispness.

use diet_coke_sort::sort_by;

#[derive(Debug)]
struct TastingNote {
    name: &'static str,
    crispness: u8,
}

fn main() {
    let mut notes = [
        TastingNote {
            name: "clean finish",
            crispness: 8,
        },
        TastingNote {
            name: "bright citrus",
            crispness: 9,
        },
        TastingNote {
            name: "caramel",
            crispness: 6,
        },
        TastingNote {
            name: "fine bubbles",
            crispness: 9,
        },
    ];

    let report = sort_by(&mut notes, |left, right| {
        right.crispness.cmp(&left.crispness)
    });

    for TastingNote { name, crispness } in notes {
        println!("{name} ({crispness})");
    }
    println!("{report:#?}");
}
