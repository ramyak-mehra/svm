#[macro_export]
macro_rules! data {
    ($x:expr) => {
        crate::token::Token::data($x)
    };
}

#[macro_export]
macro_rules! int {
    ($x:literal) => {
        Operand::Int($x)
    };
}

#[macro_export]
macro_rules! tint {
    ($x:literal) => {
        crate::token::Token::data(crate::token::operand::Operand::Int($x))
    };
}

#[macro_export]
macro_rules! float {
    ($x:literal) => {
        Operand::Float($x)
    };
}

#[macro_export]
macro_rules! tfloat {
    ($x:literal) => {
        crate::token::Token::data(crate::token::operand::Operand::Float($x))
    };
}

#[macro_export]
macro_rules! str {
    ($x:expr) => {
        Operand::Str($x)
    };
}

#[macro_export]
macro_rules! tstr {
    ($x:expr) => {
        crate::token::Token::data(crate::token::operand::Operand::Str($x))
    };
}

#[macro_export]
macro_rules! bool {
    ($x:literal) => {
        Operand::Bool($x)
    };
}

#[macro_export]
macro_rules! tbool {
    ($x:literal) => {
        crate::token::Token::data(crate::token::operand::Operand::Bool($x))
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
