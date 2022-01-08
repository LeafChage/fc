pub struct DisplayBinary<'a>(pub &'a [u8]);

impl<'a> std::fmt::Display for DisplayBinary<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let DisplayBinary(binary) = self;
        let mut texts = vec![];
        for i in 0..(binary.len() / 16) {
            let first = i * 16;
            let end = first + 16;
            texts.push(
                binary[first..end]
                    .iter()
                    .fold(format!("{:<4x}: ", i), |a, b| format!("{}{:<4x}", a, b)),
            )
        }
        write!(f, "{}", texts.join("\n"))
    }
}
