use crate::{alloc, print, println, warnln};

// THIS HAS TO BE REWRITTEN TO USE HEAP SOON!!! :C
#[derive(Copy)]
#[derive(Clone)]
pub struct FileVec {
    data: [(u32, i32, (usize, usize, usize), [u8; 20], u8); 100],
    size: usize,
}
impl FileVec {
    pub fn new() -> Self {
        println!("Created new FileSystem Vector");
        Self {
            data: [(0, -1, (0, 0, 0), [0; 20], 0); 100],
            size: 1
        }
    }

    pub fn add(&mut self, file: (u32, i32, (usize, usize, usize), [u8; 20], u8)) {
        self.data[self.size] = file;
        self.size += 1;
    }

    pub fn get(&self, index: usize) -> (u32, i32, (usize, usize, usize), [u8; 20], u8) {
        self.data[index]
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn iter(&self) -> [(u32, i32, (usize, usize, usize), [u8; 20], u8); 100] {
        self.data
    }
}

#[derive(Copy)]
#[derive(Clone)]
#[allow(dead_code)]
pub struct TokenVec {
    size: usize,
    heap_start: usize,
    heap_size: usize,
    heap_end: usize
}
impl TokenVec {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let heap_start = alloc::alloc(512);
        Self {
            size: 0,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    // Add a token to the end of the list
    #[allow(dead_code)]
    pub fn add(&mut self, token: usize, value: usize) {
        if self.size >= self.heap_size {
            warnln!("Reached vec limit :c");
            return;
        }
        alloc::write_byte(self.heap_start + self.size, token);
        alloc::write_byte(self.heap_start + self.size + 8, value);
        self.size += 16;
    }

    // Get a token at a certain index :O
    #[allow(dead_code)]
    pub fn get(&self, address: usize) -> (usize, usize) {
        if address * 16 >= self.size {
            warnln!("Address {} out of range for reading from token vector :c", address);
            return (0, 0);
        }
        (alloc::read_byte(self.heap_start + address * 16),
         alloc::read_byte(self.heap_start + address * 16 + 8))
    }

    // Prints the token vector
    #[allow(dead_code)]
    pub fn print(&self) {
        if self.len() == 0 {
            return;
        }
        print!("[");
        for i in 0..self.len() {
            let data_type = alloc::read_byte(self.heap_start + i * 16);

            if data_type == 6 {
                if i < self.len() - 1 {
                    print!("{}, ", alloc::read_byte(self.heap_start + i * 16 + 8) as u8 as char);
                } else {
                    print!("{}", alloc::read_byte(self.heap_start + i * 16 + 8) as u8 as char);
                }
            } else {
                if i < self.len() - 1 {
                    print!("({} {}) ", data_type, alloc::read_byte(self.heap_start + i * 16 + 8));
                } else {
                    print!("({} {})", data_type, alloc::read_byte(self.heap_start + i * 16 + 8));
                }
            }
        }
        print!("]\n");
    }

    // Set a value in this vector
    #[allow(dead_code)]
    pub fn set(&mut self, address: usize, token: usize, value: usize) {
        if address * 16 >= self.size {
            warnln!("Address out of range for setting in token vector :c");
            return;
        }
        alloc::write_byte(self.heap_start + address * 16, token);
        alloc::write_byte(self.heap_start + address * 16 + 8, value);
    }

    // This is used to remove values, mainly used in the interpreters
    #[allow(dead_code)]
    pub fn shift(&mut self, index: usize, length: usize) {
        for i in index..self.len(){
            if i >= self.len() - length {
                self.set(i, 0, 0);
            } else {
                let next_token = self.get(i + length);
                self.set(i, next_token.0, next_token.1);
            }
        }
        self.size = self.size - length * 16;
    }

    // Get the length of this vector
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.size / 16
    }

    // This will create an entirely new token vector
    #[allow(dead_code)]
    pub fn copy(&self) -> TokenVec {
        let mut new_token_vec = TokenVec::new();
        for value in 0..self.len() {
            let current_value = self.get(value);
            new_token_vec.add(current_value.0, current_value.1);
        }
        new_token_vec
    }

    // Unallocate this vector (please don't forget to, I'm being serious :C)
    #[allow(dead_code)]
    pub fn remove(&self) {
        alloc::unalloc(self.heap_start, self.heap_size);
    }
}

#[derive(Copy)]
#[derive(Clone)]
#[allow(dead_code)]
pub struct BigVec {
    size: usize,
    heap_start: usize,
    heap_size: usize,
    heap_end: usize
}
impl BigVec {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let heap_start = alloc::alloc(262144);
        Self {
            size: 0,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    // Creates an empty version of this Vector, it's quite a wastefull vector if not properly used
    // so use this when you don't need to add or read values
    #[allow(dead_code)]
    pub fn empty() -> Self {
        let heap_start = alloc::alloc(0);
        Self {
            size: 0,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    // Add a value at the end
    #[allow(dead_code)]
    pub fn add(&mut self, value: usize) {
        if self.size >= self.heap_size {
            warnln!("Reached vec limit :c");
            return;
        }
        alloc::write_byte(self.heap_start + self.size, value);
        self.size += 8;
    }

    // Get a value at a certain index
    #[allow(dead_code)]
    pub fn get(&self, address: usize) -> usize {
        if address * 8 > self.size {
            warnln!("Address out of range for {} with bounds {} :c", address * 8, self.heap_end - self.heap_start);
            return 0;
        }
        alloc::read_byte(self.heap_start + address * 8)
    }

    // Please use this with caution, it can crash the system
    // This get will read at some index, no matter if it is owned by this vec, this should strictly be kernel level
    #[allow(dead_code)]
    pub fn get_unsafe(&self, address: usize) -> usize {
        alloc::read_byte(self.heap_start + address * 8)
    }

    // Set at a certain index
    #[allow(dead_code)]
    pub fn set(&mut self, address: usize, value: usize) {
        if address * 8 >= self.size {
            warnln!("Address out of range for {} :c", address);
            return;
        }
        alloc::write_byte(self.heap_start + address * 8, value);
    }

    // Another weird set function, this will pad values that haven't been added yet when you set out of range
    #[allow(dead_code)]
    pub fn set_add(&mut self, address: usize, value: usize) {
        if address * 8 >= self.size {
            self.size = (address + 1) * 8;
        }
        alloc::write_byte(self.heap_start + address * 8, value);
    }

    // Print the current vector
    #[allow(dead_code)]
    pub fn print(&self) {
        if self.len() == 0 {
            return;
        }
        print!("[");
        for i in 0..self.len() {
            if i < self.len() - 1 {
                print!("{} ", alloc::read_byte(self.heap_start + i * 8));
            } else {
                print!("{}", alloc::read_byte(self.heap_start + i * 8));
            }
        }
        print!("]\n");
    }

    // Get the length of this vector
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.size / 8
    }

    // Get the lowest value in the vector
    #[allow(dead_code)]
    pub fn min(&self) -> usize {
        let mut lowest = [0, 999999999];

        for value_index in 0..self.len() {
            let value = self.get(value_index);
            if value < lowest[1] {
                lowest[0] = value_index;
                lowest[1] = value;
            }
        }

        lowest[0]
    }

    // Get the highest value in this vector, no support for negative values
    #[allow(dead_code)]
    pub fn max(&self) -> usize {
        let mut greatest = [0, 0];

        for value_index in 0..self.len() {
            let value = self.get(value_index);
            if value > greatest[1] {
                greatest[0] = value_index;
                greatest[1] = value;
            }
        }

        greatest[0]
    }

    // Unallocate, DONT FORGET TO, I"M BETING SERIOUS FOR THIS ONE!!!11!!11 :D
    #[allow(dead_code)]
    pub fn remove(&self) {
        alloc::unalloc(self.heap_start, self.heap_size);
    }
}

#[derive(Copy)]
#[derive(Clone)]
#[allow(dead_code)]
pub struct Vec {
    size: usize,
    heap_start: usize,
    heap_size: usize,
    heap_end: usize
}
impl Vec {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let heap_start = alloc::alloc(8192);
        Self {
            size: 0,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    // Add at the end of this vector
    #[allow(dead_code)]
    pub fn add(&mut self, value: usize) {
        if self.size >= self.heap_size {
            warnln!("Reached vec limit :c");
            return;
        }
        alloc::write_byte(self.heap_start + self.size, value);
        self.size += 8;
    }

    // Get a value at a certain index
    #[allow(dead_code)]
    pub fn get(&self, address: usize) -> usize {
        if address * 8 > self.size {
            warnln!("Address out of range for {} with actual {} :c", address, address * 8);
            return 0;
        }
        alloc::read_byte(self.heap_start + address * 8)
    }

    // Get a value at a certain index
    #[allow(dead_code)]
    pub fn get_unsafe(&self, address: usize) -> usize {
        alloc::read_byte(self.heap_start + address * 8)
    }

    // Set a value at a certain index
    #[allow(dead_code)]
    pub fn set(&mut self, address: usize, value: usize) {
        if address * 8 >= self.size {
            warnln!("Address out of range for {} :c", address);
            return;
        }
        alloc::write_byte(self.heap_start + address * 8, value);
    }

    // set at an index and padd if doesn't exist yet
    #[allow(dead_code)]
    pub fn set_add(&mut self, address: usize, value: usize) {
        if address * 8 >= self.size {
            self.size = (address + 1) * 8;
        }
        alloc::write_byte(self.heap_start + address * 8, value);
    }

    // print this vector
    #[allow(dead_code)]
    pub fn print(&self) {
        if self.len() == 0 {
            return;
        }
        print!("[");
        for i in 0..self.len() {
            if i < self.len() - 1 {
                print!("{} ", alloc::read_byte(self.heap_start + i * 8));
            } else {
                print!("{}", alloc::read_byte(self.heap_start + i * 8));
            }
        }
        print!("]\n");
    }

    // convert a 64 byte list to a vector
    #[allow(dead_code)]
    pub fn set_as_b64(&mut self, value: [u8; 64]) {
        for i in 0..64 {
            alloc::write_byte(self.heap_start + i * 8, value[i] as usize);
        }
        self.size = 512;
    }

    // get this vector as a 64 byte list to loop over (DEPRICATED AND WILL BE REMOVED SOON)
    #[deprecated(note="please use STRINGS instead!")]
    #[allow(dead_code)]
    pub fn get_as_b64(&self) -> [u8; 64] {
        let mut b64_list = [0; 64];
        for i in 0..self.len() {
            b64_list[i] = alloc::read_byte(self.heap_start + i * 8) as u8;
        }
        b64_list
    }

    // convert a 128 byte list to a vector
    #[allow(dead_code)]
    pub fn set_as_b128(&mut self, value: [u8; 128]) {
        for i in 0..128 {
            alloc::write_byte(self.heap_start + i * 8, value[i] as usize);
        }
        self.size = 512;
    }

    // get this vector as a 128 byte list to loop over (DEPRICATED AND WILL BE REMOVED SOON)
    #[deprecated(note="please use STRINGS instead!")]
    #[allow(dead_code)]
    pub fn get_as_b128(&self) -> [u8; 128] {
        let mut b64_list = [0; 128];
        for i in 0..self.len() {
            b64_list[i] = alloc::read_byte(self.heap_start + i * 8) as u8;
        }
        b64_list
    }

    // convert a 256 byte list to a vector
    #[allow(dead_code)]
    pub fn set_as_b256(&mut self, value: [u8; 256]) {
        for i in 0..256 {
            alloc::write_byte(self.heap_start + i * 8, value[i] as usize);
        }
        self.size = 512;
    }

    // get this vector as a 256 byte list to loop over (DEPRICATED AND WILL BE REMOVED SOON)
    #[deprecated(note="please use STRINGS instead!")]
    #[allow(dead_code)]
    pub fn get_as_b256(&self) -> [u8; 256] {
        let mut b64_list = [0; 256];
        for i in 0..self.len() {
            b64_list[i] = alloc::read_byte(self.heap_start + i * 8) as u8;
        }
        b64_list
    }

    // Get the length of this vector
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.size / 8
    }

    // Get the lowest value
    #[allow(dead_code)]
    pub fn min(&self) -> usize {
        let mut lowest = [0, 999999999];

        for value_index in 0..self.len() {
            let value = self.get(value_index);
            if value < lowest[1] {
                lowest[0] = value_index;
                lowest[1] = value;
            }
        }

        lowest[0]
    }

    // Get the highest value
    #[allow(dead_code)]
    pub fn max(&self) -> usize {
        let mut greatest = [0, 0];

        for value_index in 0..self.len() {
            let value = self.get(value_index);
            if value > greatest[1] {
                greatest[0] = value_index;
                greatest[1] = value;
            }
        }

        greatest[0]
    }

    // Unallocate this vector
    #[allow(dead_code)]
    pub fn remove(&self) {
        alloc::unalloc(self.heap_start, self.heap_size);
    }
}