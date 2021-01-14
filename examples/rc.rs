use std::rc::Rc;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct Owner {
    name: String
}

struct Gadget {
    id: u32,
    owner: Rc<Owner>
}

fn main() {
    let rowner = Rc::new(
        Owner {
            name: "Bruce".to_string()
        }
    );

    let m = HashMap::new();
    let o = Owner {
        name: "Bruce".to_string()
    };
    if m.contains_key(&o) {

    }

    let gad1 = Gadget {
        id: 0,
        owner: rowner.clone()
    };

    let gad2 = Gadget {
        id: 1,
        owner: Rc::clone(&rowner)
    };

    drop(rowner);
    println!("{} {:?}", gad1.id, gad1.owner.name);
}