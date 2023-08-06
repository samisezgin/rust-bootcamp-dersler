
struct FilterCondition<T> {
    desired_value: T,
}


impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        *item == self.desired_value
    }


}

fn custom_filter<T>(collection: &[T], condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq,
    T: Clone,
{
    collection
        .iter()
        .cloned()
        .filter(|item| condition.is_match(item))
        .collect()
}

fn main() {
    let my_collection = vec![1, 2, 3, 4, 5];
    let filter_condition = FilterCondition { desired_value: 3 };

    let filtered_result = custom_filter(&my_collection, &filter_condition);

    println!("Filtered result: {:?}", filtered_result);
}
