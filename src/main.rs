use mysql::*;
use mysql::prelude::*;

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

    let cubes: Vec<Cube> = conn.query_map(
        "SELECT make, model, price, magnets, magnetic_core, description FROM cubes",
        |(make, model, price, magnets, magnetic_core, description)| {
            Cube {make, model, price, magnets, magnetic_core, description}
        },
    ).unwrap();
        
    let mut counter: i32 = 0;
    for x in cubes {
        counter += 1;
        println!("{counter} {} {}", x.make, x.model);
    }
}