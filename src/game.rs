use std::fmt;

const WORD_LENGTH: usize = 5;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Placement {
    Correct,
    Misplaced,
    Absent,
}

impl fmt::Display for Placement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Placement::Correct => "🟩",
            Placement::Misplaced => "🟨",
            Placement::Absent => "⬛️",
        };

        write!(f, "{}", s)
    }
}

pub struct GuessResult {
    pub guess: String,
    pub placement: [Placement; WORD_LENGTH],
}

impl fmt::Display for GuessResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in self.guess.chars() {
            write!(f, "{}\t", c)?;
        }
        writeln!(f)?;
        for p in self.placement {
            write!(f, "{}\t", p)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Wordle {
    pub word: String,
    pub guesses: u32,
}

impl Wordle {
    pub fn new(word: String) -> Self {
        Self { word, guesses: 0 }
    }

    pub fn guess(&mut self, guess: String) -> GuessResult {
        let mut placement = [Placement::Absent; WORD_LENGTH];
        let mut used = [false; WORD_LENGTH];

        for (i, (a, g)) in self.word.chars().zip(guess.chars()).enumerate() {
            if a == g {
                placement[i] = Placement::Correct;
                used[i] = true;
            }
        }

        for (i, g) in guess.chars().enumerate() {
            if placement[i] == Placement::Correct {
                continue;
            }
            if self.word.chars().enumerate().any(|(i, c)| {
                if c == g && !used[i] {
                    used[i] = true;
                    true
                } else {
                    false
                }
            }) {
                placement[i] = Placement::Misplaced;
            }
        }

        self.guesses += 1;

        GuessResult { guess, placement }
    }
}
