#[derive(Clone, Debug)]
pub struct Segment<'a> {
    /// Content
    pub content: &'a str,

    /// Color information
    /// This is an ANSI escape code
    pub color: &'a str,
}

/// The prompt seperated by color to be easier to handle for non-terminal enviornments.
#[derive(Debug)]
pub struct Prompt<'a> {
    pub segments: Box<[Segment<'a>]>,
}

impl<'a> Prompt<'a> {
    pub fn new(prompt: &'a str) -> Self {
        let mut result = Vec::new();
        let mut chars = prompt.chars();
        chars.next();

        // Stupid hack to get the escape code in the color
        for x in prompt.split_inclusive(|_| chars.next() == Some('\x1b')) {
            if let Some(end) = x.find("m") {
                result.push(Segment {
                    color: &x[..end],
                    content: &x[end + 1..],
                })
            };
        }

        Self {
            segments: result.into_boxed_slice(),
        }
    }
}
