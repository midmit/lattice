use std::{
    any::Any,
    collections::HashMap,
    ffi::c_void,
    fmt::{self, Debug},
    rc::Rc,
};

use crate::obj;

pub type Object<T> = Rc<T>;

#[derive(Clone)]
pub enum Value {
    Null,
    Bool(bool),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    String(String),
    Map(LMap),
    ExternSymbol(*mut c_void),
    Function(Object<LFn>),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::I32(i) => write!(f, "{}i32", i),
            Value::I64(i) => write!(f, "{}i64", i),
            Value::U32(i) => write!(f, "{}u32", i),
            Value::U64(i) => write!(f, "{}u64", i),
            Value::F32(i) => write!(f, "{}f32", i),
            Value::F64(i) => write!(f, "{}f64", i),
            Value::String(s) => write!(f, "{:?}", s),
            Value::Map(map) => {
                write!(f, "{{\n\tmap: {:?},\n\tlist: {:?}\n}}", map.map, map.list)
            }
            Value::ExternSymbol(s) => {
                write!(f, "{:?}", s)
            }
            Value::Function(func) => write!(f, "{:?}", func),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LMap {
    map: HashMap<String, Object<Value>>,
    list: Vec<Value>,
}

impl LMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            list: Vec::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.map.len() + self.list.len()
    }

    pub fn set(&mut self, k: String, v: Object<Value>) {
        self.map.insert(k, v);
    }

    pub fn get(&mut self, k: &String) -> Object<Value> {
        match self.map.get(k) {
            Some(val) => val.clone(),
            None => obj!(Value::Null),
        }
    }
}

pub struct LFn {
    body: Box<dyn Fn(&mut CallerCtx)>,
}

impl LFn {
    pub fn call(&self, ctx: &mut CallerCtx) {
        (self.body)(ctx);
    }
}

impl<T: Fn(&mut CallerCtx) + 'static> From<T> for LFn {
    fn from(value: T) -> Self {
        Self {
            body: Box::new(value),
        }
    }
}

impl fmt::Debug for LFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "function: {:?}", (&*self.body).type_id())
    }
}

#[derive(Debug)]
pub struct CallerCtx {
    pub args: Vec<Object<Value>>,
    pub return_t: Object<Value>,
    error: Option<&'static str>,
}

impl CallerCtx {
    pub fn args(&mut self) -> &mut Vec<Object<Value>> {
        &mut self.args
    }

    pub fn push_arg(&mut self, value: Object<Value>) {
        self.args.push(value);
    }

    pub fn has_errored(&self) -> bool {
        self.error.is_some()
    }

    pub fn error_unwrap(&self) -> &str {
        self.error.unwrap()
    }

    pub fn set_error(&mut self, error: &'static str) {
        self.error = Some(error);
    }

    pub fn set_return_t(&mut self, value: Object<Value>) {
        self.return_t = value;
    }

    pub fn return_t(self) -> Object<Value> {
        self.return_t
    }
}

impl Default for CallerCtx {
    fn default() -> Self {
        Self {
            args: vec![],
            return_t: Object::new(Value::Null),
            error: None,
        }
    }
}
