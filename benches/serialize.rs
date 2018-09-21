#![feature(test)]
extern crate test;
extern crate serde_tera;
#[macro_use]
extern crate serde_derive;

use serde_tera::to_value;

static LOREM: &'static str = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas et consequat justo. In varius nisl eu placerat fermentum. In lacinia pellentesque metus, in pharetra libero pulvinar et. Aliquam ac erat ligula. Cras aliquet dolor non nibh fermentum, ut dapibus tortor convallis. Vivamus scelerisque felis varius tortor commodo, eget suscipit tortor fringilla. Integer sollicitudin, libero in rutrum vulputate, libero purus ultricies augue, a ullamcorper dui odio non lorem.

Nam et tellus velit. Cras aliquam faucibus arcu vel tempus. Maecenas nec porta elit, nec blandit ipsum. Fusce molestie hendrerit feugiat. Nullam sem quam, volutpat at odio sit amet, lobortis dignissim magna. Donec tempus vulputate diam, et malesuada elit rhoncus in. Integer nec libero odio. Etiam a quam at velit scelerisque euismod. Donec felis nibh, tincidunt vel lacus sit amet, fermentum malesuada quam.

Morbi ultrices vel eros in fringilla. Ut vel ultricies ante. Fusce ac laoreet mi. Vivamus sollicitudin vel ligula ut eleifend. Aliquam erat volutpat. Aliquam imperdiet lacus vitae dictum maximus. Etiam egestas velit sit amet quam malesuada ullamcorper. Cras mi libero, fermentum sed tempor id, ultrices quis nulla.

Sed faucibus, lectus ac auctor porttitor, nibh justo interdum sapien, nec ornare purus metus ut tortor. Nulla porta nunc eget mi pharetra commodo. In venenatis ex non ipsum hendrerit, nec porta sapien fringilla. Sed id semper justo. Donec a dui suscipit, congue magna sed, tempor orci. In efficitur eget arcu et blandit. Nunc eget odio eu massa maximus semper. Suspendisse sed hendrerit sapien, id elementum mauris. Curabitur eleifend ullamcorper ultricies. Maecenas congue neque elit, in imperdiet dui aliquet eget. Fusce vitae neque tortor. Curabitur id auctor dui. Vestibulum a sem in tellus venenatis tincidunt eu non risus. Fusce sed libero augue. Morbi eget finibus magna. Donec rutrum, nisl eu auctor dignissim, tellus ante tincidunt dolor, sit amet hendrerit sem purus vitae dolor.

Curabitur ultrices augue eget leo porta, eu eleifend velit dignissim. Nam ligula risus, pellentesque eu tempor in, rhoncus viverra augue. Proin eu justo et mauris faucibus aliquam quis vitae libero. Nunc scelerisque, nunc ut ultricies ullamcorper, libero ante semper sapien, quis egestas diam orci quis lorem. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Sed feugiat gravida pulvinar. Quisque sit amet diam in odio maximus mollis sit amet at lacus. Vivamus tincidunt justo vel tellus pretium, vitae ullamcorper libero accumsan. Morbi efficitur pellentesque nisl, in finibus risus tempus et. Quisque euismod diam consectetur iaculis fermentum.
"#;

#[derive(Debug, Serialize)]
struct Product {
    name: String,
    manufacturer: String,
    price: i32,
    summary: String,
}
impl Product {
    pub fn new() -> Product {
        Product {
            name: "Moto G".to_owned(),
            manufacturer: "Motorala".to_owned(),
            summary: "A phone".to_owned(),
            price: 100,
        }
    }
}

#[derive(Serialize, Clone)]
struct DataWrapper {
    v: String,
}

#[derive(Serialize, Clone)]
struct RowWrapper {
    real: Vec<DataWrapper>,
    dummy: Vec<DataWrapper>,
}

#[bench]
fn bench_serializing_struct(b: &mut test::Bencher) {
    let prod = Product::new();
    b.iter(|| to_value(&prod).unwrap());
}


#[bench]
fn bench_serializing_string(b: &mut test::Bencher) {
    b.iter(|| to_value(&LOREM).unwrap());
}

#[bench]
fn bench_serializing_big_struct(b: &mut test::Bencher) {
    let real: Vec<DataWrapper> =
        (1..1000).into_iter().map(|i| DataWrapper { v: format!("n={}", i) }).collect();
    let dummy: Vec<DataWrapper> =
        (1..1000).into_iter().map(|i| DataWrapper { v: format!("n={}", i) }).collect();
    let rows = RowWrapper { real, dummy };

    b.iter(|| to_value(&rows).unwrap());
}

