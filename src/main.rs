// use mysql::*;
use mysql::prelude::*;
use std::io;
use std::cmp::Ordering;
use std::collections::HashMap;

struct Cube {
    make: String, // Gan
    model: String, // 12 MagLev UV
    price: f64,
    magnets: i32, // strength
    magnetic_core: i32, // alignment strength
    description: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // connecting to DB

    let url = "mysql://root:$w@g69$w@g69@localhost:3306/cubing";
    let pool = mysql::Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // mapping sql data to struct vector
    
    let mut cubes: Vec<Cube> = conn.query_map(
        "SELECT make, model, price, magnets, magnetic_core, description FROM cubes",
        |(make, model, price, magnets, magnetic_core, description)| {
            Cube {make, model, price, magnets, magnetic_core, description}
        },
    )?;

    // creating string vectors to later dump into hashmaps for valid options

    let valid_dir_asc_vec: Vec<&str> = vec!["a", "asc", "ascend", "ascending", "u", "up"];
    let valid_dir_desc_vec: Vec<&str> = vec!["d", "desc", "descend", "descending", "down"];
    let valid_sorts_vec: Vec<&str> = vec!["make", "model", "price", "magnets", "magnetic_core"];

    // creating hashmaps and dumping the vector data into them

    let mut valid_dir_asc = HashMap::new();
    for i in valid_dir_asc_vec {
        valid_dir_asc.insert(i, i);
    }

    let mut valid_dir_desc = HashMap::new();
    for i in valid_dir_desc_vec {
        valid_dir_desc.insert(i, i);
    }

    let mut valid_sorts = HashMap::new();
    for i in valid_sorts_vec {
        valid_sorts.insert(i, i);
    }

    // getting input on how to sort the results
    // TODO make literally everything after this point loop so it can keep querying 
    // maybe make entering "exit" at any point make it immediately close

    println!("Enter a valid sorting method:");
    let mut sort_selection = String::new();
    loop {
        sort_selection.clear();
        io::stdin()
            .read_line(&mut sort_selection)
            .expect("Input error");
        sort_selection = sort_selection.trim().to_lowercase().replace(" ", "_");
        if valid_sorts.contains_key(sort_selection.as_str()) {
            break;
        } else {
            println!("Whoops! \"{sort_selection}\" is not a valid sort, such as \"make\" or \"model\". Try again! ");
        }
    }

    println!("Enter a valid sorting direction:");
    let mut sort_direction = String::new();
    let mut sort_reverse = false;
    loop {
        sort_direction.clear();
        io::stdin()
            .read_line(&mut sort_direction)
            .expect("Input error");
        sort_direction = sort_direction.trim().to_lowercase().replace(" ", "_");
        if valid_dir_asc.contains_key(sort_direction.as_str()) {
            break;
        } else if valid_dir_desc.contains_key(sort_direction.as_str()) {
            sort_reverse = true;
            break;
        } else {
            println!("Whoops! \"{sort_direction}\" is not a valid direction, such as \"asc\" or \"desc\". Try again! ");
        }
    }

    // sorting based on input

    cubes.sort_by(|a, b| match sort_selection.as_str() {
        "make" => a.make.cmp(&b.make),
        "model" => a.model.cmp(&b.model),
        "price" => a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal),
        "magnets" => a.magnets.cmp(&b.magnets),
        "magnetic_core" => a.magnetic_core.cmp(&b.magnetic_core),
        "description" => a.description.cmp(&b.description), // this will never be used, just want to get rid of a warning in the struct
        _ => panic!("Invalid field: {}", sort_selection), // this will also never be used, as the selections are validated way earlier
    });

    // printing time!!!
    
    let mut counter: i32 = 0;
    if sort_reverse {
        for x in cubes.iter().rev() { 
            counter += 1;
            println!("{counter} {} {}", x.make, x.model);
        }
    } else {
        for x in cubes { 
            counter += 1;
            println!("{counter} {} {}", x.make, x.model);
        }
    }

    Ok(())
}