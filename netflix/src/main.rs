mod fetch_top_movies;
mod find_median_age;
mod group_similar_title;

fn main() {
    group_similar_title::driver();
    fetch_top_movies::driver();
    find_median_age::driver(); // TODO: Implement min-max heap
}
