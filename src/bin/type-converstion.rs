use std::convert::TryFrom;

struct NonZeroNumber(i32);

struct PositiveSingleDigitNumber(i32);

impl TryFrom<i32> for PositiveSingleDigitNumber {
    type Error = &'static str;

    fn try_from(number: i32) -> Result<Self, Self::Error> {
        if number < 0 || number >= 10 {
            Err("Not an single digit positive number")
        } else {
            Ok(Self(number))
        }
    }
}

impl From<i32> for NonZeroNumber {
    fn from(number: i32) -> Self {
        if number <= 0 {
            Self(1)
        } else {
            Self(number)
        }
    }
}

fn main() {
    let num_one = NonZeroNumber::from(1);
    let num_two: NonZeroNumber = 0.into();

    println!("{:?} {:?}", num_one.0, num_two.0);

    match PositiveSingleDigitNumber::try_from(12) {
        Ok(number) => println!("{:?}", number.0),
        Err(reason) => println!("{:?}", reason),
    };
}
