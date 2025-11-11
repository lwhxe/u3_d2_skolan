// MaCro eXPansIOn https://youtu.be/TGfQu0bQTKc?t=39

macro_rules! create_roles {
    ($name:ident, $part1:ident, $type1:ident, $part2:ident, $type2:ident, $fmt:expr) => {
        struct $name {
            name: String,
            age: u32,
            $part1: $type1,
            $part2: $type2,
        }

        impl $name {
            fn info(self: &mut $name) {
                println!($fmt, self.name, self.age, self.$part1, self.$part2);
            }
        }
    };
}

create_roles!(Students, merit, f32, favorite_subject, String, "My name is {} and I am {} years old. My merit is {} and my favorite subject is {}.");
create_roles!(Teachers, revenue, f64, subject, String, "My name is {} and I am {} years old. I earn {} while teaching {}.");
create_roles!(Guitarist, band, String, fans, u32, "My name is {} and I am {} years old. I play in the band {} to {} fans.");

// Make sure the shit is like the shit should be.
// Not red.

enum Individual {
    S(Students),
    T(Teachers),
    G(Guitarist),
}

impl Individual {
    fn info(&mut self) {
        match self {
            Individual::S(student) => student.info(),
            Individual::T(teacher) => teacher.info(),
            Individual::G(guitarist) => guitarist.info(),
        }
    }
}

fn main() {
    let mut vals: [Individual; 9] = [
            Individual::S(Students {
                name: String::from("Saaim"),
                age: 17,
                merit: 18.5,
                favorite_subject: String::from("Cookie Clicker"),
            }),
            Individual::S(Students {
                name: String::from("Felix"),
                age: 17,
                merit: 18.5,
                favorite_subject: String::from("Entrepreneuring"),
            }),
            Individual::S(Students {
                name: String::from("Lukas"),
                age: 17,
                merit: 20.7,
                favorite_subject: String::from("Mathematics"),
            }),
            Individual::T(Teachers { 
                name: String::from("Josef"),
                age: 420,
                revenue: 100000000.0,
                subject: String::from("Sales"),
            }),
            Individual::T(Teachers { 
                name: String::from("Ehab"),
                age: 69,
                revenue: 100000000.0,
                subject: String::from("Programming"),
            }),
            Individual::T(Teachers { 
                name: String::from("Tedde"),
                age: 67,
                revenue: 13.37,
                subject: String::from("Idk man..."),
            }),
            Individual::G( Guitarist {
                name: String::from("Tim Henson"),
                age: 30,
                band: String::from("Polyphia"),
                fans: 500000,
            }),
            Individual::G( Guitarist {
                name: String::from("Jimi Hendrix"),
                age: 1000,
                band: String::from("Jimi Hendrix"),
                fans: 1200000,
            }),
            Individual::G( Guitarist {
                name: String::from("Mark Knopfler"),
                age: 76,
                band: String::from("Dire Straits"),
                fans: 5000000,
            }),
        ];

    // We made sure the shit isn't red so this works.
    for val in &mut vals {
        val.info();
    }
}
