use crate::Event::WithClosure;

// Event with closure
enum Event<F> where F: Fn(usize), {
    Simple,
    WithClosure(Option<F>)
}

struct Struct1 {
}

impl Struct1 {
    pub fn run<F>(&self, event: Event<F>) where F: Fn(usize) {
        match event {
            Event::Simple => {
                println!("Simple");
            },
            Event::WithClosure(closure) => {
                if let Some(cl) = closure {
                    cl(1);
                }
            }
        }
    }

    pub fn dummy(&self, num: usize) {
        println!("Struct1: {}", num);
    }

    pub fn test(&self, s2: &Struct2) {
        s2.run(Event::WithClosure(Some(|e| {self.dummy(3)})));
    }
}

struct Struct2 {
}

impl Struct2 {
    pub fn run<F>(&self, event: Event<F>) where F: Fn(usize) {
        match event {
            Event::Simple => {
                println!("Simple");
            },
            Event::WithClosure(closure) => {
                if let Some(cl) = closure {
                    cl(2);
                }
            }
        }
    }

    pub fn dummy(num: usize) {
        println!("Struct2: {}", num);
    }

}


fn main() {
    let struct1 = Struct1{};
    let struct2 = Struct2{};
    struct1.test(&struct2);
    println!("Hello, world!");
}
