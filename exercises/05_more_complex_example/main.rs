////////// DO NOT CHANGE BELOW HERE /////////
// This function should be called by the `show_output!()` macro
#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn show(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! for_2d {
    ($var1:ident <$var1_ty:ty> in $range_1:expr, $var2:ident <$var2_ty:ty> in $range_2:expr, $code:block) => {
        for $var1 in $range_1 {
            let $var1: $var1_ty = $var1;
            for $var2 in $range_2 {
                let $var2: $var2_ty = $var2;
                $code
            }
        }
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    for_2d!(row <i32> in 1..5, col <i32> in 2..7, {
        (Coordinate {x: col, y: row}).show()
    });

    let values = [1, 3, 5];

    for_2d!(x <u16> in values, y <u16> in values, {
        (Coordinate {x: x.into(), y: y.into()}).show()
    });
}
