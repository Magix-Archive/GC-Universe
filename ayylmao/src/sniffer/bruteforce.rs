use std::{num::Wrapping, time::Instant};

use common::utils;
use log::{debug, warn};

use crate::keys::Key;

pub struct Bruteforce {
    seeds: Vec<i64>
}

impl Bruteforce {
    /// Creates a new instance of the `Bruteforce`.
    /// Loads all previous seeds from the file system.
    pub fn new() -> Self {
        let mut seeds = Vec::new();

        // Check if the seeds file exists.
        if utils::file_exists("previous_seeds.txt") {
            let content = utils::read_file("previous_seeds.txt").unwrap();
            for line in content.lines() {
                seeds.push(line.parse().unwrap());
            }
        }

        debug!("Loaded {} previous seeds.", seeds.len());

        Bruteforce {
            seeds
        }
    }

    /// Saves the current seeds to the file system.
    pub fn save(&self) {
        let content = self.seeds.iter()
            .map(|seed| seed.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        utils::write_file("previous_seeds.txt", content).unwrap();
    }

    /// Checks if the seed is valid.
    /// Sourced from: https://github.com/TheLostTree/evergreen/blob/master/evergreen/src/key_bruteforce.rs
    /// seed: The seed to check.
    /// server_seed: The server-provided seed. Used in the seed calculation.
    /// depth: The amount of times to try checking the seed.
    /// test: The test data to check against.
    pub fn guess(
        &self,
        seed: i64,
        server_seed: u64,
        depth: i32,
        test: &[u8]
    ) -> Option<u64> {
        // Calculate the known prefix and suffix of the key.
        let prefix = [test[0] ^ 0x45, test[1] ^ 0x67];
        let suffix = [
            test[test.len() - 2] ^ 0x89,
            test[test.len() - 1] ^ 0xAB,
        ];

        // Attempt to generate the key.
        let mut generator = Random::seeded(seed as i32);
        for _ in 0..depth {
            let client_seed = generator.next_safe_uint64();
            
            let seed = client_seed ^ server_seed;
            let key = Key::new(seed);

            if key.compare((prefix, suffix), test) {
                debug!("Found encryption key seed: {seed}");
                return Some(seed);
            }
        }

        None
    }

    /// Run the bruteforce loop.
    /// sent_time: The time the packet was sent.
    /// server_seed: The server-provided seed.
    /// test: The test data to check against.
    pub fn run(
        &mut self,
        sent_time: u64,
        server_seed: u64,
        test: &[u8]
    ) -> Option<u64> {
        // Try old seeds.
        for seed in &self.seeds {
            if let Some(seed) = self.guess(*seed, server_seed, 1000, test) {
                return Some(seed);
            }
        }
        
        // Generate new seeds.
        for i in 0..3000i64 {
            let offset = if i % 2 == 0 { i / 2 } else { -(i - 1) / 2 };
            let time = sent_time as i64 + offset; // This will act as the seed.

            if let Some(key) = self.guess(time, server_seed, 1000, test) {
                self.seeds.push(time);
                return Some(key);
            }
        }

        warn!("Unable to find the encryption key seed.");
        None
    }
}

pub struct Random {
    m_big: i32,
    m_seed: i32,
    inext: i32,
    inextp: i32,
    seed_array: [i32; 56],
}

/// "it's almost a direct translation of the c# source 😭" -thelosttree
/// Sourced from: https://github.com/TheLostTree/evergreen/blob/master/evergreen/src/key_bruteforce.rs
impl Random {
    /// Creates a new instance of the `Random`.
    /// This uses a default seed.
    pub fn default() -> Self {
        Random {
            m_big: i32::MAX,
            m_seed: 161803398,
            inext: 0,
            inextp: 0,
            seed_array: [0; 56],
        }
    }

    /// Creates a new instance of the `Random`.
    pub fn new() -> Self {
        Random::seeded(Instant::now().elapsed().as_millis() as i32)
    }

    /// Creates a new instance of the `Random`.
    /// seed: The seed to use for the generator.
    pub fn seeded(seed: i32) -> Random {
        let mut ii;
        let mut rand = Random::default();

        let subtraction = if seed == i32::MIN {
            i32::MAX
        } else {
            i32::abs(seed)
        };
        let mut mj = rand.m_seed - subtraction;
        rand.seed_array[55] = mj;

        let mut mk = 1;

        for i in 1..55 {
            ii = 21 * i % 55;
            rand.seed_array[ii] = mk;
            mk = mj - mk;
            if mk < 0 {
                mk += rand.m_big
            }
            mj = rand.seed_array[ii]
        }

        for _ in 1..5 {
            for i in 1..56 {
                rand.seed_array[i] =
                    rand.seed_array[i].wrapping_sub(rand.seed_array[1 + (i + 30) % 55]);
                if rand.seed_array[i] < 0 {
                    rand.seed_array[i] += rand.m_big
                };
            }
        }

        rand.inext = 0;
        rand.inextp = 21;

        rand
    }

    pub fn next_double(&mut self) -> f64 {
        (self.internal_sample() as f64) * (1.0 / (self.m_big as f64))
    }

    fn internal_sample(&mut self) -> i32 {
        let mut ret_val: i32;
        let mut loc_inext = self.inext;
        let mut loc_inextp = self.inextp;

        if (loc_inext += 1, loc_inext).1 >= 56 {
            loc_inext = 1;
        }
        if (loc_inextp += 1, loc_inextp).1 >= 56 {
            loc_inextp = 1;
        }

        ret_val = self.seed_array[loc_inext as usize] - self.seed_array[loc_inextp as usize];
        if ret_val == self.m_big {
            ret_val -= 1
        };
        if ret_val < 0 {
            ret_val += self.m_big
        };

        self.inext = loc_inext;
        self.inextp = loc_inextp;

        ret_val
    }

    pub fn next_safe_uint64(&mut self) -> u64 {
        (self.next_double() * (u64::MAX as f64)) as u64
    }
}

pub struct MT19937_64 {
    mt: [u64; 312],
    mti: u32,
}

/// Sourced from: https://github.com/TheLostTree/evergreen/blob/master/evergreen/src/mtkey.rs
impl MT19937_64 {
    pub fn default() -> MT19937_64 {
        MT19937_64 {
            // these are used in c# for some reason but not here?
            // N: 0x138, // 312
            // M: 0x9C, // 156
            // matrix_a: 0xB5026F5AA96619E9, //13043109905998158313
            mt: [0; 312],
            mti: 0x138,
        }
    }

    pub fn seed(&mut self, seed: u64) {
        self.mt[0] = seed & 0xffffffffffffffff;
        for i in 1..312 {
            let value = Wrapping(self.mt[i - 1] ^ (self.mt[i - 1] >> 62));

            self.mt[i] =
                ((Wrapping(6364136223846793005u64) * value).0 + (i as u64)) & 0xffffffffffffffff;
        }
        self.mti = 312;
    }

    pub fn next_ulong(&mut self) -> u64 {
        if self.mti >= 312 {
            if self.mti == 313 {
                self.seed(5489)
            }
            for k in 0..311 {
                let y = (self.mt[k] & 0xffffffff80000000) | (self.mt[k + 1] & 0x7fffffff);
                if k < (312 - 156) {
                    self.mt[k] = self.mt[k + 156]
                        ^ (y >> 1)
                        ^ (if (y & 1) == 0 { 0 } else { 0xb5026f5aa96619e9 });
                } else {
                    self.mt[k] = self.mt[(Wrapping(k + 156 + self.mt.len()) - Wrapping(624)).0]
                        ^ (y >> 1)
                        ^ (if (y & 1) == 0 { 0 } else { 0xb5026f5aa96619e9 });
                }
            }

            let yy = (self.mt[311] & 0xffffffff80000000) | (self.mt[0] & 0x7fffffff);
            self.mt[311] =
                self.mt[155] ^ (yy >> 1) ^ (if yy & 1 == 0 { 0 } else { 0xb5026f5aa96619e9 });
            self.mti = 0;
        }
        let mut x = self.mt[self.mti as usize];
        self.mti += 1;
        x ^= (x >> 29) & 0x5555555555555555;
        x ^= (x << 17) & 0x71d67fffeda60000;
        x ^= (x << 37) & 0xfff7eee000000000;
        x ^= x >> 43;
        x
    }
}
