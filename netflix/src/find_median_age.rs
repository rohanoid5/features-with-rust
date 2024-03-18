/*
 Our third task is building a filter that will fetch relevant content based on the age of the users for a specific country or region.
 For this, we use the median age of users and the preferred content for users that fall into that specified category.
*/

struct Users {
    list_of_age: Vec<i32>,
    median: f32,
}

impl Users {
    fn new(list_of_age: &mut Vec<i32>) -> Self {
        let mut user = Self {
            list_of_age: list_of_age.to_owned(),
            median: 0.0,
        };
        user.calculate_median();

        user
    }

    fn calculate_median(&mut self) {
        self.list_of_age.sort_by(|a, b| a.cmp(b));

        let median;
        let number_of_users = self.list_of_age.len();
        let mid = number_of_users / 2;

        if number_of_users % 2 == 0 {
            median =
                ((self.list_of_age[mid] as f32 + self.list_of_age[mid - 1] as f32) / 2.0) as f32;
        } else {
            median = self.list_of_age[mid] as f32;
        }

        self.median = median;
    }

    fn add_user(&mut self, age: i32) {
        self.list_of_age.push(age);
        self.calculate_median();
    }
}

pub fn driver() {
    let mut ages = vec![3, 13, 2, 34, 11, 26, 47];
    let mut users = Users::new(&mut ages);
    println!("Median age is: {}", users.median);

    users.add_user(32);
    println!("Median age is: {}", users.median);
}
