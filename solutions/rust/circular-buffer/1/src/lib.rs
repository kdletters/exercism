pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(None);
        }
        Self {
            buffer,
            head: 0,
            tail: 0,
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }

        self.buffer[self.tail] = Some(_element);
        self.tail = (self.tail + 1) % self.buffer.len();
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }

        let element = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.buffer.len();
        element.ok_or(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
        for item in self.buffer.iter_mut() {
            *item = None;
        }
        self.head = 0;
        self.tail = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.is_full() {
            self.head = (self.head + 1) % self.buffer.len();
        }
        self.buffer[self.tail] = Some(_element);
        self.tail = (self.tail + 1) % self.buffer.len();
    }

    fn is_full(&self) -> bool {
        self.tail == self.head && self.buffer[self.tail].is_some()
    }

    fn is_empty(&self) -> bool {
        self.tail == self.head && self.buffer[self.tail].is_none()
    }
}
