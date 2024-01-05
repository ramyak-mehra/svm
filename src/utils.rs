#[macro_export]
macro_rules! num {
    ($x:literal) => {
        Token::number($x)
    };
}

#[macro_export]
macro_rules! stack {
    ($($x:expr), *) => {{
        let mut temp_vec_deque = VecDeque::new();
        $(
            temp_vec_deque.push_back($x);
        )*
        temp_vec_deque
    }
    };
}
