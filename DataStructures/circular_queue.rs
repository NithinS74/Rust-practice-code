const MAX: usize = 10;
#[derive(Debug)]
struct queue {
    que: [i32; MAX],
    front: Option<usize>,
    back: Option<usize>,
}

enum QueueError {
    QueueFull,
    QueueEmpty,
}

pub fn circular_queue() {
    let mut q = queue::new();
    q.enqueue(6);
    q.enqueue(7);
    q.dequeue();
    q.display();
}

impl queue {
    fn new() -> Self {
        Self {
            que: [0; MAX],
            front: None,
            back: None,
        }
    }

    fn display_full(&self) {
        println!("{:?}", self);
    }

    fn display(&self) {
        if let Some(back) = self.back {
            println!("{:?}", &self.que[..=back]);
        } else {
            println!("Empty que");
        }
    }

    fn enqueue(&mut self, value: i32) -> Result<Option<usize>, QueueError> {
        if self.is_full() {
            return Err(QueueError::QueueFull);
        }

        if let Some(back) = self.back.as_mut() {
            *back = (*back + 1) % MAX;
            self.que[*back] = value;
            return Ok(Some(*back));
        } else {
            self.back = Some(0);
            self.que[0] = value;
            return Ok(Some(0));
        }
    }

    fn dequeue(&mut self) -> Result<Option<usize>, QueueError> {
        if self.is_empty() {
            return Err(QueueError::QueueEmpty);
        }
        if let Some(front) = self.front.as_mut() {
            *front = (*front + 1) % MAX;
            return Ok(Some(*front));
        } else {
            self.front = Some(0);
            return Ok(Some(0));
        }
    }

    fn is_empty(&mut self) -> bool {
        if let Some(back) = self.back {
            if let Some(front) = self.front {
                if front > back {
                    self.back = None;
                    self.front = None;
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return true;
        }
    }

    fn is_full(&self) -> bool {
        if let Some(back) = self.back {
            if let Some(front) = self.front {
                if ((back + 1) % MAX) == front {
                    return true;
                } else {
                    return false;
                }
            } else {
                if back + 1 == MAX {
                    return true;
                } else {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
}
