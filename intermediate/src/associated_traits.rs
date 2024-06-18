//
//
//  Associated Traits
//
//
#[derive(Debug)]
struct Km {
    value: u32,
}

#[derive(Debug)]
struct Kmh {
    value: u32,
}
// impl Kmh {
//     fn distance_in_three_hours(&self) -> Km {
//         Km {
//             value: self.value * 3,
//         }
//     }
// }

#[derive(Debug)]
struct Miles {
    value: u32,
}

#[derive(Debug)]
struct Mph {
    value: u32,
}
// impl Mph {
//     fn distance_in_three_hours(&self) -> Miles {
//         Miles {
//             value: self.value * 3,
//         }
//     }
// }

// trait DistanceThreeHours {
//     fn distance_in_three_hours(&self) -> Self;
// }

// impl DistanceThreeHours for Kmh {
//     fn distance_in_three_hours(&self) -> Self {
//         Self {
//             value: self.value * 3,
//         }
//     }
// }

// impl DistanceThreeHours for Mph {
//     fn distance_in_three_hours(&self) -> Self {
//         Self {
//             value: self.value * 3,
//         }
//     }
// }

trait DistanceThreeHours {
    type Distance; // Associated Type
    fn distance_in_three_hours(&self) -> Self::Distance;
}

impl DistanceThreeHours for Kmh {
    type Distance = Km;
    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

impl DistanceThreeHours for Mph {
    type Distance = Miles;
    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

fn main() {
    let speed_kmh = Kmh { value: 90 };
    let distance_km = speed_kmh.distance_in_three_hours();
    println!("{:?}", distance_km);

    let speed_mph = Mph { value: 50 };
    let distance_miles = speed_mph.distance_in_three_hours();
    println!("{:?}", distance_miles);
}
