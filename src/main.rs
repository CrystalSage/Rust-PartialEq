use std::cmp::Ordering;


#[derive(PartialOrd, PartialEq)]
enum Size{
  Small,
  Medium,
  Large,
}

#[allow(dead_code)]
struct Umbrella{
  size : Size,
  cost : u16,
}

impl PartialOrd for Umbrella {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.size.partial_cmp(&other.size)
    }
}

impl PartialEq for Umbrella {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}

fn main(){
  let u1 = Umbrella{ size: Size::Small, cost: 20};
  let u2 = Umbrella{ size: Size::Medium, cost: 20};
  let u3 = Umbrella{ size: Size::Large, cost: 40};
  let u4 = Umbrella{ size: Size::Large, cost: 40};


  assert!(u1 < u2);
  assert!(u1 < u3);
  assert!(u1 < u2 && u2 < u3);
  assert!(u4 == u3);

  println!("Passed all checks!");
}
