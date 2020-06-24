fn main() {
    // rust have 4 main types
    // integer, floating point, boolean, characters
    //
    // let x = 1;
    // let a: i8 = -2;
    // let y = 2;
    // let qaiyum = "qaiyyum";
    // let boolean = true;
    // let r: [u8; 3] = [3, 4, 5];
    // println!("the value of {:#?}", r);

    // create new vector
    // let mut u:Vec<u8> = Vec::new()

    // let dynamic_math =  8 - 2 - 6;

    // let my_array = [1,2,3,4,5,6];

    // let my_tuple = (5, 5.9, "chris");

    // println!("The value of x is: {}", x);
    // println!("The value of x is: {}", y);

    calculate_number(1);
    calculate_number(2);
    calculate_number(3);

    let one = Number {
        odd: true,
        value: 1,
    };

    let two = Number {
        odd: false,
        value: 2,
    };

    let minus_two = Number {
        odd: false,
        value: -2,
    };

    let mut mutable_var = Number {
        odd: false,
        value: 3,
    };

    mutable_var.value = 5;
    print_number(mutable_var);

    print_number(one);

    print_number(two);
    println!("positive {}", minus_two.is_strictly_positive());
}

// write function that prints base on input number

fn calculate_number(a: i8) {
    if a == 1 {
        println!("the value is {}", a)
    } else if a == 2 {
        println!("helllllooo")
    } else if a == 3 {
        println!("threeee")
    }
}

struct Number {
    odd: bool,
    value: i32,
}

impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}

fn print_number(n: Number) {
    match n {
        Number { odd: true, value } => println!("Odd number {} ", value),
        Number { odd: false, value } => println!("Even number {}", value),
    }
}

struct Shape {
    value: i8,
    types: str,
}

trait Calculate {
    fn return_width() -> i8 {
        return 8;
    }
}

impl Calculate for Shape {}

// enum
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String.from("::1")
};
