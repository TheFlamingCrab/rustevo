use std::vec::Vec;
use core::clone::Clone;

pub struct SlotMap<T> {
    slots: Vec<Slot<T>>,
    free: Vec<usize>,
    init_size: usize,
}

#[derive(Copy)]
#[derive(Clone)]
struct Slot<T> {
    value: Option<T>,
    count: usize,
}

impl<T> SlotMap<T> {
    pub fn new(size: usize) -> Self {
        let free = (0..size).collect();
        let mut slots = Vec::with_capacity(size);

        for i in 0..size {
            slots.push(Slot {value: None, count: 0});
        }

        Self {
            slots,
            free,
            init_size: size,
        }
    }

    pub fn remove(&mut self, key: SlotKey) {
        if self.slots[key.index].count != key.count {
            return
        }

        if let Some(s) = &self.slots[key.index].value {
            self.slots[key.index].value = None;
            self.free.push(key.index);
        }
    }

    pub fn get(&mut self, key: &SlotKey) -> Option<&mut T> {
        if key.count != self.slots[key.index].count {
            return None
        }

        self.slots[key.index].value.as_mut()
    }

    pub fn insert(&mut self, data: T) -> SlotKey {
        let slot_index = match self.free.pop() {
            Some(index) => index,
            None => {
                println!("EXPANDING SLOTMAP");
                self.slots.reserve(self.init_size / 2);
                self.free.reserve(self.init_size / 2);

                for i in 0..(self.init_size / 2) {
                    self.free.push(i);
                }

                self.free.pop().unwrap()
            }
        };

        self.slots[slot_index].value = Some(data);
        self.slots[slot_index].count += 1;

        SlotKey {
            index: slot_index,
            count: self.slots[slot_index].count,
        }
    }
}

pub struct SlotKey {
    index: usize,
    count: usize,
}