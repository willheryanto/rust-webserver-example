//use crate::modules::movie::domain::value_objects::Title;
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize)]
pub struct MovieEntity {
    pub id: u64,
    pub title: String,
    pub image_url: String,
    pub overview: String,
    pub release_date: NaiveDate,
    pub vote_average: f64,
}

//pub struct MovieProps {
//pub id: u64,
//pub title: Title,
//pub image_url: String,
//pub overview: String,
//pub release_date: NaiveDate,
//pub vote_average: f64,
//}


//impl MovieEntity {
//pub fn create(props: MovieProps) -> MovieEntity {
//MovieEntity {
//id: props.id,
//title: props.title.as_ref().to_string(),
//image_url: props.image_url,
//overview: props.overview,
//release_date: props.release_date,
//vote_average: props.vote_average,
//}
//}
//}
