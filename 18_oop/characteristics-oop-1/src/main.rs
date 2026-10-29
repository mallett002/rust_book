fn main() {}

#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        }
    }

    pub fn add(&mut self, item: i32) {
        self.list.push(item);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        match self.list.pop() {
            Some(item) => {
                self.update_average();
                Some(item)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_item() {
        let mut collection = AveragedCollection::new();

        println!("{collection:?}");

        assert_eq!(collection.average, 0.0);

        collection.add(5);
        collection.add(7);

        assert_eq!(collection.average, 6.0);
    }
}
