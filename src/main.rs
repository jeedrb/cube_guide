use mysql::*;
use mysql::prelude::*;
use std::io;
use std::cmp::Ordering;

struct Cube {
    make: String, // Gan
    model: String, // 12 MagLev UV
    price: f64,
    magnets: i32, // strength
    magnetic_core: i32, // alignment strength
    description: String,
}

fn main() {
    let url = "mysql://root:$w@g69$w@g69@localhost:3306/cubing";
    let pool = mysql::Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let mut cubes: Vec<Cube> = conn.query_map(
        "SELECT make, model, price, magnets, magnetic_core, description FROM cubes",
        |(make, model, price, magnets, magnetic_core, description)| {
            Cube {make, model, price, magnets, magnetic_core, description}
        },
    ).unwrap();
    
    // getting input on how to sort the results
    let mut sort_selection = String::new();
    io::stdin()
        .read_line(&mut sort_selection)
        .expect("try again dummy");
    
    // first trims edge whitespace, then makes it all lowercase, then converts the remaining spaces to underscores
    let sort_selection = sort_selection.trim().to_lowercase().replace(" ", "_");

    // sorting based on input
    cubes.sort_by(|a, b| match sort_selection.as_str() {
        "make" => a.make.cmp(&b.make),
        "model" => a.model.cmp(&b.model),
        "price" => a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal),
        "magnets" => a.magnets.cmp(&b.magnets),
        "magnetic_core" => a.magnetic_core.cmp(&b.magnetic_core),
        // "magnetic core" => a.magnetic_core.cmp(&b.magnetic_core),
        _ => panic!("Invalid field: {}", sort_selection),
    });

    let mut counter: i32 = 0;
    for x in cubes {
        counter += 1;
        println!("{counter} {} {}", x.make, x.model);
    }
}