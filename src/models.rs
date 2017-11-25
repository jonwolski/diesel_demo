use schema::posts;

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[derive(Serialize, Deserialize)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

/*
impl FromData for NoteData {
	type Error = String;

	fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
		let reader = data.open();
		match serde_json::from_reader(reader).map(|val| val) {
			Ok(value) => Success(value),
			Err(e) => Failure((Status::BadRequest, e.to_string())),
		}
	}
}
*/
