
fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

#[test]
fn test_apply() {
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));
}

fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    3.1415926;
}


// cargo test -- --nocapture
#[test]
fn test_pi() {
    let is_pi = pi();

    let is_unit1 = not_pi();
    let is_unit2 = {
        pi();
    };

    println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2);
}


//define struct

#[derive(Debug)]
enum Gender {
    Unspacified = 0,
    Femal = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);


#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId, 
    name: String,
    gender: Gender,
}


#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    ownder: UserId,
}



#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}



fn process_event(event: &Event) {
    match event {
        Event::Join((uid, _tid)) => println!("User {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("User {:?}, left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcast: {}",msg),
    }
}


fn process_message(event:  &Event) {
    if let Event::Message((_,_,msg) ) = event {
        println!("broadcast: {}", msg);
    }
}

#[test]
fn test_msg() {
    let alice = User {
        id: UserId(1),
        name: "Alice".into(), 
        gender: Gender::Femal,
    };

    let bob = User {
        id: UserId(2),
        name: "Bob".into(), 
        gender: Gender::Male,  
    };

    let topic = Topic {
        id: TopicId(1),
        name: "rust".into(), 
        ownder: UserId(1),
    };

    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));


    println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);

    process_message(&event3);

    process_event(&event1);
    process_event(&event2);
}  


fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("Next val is {}", b);

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);
    }
}


fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", b);
    }
}

#[test]
fn test_fib()  {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}
