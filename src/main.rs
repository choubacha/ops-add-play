use std::ops::Add;

#[derive(Debug, Default, PartialEq)]
struct MyValue {
    a: u32,
    b: String
}

impl Add for MyValue {
    type Output = MyValue;

    fn add(self, rhs: MyValue) -> MyValue {
        &self + &rhs
    }
}

impl<'a> Add<&'a MyValue> for MyValue {
    type Output = MyValue;

    fn add(self, rhs: &MyValue) -> MyValue {
        &self + rhs
    }
}

impl<'a> Add for &'a MyValue {
    type Output = MyValue;

    fn add(self, rhs: &MyValue) -> MyValue {
        MyValue {
            a: self.a + rhs.a,
            b: format!("{}{}", self.b, rhs.b),
        }
    }
}

impl<'a> Add<MyValue> for &'a MyValue {
    type Output = MyValue;

    fn add(self, rhs: MyValue) -> MyValue {
        MyValue {
            a: self.a + rhs.a,
            b: format!("{}{}", self.b, rhs.b),
        }
    }
}

fn main() {
    let values = vec![
        MyValue { a: 2, b: "hello ".to_string() },
        MyValue { a: 4, b: "there ".to_string() },
        MyValue { a: 5, b: "world ".to_string() },
        MyValue { a: 10, b: "earth.".to_string() },
    ];

    println!("{:?}", &values[0] + &values[1]);

    let result: MyValue = values.iter().fold(MyValue::default(), |a, b| a + b);
    println!("{:?}", result);
}

#[test]
fn can_add_two_values() {
    assert_eq!(MyValue { a: 1, b: "test".to_string() } + MyValue { a: 1, b: "test".to_string() },
               MyValue { a: 2, b: "testtest".to_string() });
}

#[test]
fn can_add_two_references() {
    let v1 = MyValue { a: 1, b: "test".to_string() };
    let v2 = MyValue { a: 1, b: "test".to_string() };
    assert_eq!(&v1 + &v2,
               MyValue { a: 2, b: "testtest".to_string() });
}

#[test]
fn can_add_reference_to_value() {
    let v1 = MyValue { a: 1, b: "test".to_string() };
    let v2 = MyValue { a: 1, b: "test".to_string() };
    assert_eq!(&v1 + v2,
               MyValue { a: 2, b: "testtest".to_string() });
}

#[test]
fn can_add_value_to_reference() {
    let v1 = MyValue { a: 1, b: "test".to_string() };
    let v2 = MyValue { a: 1, b: "test".to_string() };
    assert_eq!(v1 + &v2,
               MyValue { a: 2, b: "testtest".to_string() });
}
