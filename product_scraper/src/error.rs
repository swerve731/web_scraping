
#[derive(derive_more::From, Debug)]
pub enum Error<'a> {
    NotFound(String),

    #[from]
    WebDriver(fantoccini::error::WebDriver),

    #[from]
    FantocciniCmd(fantoccini::error::CmdError),

    #[from]
    FantocciniSession(fantoccini::error::NewSessionError),
    #[from]
    Window(fantoccini::error::InvalidWindowHandle),

    #[from]
    Scraper(scraper::error::SelectorErrorKind<'a>),


}

pub type Result<T> = std::result::Result<T, Error<'static>>;