fn ch_3() {
    let x = 13;
    println!("{}", x);

    let x: f64 = 3.14;
    println!("{}", x);
}

fn ch_4() {
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);
}

fn ch_5() {
    let x = 12; // default is i32. others area i8, i32, i64, i128
    let a = 12u8; // u8, u32, u64, u128
    let b = 4.3; // f64
    let c = 4.3f32; // f32, f64
    let bv = true; // true or false
    let t = (13, true);
    let sentence = "h w";
    println!("{} {} {} {} {} {} {} {}", x, a, b, c, bv, t.0, t.1, sentence);
}

fn ch_6() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}

const PI: f32 = 3.14;
fn ch_7() {
    println!("{}", PI);
}

fn ch_8() {
    let nums: [i32; 3] = [1,2,3];
    println!("{:?}", nums);
    println!("{}", nums[0]);
}

fn ch_9() {
    println!("{}", add(1, 2));
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn ch_10() {
    let r = swap(1, 2);
    println!("{} {}", r.0, r.1);

    let (a, b) = swap(1, 2);
    println!("{} {}", a, b);
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn ch_11() {
    let a = nothing();
    println!("{:?}", a);

    let b = nothing2();
    println!("{:?}", b);
}

fn nothing() -> () {
    return ();
}

fn nothing2() {

}

fn ch_14() {
    let x = 42;
    if x < 42 {
        println!("x < 42");
    } else if x == 42 {
        println!("x == 42");
    } else {
        println!("x > 42");
    }
}

fn ch_15() {
    let mut x = 0;
    loop {
        x += 1;
        if x > 42 {
            break;
        }
    }
    println!("{}", x);
}

fn ch_16() {
    let mut x = 0;
    while x != 42 {
        x += 1;
    }
    println!("{}", x);
}

fn ch_17() {
    for x in 0..5 {
        println!("{}", x);
    }

    for x in 0..=5 {
        println!("{}", x);
    }
}

fn ch_18() {
    let x = 111;

    match x {
        0 => {
            println!("0");
        }
        1 | 2 => {
            println!("1 or 2")
        }
        3..=9 => {
            println!("3 ~ 9")
        }
        matched_num @ 10..=100 => {
            println!("{}", matched_num);
        }
        _ => {
            println!("default")
        }
    }
}

fn ch_19() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "Found 13.";
        }
    };
    println!("{}", v);
}

fn ch_20() {
    let x = 42;
    let v = if x < 42 { -1 } else { 1 };
    println!("if: {}", v);

    let food = "hamburger";
    let r = match food {
        "hamburger" => "yes",
        _ => "no",
    };
    println!("{}", r);

    let v = {
        let a = 1;
        let b = 2;
        a + b
    };
    println!("v: {}", v);
}

fn ch_21() {
    let s = String::from("hello");
    println!("{} is {} characters long", s, s.len());
}

fn ch_26() {
    struct SeaCreature {
        animal_type: String,
        name: String,
        arms: i32,
        legs: i32,
        weapon: String
    }

    let ferris = SeaCreature {
        animal_type: String::from("crab"),
        name: String::from("ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw")
    };

    println!("animal_type: {}, name: {}, arms: {}, legs: {}, weapon: {}", ferris.animal_type, ferris.name, ferris.arms, ferris.legs, ferris.weapon);
}

fn ch_27() {
    struct Location(i32, i32);

    let loc = Location(1, 2);
    println!("{}, {}", loc.0, loc.1);
}

fn ch_29() {
    #![allow(dead_code)]
    enum Species {
        Crab,
        Octopus,
        Fish,
        Clam
    }

    struct SeaCreature {
        species: Species,
        name: String
    }

    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("ferris")
    };

    match ferris.species {
        Species::Crab => println!("crab"),
        Species::Octopus => println!("octopus"),
        Species::Fish => println!("fish"),
        Species::Clam => println!("clam")
    }
}

fn ch_30() {
    #![allow(dead_code)]

    enum Species { Crab, Octopus }
    enum PoisonType { Acidic, Painful }
    enum Size { Big, Small }
    enum Weapon {
        Claw(i32, Size),
        Poison(PoisonType)
    }

    struct SeaCreature {
        species: Species,
        name: String,
        weapon: Weapon
    }

    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("ferris"),
        weapon: Weapon::Claw(1, Size::Big)
    };

    match ferris.species {
        Species::Crab => {
            match ferris.weapon {
                Weapon::Claw(num_claws, size) => {
                    let size_dest = match size {
                        Size::Big => "big",
                        Size::Small => "small"
                    };
                    println!("num: {}, size: {}", num_claws, size_dest)
                },
                _ => println!("other weapon")
            }
        }
        _ => println!("other species")
    }
}

fn main() {
    println!("- ch_3 -");
    ch_3();

    println!("- ch_4 -");
    ch_4();

    println!("- ch_5 -");
    ch_5();

    println!("- ch_6 -");
    ch_6();

    println!("- ch_7 -");
    ch_7();

    println!("- ch_8 -");
    ch_8();

    println!("- ch_9 -");
    ch_9();

    println!("- ch_10 -");
    ch_10();

    println!("- ch_11 -");
    ch_11();

    println!("- ch_14 -");
    ch_14();

    println!("- ch_15 -");
    ch_15();

    println!("- ch_16 -");
    ch_16();

    println!("- ch_17 -");
    ch_17();

    println!("- ch_18 -");
    ch_18();

    println!("- ch_19 -");
    ch_19();

    println!("- ch_20 -");
    ch_20();

    println!("- ch_21 -");
    ch_21();

    println!("- ch_26 -");
    ch_26();

    println!("- ch_27 -");
    ch_27();

    println!("- ch_29 -");
    ch_29();

    println!("- ch_30 -");
    ch_30();
}
