use std::ops::Add;

#[derive(Debug)]
struct Cup<A> {
    content: A
}

// impl <A> Add<Cup<A>> for Cup<A>
// where A: Add<A>
// {
//     type Output = Cup<<A as Add<A>>::Output>;
//
//     fn add(self, rhs: Cup<A>) -> Self::Output {
//         let added_content = self.content.add(rhs.content);
//         Cup { content: added_content}
//     }
// }

impl <A,B> Add<Cup<B>> for Cup<A>
where A: Add<B>
{
    type Output = Cup<<A as Add<B>>::Output>;

    fn add(self, rhs: Cup<B>) -> Self::Output {
        let sum = self.content + rhs.content;
        Cup { content: sum }
    }
}

fn main() {
    let c= Cup {content: 3};
    let d = Cup {content: 4.0};
    // let e = c.add(d);
    let e = 4;
    println!("{:?} {:?}", e, c+d);
}