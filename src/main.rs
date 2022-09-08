// Event with closure
enum Event<F> where F: Fn(), {
    Simple,
    WithClosure(Option<F>)
}

struct Struct1<U> where U: Fn(usize) {
    recall: U,
}

impl<U> Struct1<U> where U: Fn(usize) {

    pub fn new(recall: U) -> Self {
        Struct1{recall}
    }

    pub fn run<F>(&self, event: Event<F>) where F: Fn() {
        match event {
            Event::Simple => {
                println!("Simple");
            },
            Event::WithClosure(closure) => {
                if let Some(cl) = closure {
                    cl();
                }
            }
        }
    }

    pub fn test(&self, s2: &Struct2) {
        s2.run(Event::WithClosure(Some(|| {(self.recall)(3)})));
    }
}

struct Struct2 {
}

impl Struct2 {
    pub fn run<F>(&self, event: Event<F>) where F: Fn() {
        match event {
            Event::Simple => {
                println!("Simple");
            },
            Event::WithClosure(closure) => {
                if let Some(cl) = closure {
                    cl();
                }
            }
        }
    }

}


struct A<F: Fn(usize)> {
    s: Struct1<F>,
}

impl<F: Fn(usize)> A<F> {
    pub fn new(recall: F) -> Self{
        A{s: Struct1::new(recall)}
    }
}


fn main() {
    let struct1 = Struct1::new(|event| {println!("Event = {}", event)});
    let struct2 = Struct2{};
    let a = A::new(|event| {println!("Event = {}", event)});
    struct1.test(&struct2);
    println!("Hello, world!");
}
