use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum PathArg {
    Pipe,
    Path(PathBuf),
}

impl FromStr for PathArg {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            Ok(PathArg::Pipe)
        } else {
            PathBuf::from_str(s).map(PathArg::Path)
        }
    }
}

impl PathArg {
    pub fn read(&self) -> std::io::Result<String> {
        match self {
            PathArg::Pipe => {
                let stdin = std::io::stdin();
                std::io::read_to_string(stdin)
            }
            PathArg::Path(p) => fs_err::read_to_string(p),
        }
    }

    pub fn open(&self, append: bool) -> std::io::Result<Output> {
        match self {
            PathArg::Pipe => Ok(Output::stdout()),
            PathArg::Path(p) => {
                let file = if append {
                    fs_err::OpenOptions::new().append(true).open(p)
                } else {
                    fs_err::OpenOptions::new().create(true).open(p)
                };
                file.map(Output::File)
            }
        }
    }
}

pub enum Output {
    Stdout(std::io::Stdout),
    File(fs_err::File),
}

impl Output {
    fn stdout() -> Self {
        Self::Stdout(std::io::stdout())
    }
}

impl std::io::Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Output::Stdout(o) => o.write(buf),
            Output::File(o) => o.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Output::Stdout(o) => o.flush(),
            Output::File(o) => o.flush(),
        }
    }
}
