#[macro_export]
macro_rules! l_fn {
    ($e:expr) => {
        crate::types::Value::Function(std::rc::Rc::new(crate::types::LFn::from($e)))
    };
}

#[macro_export]
macro_rules! l_str {
    ($e:expr) => {
        crate::types::Value::String($e.into())
    };
}

#[macro_export]
macro_rules! l_true {
    () => {
        crate::types::Value::Bool(true)
    };
}

#[macro_export]
macro_rules! l_false {
    () => {
        crate::types::Value::Bool(false)
    };
}

#[macro_export]
macro_rules! l_null {
    () => {
        crate::types::Value::Null
    };
}

#[macro_export]
macro_rules! l_float {
    ($e:expr) => {
        crate::types::Value::Float($e)
    };
}

#[macro_export]
macro_rules! l_map {
    ($e:expr) => {
        crate::types::Value::Map($e)
    };
}

#[macro_export]
macro_rules! obj {
    ($e:expr) => {
        std::rc::Rc::new($e)
    };
}
