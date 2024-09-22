use std::collections::HashMap;

use bit_vec::BitVec;

pub struct ArithmeticCoding {
    max: u16,
    min: u16,
    msd: u16,
    ssd: u16,
    scale: u16,
    underflow_bits: u16,
    source: String,
    probabilities: HashMap<char, (u16, u16)>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Frequency {
    val: u16,
    key: char,
}

impl Frequency {
    fn new(key: char) -> Frequency {
        Frequency { key, val: 0 }
    }
}

impl ArithmeticCoding {
    pub fn new(source: String) -> ArithmeticCoding {
        let mut ans = ArithmeticCoding {
            max: 0xffff,
            min: 0,
            msd: 0x8000,
            ssd: 0x4000,
            scale: 0,
            underflow_bits: 0,
            source,
            probabilities: HashMap::new(),
        };

        ans.calculate_probabilities();

        ans
    }

    fn calculate_probabilities(&mut self) {
        let mut frequencies: HashMap<char, Frequency> = HashMap::new();

        for c in self.source.chars() {
            if !frequencies.contains_key(&c) {
                frequencies.insert(c.clone(), Frequency::new(c.clone()));
            }

            if let Some(v) = frequencies.get_mut(&c) {
                v.val += 1;
            }
        }

        let mut frequencies: Vec<Frequency> = frequencies.into_values().collect();
        frequencies.sort();

        self.scale = self.source.len() as u16;

        let mut low: u16 = 0;

        for symbol in frequencies {
            let high: u16 = low + symbol.val;
            self.probabilities.insert(symbol.key, (low, high));
            low = high
        }
    }

    pub fn compress(&mut self, input: String) -> BitVec {
        let mut low: u16 = self.min;
        let mut high: u16 = self.max;

        let mut output_stream = BitVec::new();

        self.underflow_bits = 0;

        let mut output = String::new();

        for symbol in input.chars() {
            let range = (high - low) as u64 + 1;

            if let Some(v) = self.probabilities.get_mut(&symbol) {
                //want any more as u64?
                high = (low as u64 + range * v.1 as u64 / self.scale as u64 - 1) as u16;
                low = (low as u64 + range * v.0 as u64 / self.scale as u64) as u16;

                loop {
                    if (high & self.msd) == (low & self.msd) {
                        if high & self.msd != 0 {
                            output_stream.push(true);
                            output.push('1');
                        } else {
                            output_stream.push(false);
                            output.push('0');
                        }

                        while self.underflow_bits > 0 {
                            output_stream.push((high & self.msd) == 0);
                            if (high & self.msd) == 0 {
                                output.push('1');
                            } else {
                                output.push('0');
                            }

                            self.underflow_bits -= 1;
                        }
                    } else {
                        if (low & self.ssd) != 0 && (high & self.ssd) == 0 {
                            self.underflow_bits += 1;

                            low &= 0x3fff;
                            high |= 0x4000;
                        } else {
                            break;
                        }
                    }

                    low <<= 1;
                    high <<= 1;
                    high |= 1;
                }
            }
        }

        if (low & 0x4000) != 0 {
            output_stream.push(true);
            output.push('1');
        } else {
            output_stream.push(false);
            output.push('0');
        }

        self.underflow_bits += 1;

        while self.underflow_bits > 0 {
            if (low & 0x4000) == 0 {
                output_stream.push(true);
                output.push('1');
            } else {
                output_stream.push(false);
                output.push('0');
            }

            self.underflow_bits -= 1;
        }

        output_stream
    }
}
