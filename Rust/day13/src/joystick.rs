use intcode::future::{stream::Stream, Poll};

use std::{
    cell::Cell,
    io::BufRead,
};

pub struct JoyStick<'a, R: BufRead> {
    input: R,
    buffer: String,
    autoinput: String,
    saved: String,
    should_display: &'a Cell<bool>,
}

impl<'a, R: BufRead> JoyStick<'a, R> {
    pub fn new(input: R, should_display: &'a Cell<bool>) -> Self {
        Self {
            input,
            buffer: String::with_capacity(10),
            saved: String::new(),
            should_display,
            autoinput: "nnnnnnnnnnnnnnnrrrrrrrrrrrrrrnnnnnnnnnnnnnnnnnnnnlllllllllllnrrrnrrrrnllllnlllnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrrrrrlnrrnrrrrrrnnnlnnnnnllllllnnrnnnnnnnnnnnnllnrrnnnnnnrrrrnlllnrrrnnnnnnnnlnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrnllnnnnnnnnnnrrrnnnnnnnnnnnnnnnnnnnnnlllllllnnnnnnnnnnnnnnrnnnnnnnnnllllllllllllllrnnnnnnnlllllnllllllnnnnnnnrnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnllnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrrrrrrrrrrrrnnnnnnnlllllllnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrrrrrrrrrrrrrrrrrrrrrrrnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnllllllllllllnnnnnnnnnnnnnnrrrrrrnnnnnnnrnnnnnnnnnnnnnnnnnnnnnnnnnnnlllrrnnrnnnnnnnnnnnnnllllllllllllllllllnllllllllrrnnrrrnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrrrnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrrnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrrrrrrrnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnlllllllnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnlllllllllllllllrrnlnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrnnnrnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrrrrrrrrrrrnnnnnnnnnnnnnnnnnnnnlnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrrrnnnnnnnnnnnnnnnnnnlnnnnnnnlnnnnllllllllllllrnnrrrnnnnnllnnnnnnnnnnnnnnnrrrrrrrrrrrrrrrrnnnnnnnnnnnnnnnnnrnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnllllllllllllllrrrrrrrrrrrrrnnnnnnnnnnnnnnnnnnrnnnnnllllnnnnnnnnnnnnllllllllllnrrrrrrrrrrrrrrlnnnnnnnnnnnnnnnnrnnnnnnlllllllnnnnnnlllllnnnnllnnnnnnnrrrrrrrrrrrrrllnnnnnnnnnrrrrrlllllllllllllllllnnnnnnnnnnnnrnnnnnnnnnnnnnnnnnnrrrrrrrrrrrrrrrnnnlllllllllllllllllllrrrrrnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrrrrrrrlllllllnnnnnnnnnnnnlllnnnnnnnnrnrrrnnnnnnnnnnnrrrrrnnnnnnnrrrrrrnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnlllllllllllllllnrnnnnnnnnnnnnnnnnnnnnnnrrrrrrrrnnnnnnnnnnnnnnnnnnnnnnnllllnnnnnrrnnnnnnnnnnrnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrnnnnnnllllnnnnnnnnnnnnnnnnnnnnnnnnnnnnnllnnnnnnnnnnnnnnnnnnnnnnnnnnrrrrrnnnnnnnnnnnnnnnnnnnllnnnnnnnlllnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrnnnnnnnrrrnnnnnnnnnnnnnnnnnnnlllnnnnnnnnnlnnnnnnnnnnnnnnnnnnnnnnnnnnnrrrnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrnnnrnnnnnnnnnnnnnnnnnnllllllllnnnrnlnnnnnnnnnnnrrrrrrrrnnnnnnrrrrrrnnnnnnnllllllllllllllllllllllnrrrnnnnnnnnnnnnnrrrrrrrrrrrrrrrrrrrnnnnnnnnnnllllllllllllllnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrnnrrrrrrnnnnnnnnnnnnnnnnnnnnnnnnnnnnnlnlnnnnnnnnlnnnnnnnnnlllnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrrrrnnnnnnnnnnnnnnnnnnnnnnnnnlnnnllllnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrnnnnrrrrrlnnnnnnlllllnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrrrrrrrrrnnnnnnnnnnlllllllllnnnlllllllnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnrrrrrrrrrrrrrrrrrnnnnnnnnnnnnlllllllllllllllnnnnnlnnnnnnnnnnnnnrrrrrrrrrrrrrrnnrrnnnnnnnnnnnllllllllllllnnnnlllnlnnnnnnnnnnnnnnnrrrrrrrrrrrrrrrrnnnnnnnnnnnnnnllllllllllllllllllnrnnnnnnnnrrrrrrrrrrrrrrrrrrrrrrrlllllllllllllllllllllllnnnnllllllnnnnnnnnnnnnnnnnnnnnnnrrrrrrrrrrrrrrrrrrrrrrrrrrrrrllllllllllllllllllllllnnnnnnnlnnnnnnnnnnnnnnnrrrrrrrrrrrrrrrrrnnnnnnnnnnnlllllllllllllllllllrrnnnnnnnnrrrrrrrrrrnnrnnrrrrrrrrrrrnnnnnllllllllllllllllllllllnnnnnnnnnnnnnnnnnrrrrrrrrrrrrrrrrnnnnnnnnnnnnnnnnnnnnnnnlllllllllllnnnnnnnnnnnnnnnnnnrrrrrnnnrrrrrrnnnnnnnnnnnlllllllllllllllllnnnrnnnnnnnnnnrrrrrrrrrrrnnrrrrrrrrrrnnnnnllllllllllllllllllllllnnnnnnnnnrrrrrrrrrrrrrrrrnnnrrrrrrrrlnlllllllllllllllllllllllnnnllllnnnnnrrrrrrrrrrrrrrrrrrrrrrrrrrrrrllllllllllllllllllllllllllllllnrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrnnllllllllllllllllllll".chars().rev().collect::<String>(),
        }
    }

    fn read_char(&mut self) -> char {
        if self.autoinput.len() == 1 {
            self.should_display.set(true);
        }
        match self.autoinput.pop() {
            Some(c) => c,
            None => {
                let c = loop {
                    println!("Provide n, l, or r");
                    let _ = self.input.read_line(&mut self.buffer);
                    match self.buffer.as_str().trim() {
                        "n" => break 'n',
                        "l" => break 'l',
                        "r" => break 'r',
                        _ => self.buffer.clear(),
                    };
                };

                self.saved.push(c);
                self.buffer.clear();
                c
            }
        }
    }
}

impl<'a, R: BufRead> Stream for JoyStick<'a, R> {
    type Item = isize;
    fn poll_next(&mut self) -> Poll<Option<isize>> {
        let output = match self.read_char() {
            'n' => 0,
            'l' => -1,
            'r' => 1,
            _ => panic!(),
        };

        self.buffer.clear();

        Poll::Ready(Some(output))
    }
}
