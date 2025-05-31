use derive_getters::Getters;

#[derive(Debug, Getters)]
pub struct SingletonInteger {
    value: i64,
}

pub fn singleton_integer() -> &'static SingletonInteger {
    unsafe {
        if SINGLETON_INTEGER.is_none() {
            SINGLETON_INTEGER = Some(SingletonInteger { value: 123 });
        }
        SINGLETON_INTEGER.as_ref().unwrap()
    }
}

static mut SINGLETON_INTEGER: Option<SingletonInteger> = None;
