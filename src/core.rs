use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use crate::core::GuessBlock::{Wrong, Correct, Partial};

pub struct WordleWord {
    word: [char; 5],
    idx: [bool; 26],
}

#[derive(Copy, Clone, PartialEq)]
pub enum GuessBlock { Wrong, Partial, Correct }

pub struct GuessResult([GuessBlock; 5]);

impl PartialEq for GuessResult {
    fn eq(&self, other: &Self) -> bool {
        &self.0[..] == &other.0[..]
    }
}

impl Eq for GuessResult {}

impl Display for GuessResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str: String = self.0.into_iter().map(|x| {
            match x {
                Wrong => 'B',
                Partial => 'O',
                Correct => 'G',
            }
        }).collect();
        f.write_str(&str)
    }
}

impl Hash for GuessResult {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl FromStr for GuessResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for x in s.chars() {
            if x != 'O' && x != 'G' && x != 'B' {
                return Err(format!("expected O, G or B, got {}", x));
            }
        }
        let v: Vec<GuessBlock> = s.chars().into_iter().map(|x| {
            match x {
                'O' => GuessBlock::Partial,
                'G' => GuessBlock::Correct,
                'B' => GuessBlock::Wrong,
                _ => {
                    panic!("unpossible!");
                }
            }
        }).collect();
        let res: Result<Box<[GuessBlock; 5]>, Box<[GuessBlock]>> = v.into_boxed_slice().try_into();
        match res {
            Ok(s) => Ok(Self(*s)),
            Err(s) => Err(format!("expected 5 characters, got {}", s.len())),
        }
    }
}

impl WordleWord {
    pub fn new(word: [char; 5]) -> WordleWord {
        let mut idx = [false; 26];
        word.into_iter().for_each(|x| {
            idx[(x as u32 - 'a' as u32) as usize] = true
        });
        Self { word, idx }
    }

    pub fn guess(&self, guessed: &Self) -> GuessResult {
        let mut result = [Wrong; 5];
        for ((i, cw), gw) in self.word.iter().enumerate().zip(&mut guessed.word.iter()) {
            result[i] = if cw == gw {
                Correct
            } else if self.idx[(*gw as u32 - 97) as usize] {
                Partial
            } else {
                Wrong
            };
        }
        GuessResult(result)
    }

    pub fn calc_entropy(&self, list: &Vec<&WordleWord>) -> f64 {
        let mut map = HashMap::new();
        list.into_iter().for_each(|x| {
            let count = map.entry(x.guess(self)).or_insert(0 as u32);
            *count += 1;
        });
        let mut res = 0.0;
        map.into_iter().for_each(|p| {
            let p = p.1 as f64 / list.len() as f64;
            let i = -p.log2();
            res += i * p;
        });
        res
    }
}

impl FromStr for WordleWord {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let slice: Result<Box<[char; 5]>, Box<[char]>> = chars.into_boxed_slice().try_into();
        match slice {
            Ok(s) => Ok(Self::new(*s)),
            Err(s) => Err(format!("expected 5 characters, got {}", s.len())),
        }
    }
}

impl Display for WordleWord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str: String = self.word.into_iter().collect();
        f.write_str(&str)
    }
}

pub struct WordleGuesser<'list> {
    pub list: &'list Vec<WordleWord>,
    possible: Vec<&'list WordleWord>,
}

impl<'list> WordleGuesser<'list> {
    pub fn new(list: &'list Vec<WordleWord>) -> Self {
        let possible = list.into_iter().collect();
        Self { list, possible }
    }

    pub fn update(&mut self, word: &WordleWord, result: &GuessResult) {
        self.possible.retain(|x| &x.guess(word) == result)
    }

    pub fn suggest(&self, max_num: u32) -> Vec<(&'list WordleWord, f64)> {
        let mut res = Vec::new();
        for x in self.possible.clone() {
            res.push((x, x.calc_entropy(&self.possible)));
        }
        res.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        while res.len() as u32 > max_num {
            res.pop();
        }
        res
    }
}