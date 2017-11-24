extern crate pmc;

use pmc::model::*;

fn main() {
    let friends = gurps_friends();

    for friend in friends {
        println!("{}", friend);
    }
}

fn gurps_friends() -> Group {
    let mut friends = Group::new();

    friends.befriend(marcel());
    friends.befriend(meron());
    friends.befriend(mink());
    friends.befriend(paul());
    friends.befriend(robin());
    friends.befriend(sep());

    friends
}

fn marcel() -> Friend {
    Friend::new("Marcel", rea(), Snack::new("Doritos"))
}

fn rea() -> Character {
    CharacterBuilder::character()
        .with_name("Rea")
        .with_iq(15)
        .with_dx(8)
        .with_st(8)
        .with_ht(8)
        .create().unwrap()
}

fn meron() -> Friend {
    Friend::new("Meron", black_sword(), Snack::new("Cashew nuts"))
}

fn black_sword() -> Character {
    CharacterBuilder::character()
        .with_name("Black Sword")
        .with_iq(11)
        .with_dx(12)
        .with_st(10)
        .with_ht(10)
        .create().unwrap()
}

fn mink() -> Friend {
    Friend::new("Meron", ping(), Snack::new("Cashew nuts"))
}

fn ping() -> Character {
    CharacterBuilder::character()
        .with_name("Ping")
        .with_iq(10)
        .with_dx(14)
        .with_st(10)
        .with_ht(10)
        .create().unwrap()
}


fn paul() -> Friend {
    Friend::new("Paul", glorpio(), Snack::new("Maltezers"))
}

fn glorpio() -> Character {
    CharacterBuilder::character()
        .with_name("Glorpio")
        .with_iq(10)
        .with_dx(15)
        .with_st(12)
        .with_ht(8)
        .create().unwrap()
}

fn robin() -> Friend {
    Friend::new("Robin", hammar(), Snack::new("IPA"))
}

fn hammar() -> Character {
    CharacterBuilder::character()
        .with_name("Hammar")
        .with_iq(8)
        .with_dx(12)
        .with_st(16)
        .with_ht(12)
        .create().unwrap()
}

fn sep() -> Friend {
    Friend::new("Sep", oscarrrr(), Snack::new("Casave Chips"))
}

fn oscarrrr() -> Character {
    CharacterBuilder::character()
        .with_name("Oscarrrr")
        .with_iq(18)
        .with_dx(7)
        .with_st(6)
        .with_ht(6)
        .create().unwrap()
}
