use std::marker::PhantomData;

use marco_polo_rs_macros::Paginate;

mod database;

#[allow(dead_code)]
#[derive(Paginate)]
struct Test<'a, T>
where
    T: 'a,
{
    a: i32,
    b: String,
    c: &'a str,
    #[order_field_name = "foo"]
    d: PhantomData<T>,
}

#[allow(dead_code)]
#[derive(Paginate)]
struct TestB {
    a: i32,
    b: String,
}

#[allow(dead_code)]
#[derive(Paginate)]
struct TestC<'a, 'b> {
    a: i32,
    b: &'a str,
    c: &'b str,
}

trait TestTraitA {}

trait TestTraitB {}

#[allow(dead_code)]
#[derive(Paginate)]
struct TestD<R, T>
where
    R: TestTraitA,
    T: TestTraitB,
{
    a: i32,
    b: String,
    c: PhantomData<R>,
    d: PhantomData<T>,
}

#[allow(dead_code)]
#[derive(Paginate)]
struct TestE<'a, R: TestTraitA, T: TestTraitB> {
    a: i32,
    b: String,
    c: &'a str,
    d: PhantomData<R>,
    e: PhantomData<T>,
}

#[test]
fn test_enum_creation() {
    let field = TestOrderFields::A;
    match field {
        TestOrderFields::A => {}
        TestOrderFields::B => {}
        TestOrderFields::C => {}
        TestOrderFields::D => {}
    }
}

#[test]
fn test_name() {
    let a = TestOrderFields::A;
    let b = TestOrderFields::B;
    let c = TestOrderFields::C;
    let d = TestOrderFields::D;

    assert_eq!(a.name(), "a");
    assert_eq!(b.name(), "b");
    assert_eq!(c.name(), "c");
    assert_eq!(d.name(), "foo");
}
