use warp::Filter;

pub fn get_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("files")
        .and(warp::get())
        .map(|| warp::reply::json(&vec!["file1.txt", "file2.jpg"]))
}
