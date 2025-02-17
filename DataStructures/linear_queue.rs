#[derive(Debug)]
struct Queue {
    que: [i32; 10],
    front: Option<usize>,
    back: Option<usize>,
}

impl Queue {
    fn new() -> Self {
        Self {
            que: [0; 10],
            front: None,
            back: None,
        }
    }
    fn is_full(&self) -> bool {
        if let Some(back_index) = self.back {
            if back_index == self.que.len() {
                return true;
            }
        }
        return false;
    }
    fn is_empty(&mut self) -> bool {
        if let Some(back_index) = self.back {
            if let Some(front_index) = self.front {
                if front_index > back_index {
                    self.front = Option::None;
                    self.back = Option::None;
                    return true;
                }
            }
        } else {
            return true;
        }

        return false;
    }

    fn display_que(&self) {
        println!("{:?}\n", self);
    }

    fn enque(&mut self, value: i32) -> Option<usize> {
        if self.is_full() {
            return None;
        }
        if let Some(b_index) = self.back.as_mut() {
            *b_index += 1;
            self.que[*b_index] = value;
            println!("enqued value {value} at index {}", b_index);
            return Some(*b_index);
        } else {
            self.back = Some(0);
            self.front = Some(0);
            println!("enqued value {value} at index {}", 0);
            self.que[0] = value;
            return Some(0);
        }
    }

    fn display(&self) {
        if let Some((front, back)) = self.front.zip(self.back) {
            println!("{:?}", &self.que[front..=back]);
        } else if let Some(back) = self.back {
            println!("{:?}", &self.que[..back]);
        } else {
            println!("Que is empty");
        }
    }

    fn deque(&mut self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        if let Some(f_index) = self.front.as_mut() {
            println!("Dequed {} from index {}", self.que[*f_index], *f_index);
            *f_index += 1;
            return Some(*f_index);
        }
        return None;
    }
}

pub fn queue() {
    let mut q = Queue::new();
    q.enque(5);
    q.enque(6);
    q.enque(7);
    q.display_que();
    q.deque();
    q.display();
}
