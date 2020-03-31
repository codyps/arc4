use std::num::Wrapping;

pub struct Context<'a> {
    key: &'a [u8],
    state: [u8; 256], 

    i: u8,
    j: u8,
}

impl<'a> Context<'a> {
    pub fn with_key(key: &'a [u8]) -> Self {
        let mut s = Context {
            key: key,
            state: [0;256],
            i: 0,
            j: 0,
        };

        ksa(&mut s.state, s.key);
        s
    }

    pub fn prga(&mut self, out: &mut [u8]) {
        let mut i = Wrapping(self.i);
        let mut j = Wrapping(self.j);
        for z in 0 .. self.key.len() {
            i = i + Wrapping(1);
            j = j + Wrapping(self.state[i.0 as usize]);
            self.state.swap(i.0 as usize, j.0 as usize);
            out[z] = self.state[(Wrapping(self.state[i.0 as usize]) + Wrapping(self.state[j.0 as usize])).0 as usize];
        }

        self.i = i.0;
        self.j = j.0;
    }

    pub fn encrypt(&mut self, data: &mut [u8]) {
        let mut i = Wrapping(self.i);
        let mut j = Wrapping(self.j);
        for z in 0 .. self.key.len() {
            i = i + Wrapping(1);
            j = j + Wrapping(self.state[i.0 as usize]);
            self.state.swap(i.0 as usize, j.0 as usize);
            data[z] ^= self.state[(Wrapping(self.state[i.0 as usize]) + Wrapping(self.state[j.0 as usize])).0 as usize];
        }

        self.i = i.0;
        self.j = j.0;
    }
}

fn ksa(s: &mut [u8], key: &[u8]) {
    for i in 0 ..= 255 {
        s[i] = i as u8;
    }

    let mut j = 0;
    for i in 0 ..= 255 {
        j = (j + s[i] + key[i % key.len()]) as u8;
        s.swap(i, j as usize);
    }
}
