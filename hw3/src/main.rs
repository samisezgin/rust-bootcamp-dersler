// Step 2: Define the FilterCondition struct with the desired type for filtering.
struct FilterCondition<T> {
    desired_value: T,
}

// Step 3: Implement the is_match method on the FilterCondition struct.
impl<T: PartialEq> FilterCondition<T> {
    // Method to check if an item matches the filter condition.
    fn is_match(&self, item: &T) -> bool {
        // Compare the item with the desired_value using PartialEq trait.
        *item == self.desired_value
    }
}

// Step 4: Define the custom_filter function to filter elements based on the condition.
fn custom_filter<T>(collection: &[T], condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq, // We require the PartialEq trait for comparison.
    T: Clone,    // We require the Clone trait to create a new Vec.
{
    // Use iter() to create an iterator over the collection, and cloned() to clone the items.
    // We need to clone the items because the Vec is moved into the filter closure.
    // This way, we avoid ownership issues when creating the new filtered Vec.
    collection
        .iter()
        .cloned()
        // Use filter() to keep only the items that match the filter condition.
        // The closure passed to filter() calls the is_match() method on the FilterCondition object.
        // The closure returns true for items that match the filter condition.
        .filter(|item| condition.is_match(item))
        // Use collect() to create a new Vec with the filtered items.
        .collect()
}

// Step 5: Create a collection and a FilterCondition object in the main function.
fn main() {
    // Create a collection of integers.
    let my_collection = vec![1, 2, 3, 4, 5];

    // Create a FilterCondition object with the desired value for filtering.
    let filter_condition = FilterCondition { desired_value: 3 };

    // Step 6: Call the custom_filter function and store the result.
    let filtered_result = custom_filter(&my_collection, &filter_condition);

    // Step 7: Print the filtered result to the console.
    println!("Filtered result: {:?}", filtered_result);


    // Create a collection of char.
    let my_collection = vec!['s', ' ', '3', '4', 's', 'a', 'm', 'i'];

    // Create a FilterCondition object with the desired value for filtering.
    let filter_condition = FilterCondition { desired_value: 's' };

    // Step 6: Call the custom_filter function and store the result.
    let filtered_result = custom_filter(&my_collection, &filter_condition);

    // Step 7: Print the filtered result to the console.
    println!("Filtered result: {:?}", filtered_result);
}