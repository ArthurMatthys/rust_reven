#[derive(Debug)]
pub struct MyCallbackData<'a> {
    data: &'a [u8],
}

pub struct MyCallback {
    pub(crate) callback: Box<dyn Fn(&MyCallbackData)>,
}

/// Allow us to store callbacks and do something with our struct
pub trait MyTrait<'a> {
    fn set_callback(&mut self, cb: MyCallback);
    fn do_something(&self);
}

/// Stores functions pointers and their parameters
pub struct MyStruct {
    pub(crate) callbacks: Vec<MyCallback>,
    pub(crate) data: [u8; 3],
}

impl<'a> MyTrait<'a> for MyStruct {
    fn set_callback(&mut self, cb: MyCallback) {
        self.callbacks.push(cb);
    }

    /// We're calling every callbacks with our initial data
    fn do_something(&self) {
        let cb_data = MyCallbackData { data: &self.data };
        for cb in &self.callbacks {
            (cb.callback)(&cb_data);
        }
    }
}
