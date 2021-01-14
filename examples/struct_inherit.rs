mod apple_eater {
    pub struct AppleEater {
        granny_smiths: u32,
        golden_delicious: u32,
    }

    impl AppleEater {
        //
        pub fn new() -> Self {
            Self {
                granny_smiths: 12,
                golden_delicious: 24,
            }
        }
        pub fn report(&self) {
            println!("I ate {} granny smiths!", self.granny_smiths);
            println!("...and {} golden delicious!", self.golden_delicious);
        }

        pub fn eat_granny_smith(&mut self) { self.granny_smiths += 1; }
        pub fn eat_golden_delicious(&mut self) { self.golden_delicious += 1; }
    }

    pub trait AsAppleEater {
        fn as_apple_eater(&self) -> &AppleEater;

        fn report(&self)
        { self.as_apple_eater().report() }
    }

    pub trait AsMutAppleEater: AsAppleEater {
        fn as_mut_apple_eater(&mut self) -> &mut AppleEater;

        fn eat_granny_smith(&mut self)
        { self.as_mut_apple_eater().eat_granny_smith() }

        fn eat_golden_delicious(&mut self)
        { self.as_mut_apple_eater().eat_golden_delicious() }
    }
}
use apple_eater::*;

struct Teacher {
    apple_eater: AppleEater,
    name: String,
    age: u16,
    subject: String,
}

struct Student {
    apple_eater: AppleEater
}

impl AsAppleEater for Teacher {
    fn as_apple_eater(&self) -> &AppleEater
    { &self.apple_eater }
}

impl AsMutAppleEater for Teacher {
    fn as_mut_apple_eater(&mut self) -> &mut AppleEater
    { &mut self.apple_eater }
}

impl AsAppleEater for Student {
    fn as_apple_eater(&self) -> &AppleEater
    { &self.apple_eater }
}

fn main() {
    let mut te = Teacher {
        apple_eater: AppleEater::new(),
        name: "Bruce".to_string(),
        age: 128,
        subject: "Math".to_string()
    };
    te.report();
    te.eat_golden_delicious();
    te.report();
}