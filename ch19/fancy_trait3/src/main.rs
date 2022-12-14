use std::ops::Add;
#[derive(Debug,PartialEq)]
struct Millimeters(u32);
#[derive(Debug,PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0*1000))
    }
}


fn main() {
    assert_eq!(Millimeters(50)+Meters(5),Millimeters(5050));
    println!("Milimeters(50) + Meters(5) = {:?} = Millimeters : {:?}",Millimeters(50)+Meters(5),Millimeters(5050));
}
