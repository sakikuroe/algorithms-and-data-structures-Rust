use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, Debug)]
pub struct Argument {
    x: isize,
    y: isize,
}
impl Argument {
    pub fn new(x: isize, y: isize) -> Self {
        Argument {
            x,
            y,
        }
    }
}
impl Ord for Argument {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x == 0 && self.y ==0 {
            return Ordering::Less;
        }
        if other.x == 0 && other.y ==0 {
            return Ordering::Greater;
        }
        if (self.y > 0 || (self.y == 0 && self.x > 0))
            && (other.y < 0 || (other.y == 0 && other.x < 0))
        {
            return Ordering::Less;
        } else if (other.y > 0 || (other.y == 0 && other.x > 0))
            && (self.y < 0 || (self.y == 0 && self.x < 0))
        {
            return Ordering::Greater;
        } else {
            let cross = self.x * other.y - other.x * self.y;
            if cross > 0 {
                return Ordering::Less;
            } else if cross == 0 {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        }
    }
}
impl PartialOrd for Argument {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Argument {
    fn eq(&self, other: &Self) -> bool {
        self.y * other.x == self.x * other.y
    }
}