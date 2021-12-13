pub mod hosting {
    pub fn add_to_waitlist() {}

    pub fn seat_at_table() {
        // super 相当于 ../, 也就是当前域的父级
        // 而 seat_at_table 的父级就是 mod hosting
        // mod hosting 是和 mod serving 平级的
        // 所以 super 就可以拿到 mod serving 里面的东西
        super::serving::take_order();
        super::serving::server_order();
        super::serving::take_payment();
    }

    pub mod greeting {
        pub fn say_hello() {
            // 同理 say_hello 的父级就是 mod greeting
            // mod greeting 和 fn add_to_waitlist, fn seat_at_table 是平级的
            // 所以 super 就可以调用 add_to_waitlist 和 seat_at_table
            super::add_to_waitlist();
        }
    }
}

pub mod serving {
    pub fn take_order() {}

    pub fn server_order() {}

    pub fn take_payment() {}
}
