mod fetch_top_movies;
mod group_similar_title;

fn main() {
    group_similar_title::driver();
    fetch_top_movies::driver();
}
