pub mod match_control_flow_operator;

fn main() {
    // 无参数枚举体
    #[allow(unused)]
    enum Number {
        Zero,
        One,
        Two,
    }

    // 类 C 枚举体
    #[allow(unused)]
    enum Status {
        Initial = 0,
        Processing = 1,
        Finish = 2,
    }

    // 携带类型参数的枚举体
    #[derive(Debug)]
    enum IpAddr<'a> {
        V4(u8, u8, u8, u8),
        V6(&'a str),
    }

    let v4 = IpAddr::V4(127, 0, 0, 1);
    let v6 = IpAddr::V6("2408:8606:1800:501::1:4");
    println!("{:?} {:?}", v4, v6);

    // 枚举也可被 impl
    impl IpAddr<'_> {
        #[allow(unused)]
        fn print_ip(&self) {
            dbg!(self);
        }
    }

    // 枚举的一个重要实现是 Option
    let s = Some(1);
    let num = s.unwrap();
    match s {
        Some(n) => {
            assert_eq!(num, n);
            println!("{}", n);
        }
        None => panic!("nil"),
    }

    assert_eq!(&s.unwrap(), s.as_ref().unwrap());
}
