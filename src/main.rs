// Event with closure
enum Event<F> where F: Fn(), {
    Simple,
    WithClosure(Option<F>)
}

pub enum LedState{
    On,
    Off,
}

pub enum UartState{
    Idle,
    Busy,
}

struct SM<S, U> where U: Fn(usize) {
    pub state: Option<S>, // Current SM state
    recall: U,
    callback_complete: Option<fn()>, // This is permanent callback
    once_callback_complete: Option<fn()>, // This is temproriary callback. It calls only one time and override permanent callback_complete
}

impl<S, U> SM<S, U> where U: Fn(usize) {

    pub fn new(recall: U) -> Self {
        SM{state: None, recall, callback_complete: None, once_callback_complete: None}
    }

    pub fn set_state(mut self, state: S) -> Self {
        self.state = Some(state);
        self
    }

    pub fn get_state(&self) -> &Option<S> {
        &self.state
    }

    pub fn set_callback_complete(mut self, callback_complete: Option<fn()>) -> Self {
        self.callback_complete = callback_complete;
        self
    }

    pub fn set_once_callback_complete(mut self, once_callback_complete: Option<fn()>) -> Self {
        self.once_callback_complete = once_callback_complete;
        self
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

impl<UartState, U> SM<UartState, U> where U: Fn(usize) {
    pub fn print_uart(&self) {
        println!("UartState");
    }
}

impl<LedState, U> SM<LedState, U> where U: Fn(usize) {
    pub fn print_led(&self) {
        println!("LedState");
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


fn main() {
    let led_red: SM<LedState, _> = SM::new(|event| {println!("Event = {}", event)});
    let uart_0: SM<UartState, _> = SM::new(|event| {println!("Event = {}", event)});

    // let a = led_red;
    
    led_red.print_uart();

    let struct1: SM<LedState, _> = SM::new(|event| {println!("Event = {}", event)})
        .set_state(LedState::Off)
        .set_callback_complete(Some(||{}));
    let struct2 = Struct2{};
    struct1.test(&struct2);
    println!("Hello, world!");
}
