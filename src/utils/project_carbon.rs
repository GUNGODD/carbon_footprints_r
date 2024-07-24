use std::io;
pub fn project_carbon_footprint() {
    //TODO : Enums and Structs

    enum Direction {
        NORTH,
        SOuTH,
        EAST,
        WEST,
    }

    struct Coordinate {
        x: i32,
        y: i32,
    }

    // user struct

    struct user {
        name: String,
        age: i32,
        traveling: bool,
        gender: String,
        distance: f32,
    }

    // fake users ? TODO:

    let mut u1 = user {
        name: String::from("Aditya"),
        age: 21,
        traveling: true,
        gender: String::from("Male"),
        distance: 10.0,
    };

    let mut u2 = user {
        name: String::from("Diksha"),
        age: 22,
        traveling: false,
        gender: String::from("Female"),
        distance: 20.0,
    };
    let mut u3 = user {
        name: String::from("Purvi"),
        age: 22,
        traveling: false,

        gender: String::from("Female"),

        distance: 20.0,
    };
    let mut u4 = user {
        name: String::from("Smita"),
        age: 22,
        traveling: false,

        gender: String::from("Female"),
        distance: 20.0,
    };
    let mut u5 = user {
        name: String::from("Yashika"),
        age: 22,
        traveling: false,
        gender: String::from("Female"),
        distance: 20.0,
    };
    let mut u6 = user {
        name: String::from("Shivani"),
        age: 22,
        traveling: false,
        gender: String::from("Female"),
        distance: 20.0,
    };

    //display users

    for i in 0..6 {
        println!("Name: {}", u[i].name);
        println!("Age: {}", u[i].age);
        println!("Traveling: {}", u[i].traveling);
        println!("Gender: {}", u[i].gender);
        println!("Distance: {}", u[i].distance);
    }
}

// functions

fn display_users() {}

fn get_users() {
    //TODO:  get users from database

    let mut searched_users = String::new();
    println!("Enter the user you want to Search ");
    io::stdin()
        .read_line(&mut searched_users)
        .expect("failed to read lines");
}

fn add_user() {
    //TODO:  add users to database
}

fn delete_user() {
    //TODO:  delete users from database
}

fn update_user() {
    //TODO:  update users in database
}
