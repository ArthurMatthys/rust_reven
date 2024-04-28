mod changeme;
use changeme::{MyCallback, MyCallbackData, MyStruct, MyTrait};

fn main() {
    let mut s = MyStruct {
        callbacks: Vec::new(),
        data: [1, 2, 3],
    };

    s.set_callback(MyCallback {
        callback: Box::new(|data: &MyCallbackData| {
            println!("Callback called with data {:?}", data);
        }),
    });

    s.do_something();
}
