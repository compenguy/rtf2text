pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub enum ErrorKind {
    InputIo = 1,
    OutputIo = 2,
    Parse = 3,
}

impl std::convert::Into<i32> for ErrorKind {
    fn into(self) -> i32 {
        self.code()
    }
}

impl ErrorKind {
    pub fn code(&self) -> i32 {
        self.clone() as i32
    }

    pub fn name(&self) -> &'static str {
        match self {
            ErrorKind::InputIo => "InputIo",
            ErrorKind::OutputIo => "OutputIo",
            ErrorKind::Parse => "Parse",
        }
    }

    /*
    pub fn description(&self) -> &'static str {
        match self {
            ErrorKind::InputIo => "An error occurred while reading the specified input file",
            ErrorKind::OutputIo => "An error occurred while writing to the specified output file",
            ErrorKind::Parse => "An error occurred while parsing the input document",
        }
    }
    */
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: Option<String>,
    inner: Option<Box<dyn std::error::Error>>,
}

impl std::error::Error for Error {
    // I just can't figure out how to get the boxed error we have to coerce to the required type
    // here
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} Error:", self.kind.name())?;
        if let Some(message) = &self.message {
            write!(f, " {}", message)?;
        }
        write!(f, "")
    }
}

impl Error {
    pub fn new(
        kind: ErrorKind,
        message: Option<String>,
        source: Option<Box<dyn std::error::Error>>,
    ) -> Self {
        Self {
            kind,
            message,
            inner: source,
        }
    }

    pub fn from_input_error(source: std::io::Error) -> Self {
        Self::new(ErrorKind::InputIo, None, Some(Box::new(source)))
    }

    pub fn from_output_error(source: std::io::Error) -> Self {
        Self::new(ErrorKind::OutputIo, None, Some(Box::new(source)))
    }

    pub fn code(&self) -> i32 {
        self.kind.code()
    }
}
