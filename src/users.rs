use lunatic::{abstract_process, ap::Config};

// (username, color)
pub struct Users(Vec<(String, String)>);

#[abstract_process(visibility = pub)]
impl Users {
    #[init]
    fn init(_: Config<Self>, _: ()) -> Result<Self, ()> {
        Ok(Self(vec![
            ("sparkles".to_owned(), "#7CDEDC".to_owned()),
            ("bubbles".to_owned(), "#FFA400".to_owned()),
            ("shimmer".to_owned(), "#FA7921".to_owned()),
            ("glimmer".to_owned(), "#276FBF".to_owned()),
            ("blossom".to_owned(), "#E63462".to_owned()),
            ("snowflake".to_owned(), "#C7EFCF".to_owned()),
        ]))
    }

    #[handle_request]
    fn get_user(&mut self) -> (String, String) {
        self.0.pop().unwrap()
    }
}
