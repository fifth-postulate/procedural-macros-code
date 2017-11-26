use std::collections::HashMap;
use std::fmt::{Display, Formatter, self};
use std::iter::{IntoIterator, Iterator};

#[derive(AnnounceDrop)]
pub struct Group {
    pub friends: Vec<Friend>,
}

impl Group {
    pub fn new() -> Group {
        Group { friends: Vec::new() }
    }

    pub fn befriend(&mut self, friend: Friend) {
        self.friends.push(friend)
    }
}

impl IntoIterator for Group {
    type Item = Friend;
    type IntoIter = GroupIterator;

    fn into_iter(self) -> Self::IntoIter {
        GroupIterator::new(self)
    }
}

pub struct GroupIterator {
    group: Group,
    index: usize,
}

impl GroupIterator {
    fn new(group: Group) -> GroupIterator {
        GroupIterator { group, index : 0 }
    }
}

impl Iterator for GroupIterator {
    type Item = Friend;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;

        if index < self.group.friends.len() {
            Some(self.group.friends[index].clone())
        } else {
            None
        }
    }
}

#[derive(Clone, AnnounceDrop)]
pub struct Friend {
    name: String,
    character: Character,
    pub favourite_snack: Snack,
}

impl Friend {
    pub fn new<S>(name: S, character: Character, snack: Snack) -> Friend where S: Into<String> {
        Friend {
            name: name.into(),
            character,
            favourite_snack: snack,
        }
    }
}

impl Display for Friend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} plays {} and eats {}", self.name, self.character, self.favourite_snack)
    }
}

#[derive(Clone)]
pub struct Character {
    name: String,
    attributes: HashMap<Attribute, u8>,
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} (", self.name)?;
        for (attribute, value) in &self.attributes {
            write!(f, " {}:{}", attribute, value)?;
        }
        write!(f, " )")
    }
}

pub struct CharacterBuilder {
    name: Option<String>,
    attributes: HashMap<Attribute, u8>,
}

impl CharacterBuilder {
    pub fn character() -> CharacterBuilder {
        CharacterBuilder {
            name: None,
            attributes: HashMap::new(),
        }
    }

    pub fn with_name<S>(mut self, name: S) -> CharacterBuilder
    where
        S: Into<String>,
    {
        self.name = Some(name.into());

        self
    }

    pub fn with_iq(mut self, iq: u8) -> CharacterBuilder {
        self.attributes.insert(Attribute::IQ, iq);

        self
    }

    pub fn with_st(mut self, st: u8) -> CharacterBuilder {
        self.attributes.insert(Attribute::ST, st);

        self
    }

    pub fn with_dx(mut self, dx: u8) -> CharacterBuilder {
        self.attributes.insert(Attribute::DX, dx);

        self
    }

    pub fn with_ht(mut self, ht: u8) -> CharacterBuilder {
        self.attributes.insert(Attribute::HT, ht);

        self
    }

    pub fn create(self) -> Result<Character, String> {
        if let Some(name) = self.name {
            if self.attributes.contains_key(&Attribute::IQ) &&
                self.attributes.contains_key(&Attribute::ST) &&
                self.attributes.contains_key(&Attribute::DX) &&
                self.attributes.contains_key(&Attribute::HT)
            {
                Ok(Character {
                    name,
                    attributes: self.attributes,
                })
            } else {
                Err("attribute missing".to_string())
            }
        } else {
            Err("no name given".to_string())
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Attribute {
    IQ,
    ST,
    DX,
    HT,
}

impl Display for Attribute {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Attribute::IQ => write!(f, "IQ"),
            Attribute::ST => write!(f, "ST"),
            Attribute::DX => write!(f, "DX"),
            Attribute::HT => write!(f, "HT"),
        }
    }
}

#[derive(Clone)]
pub struct Snack {
    name: String
}

impl Snack {
    pub  fn new<S>(name: S) -> Snack where S: Into<String> {
        Snack { name: name.into() }
    }
}

impl Display for Snack {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
