fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback: IpAddr = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    {
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));

        let loopback: IpAddr = IpAddr::V6(String::from("::1")); 
    }

    {
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
    
        let home: IpAddr = IpAddr::V4(127, 0, 0, 1);
    
        let loopback: IpAddr = IpAddr::V6(String::from("::1"));
    }

    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        impl Message {
            fn call(&self) {
                // method body would be defined here
            }
        }
    
        let m: Message = Message::Write(String::from("hello"));
        m.call();
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five: Option<i32> = Some(5);
    let six : Option<i32>= plus_one(five);
    let none: Option<i32> = plus_one(None);
}


