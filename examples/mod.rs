mod out {
    fn p_function() {
        println!("Call out::p_function()");
    }

    pub fn function(){
        println!("Call out::function()");
    }

    pub fn indirect() {
        println!("Call out::indirect()");
        p_function();
    }

    pub mod inner {
        pub fn function() {
            println!("Call out::inner::function()");
        }

        pub(in crate::out) fn pub_function_in_out() {
            println!("Call out::inner::pub_function_in_out()");
            pub_function_in_inner();
        }

        pub(self) fn pub_function_in_inner() {
            println!("Call out::inner::pub_function_in_inner()");
        }

        pub(super) fn pub_function_in_super_mod() {
            println!("Call out::inner::pub_function_in_super_mod()");
        }
    }

    pub fn call_pub_function_in_out() {
        println!("Call out::call_pub_function_in_out()");
        inner::pub_function_in_out();
        inner::pub_function_in_super_mod();
    }
}
fn function() {
    print!("Call function()");
}

fn main() {
    out::function();
    out::indirect();

    out::inner::function();

    out::call_pub_function_in_out();
}