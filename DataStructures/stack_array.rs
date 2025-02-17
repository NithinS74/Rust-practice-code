struct Stack {
    stak: [i32; 10],
    top: Option<usize>,
}

enum StackError {
    StackFull,
    StackEmpty,
}

impl Stack {
    fn new() -> Self {
        Self {
            stak: [0; 10],
            top: None,
        }
    }

    fn is_full(&self) -> bool {
        if let Some(top) = self.top {
            if top == 9 {
                return true;
            }
        }
        return false;
    }
    fn is_empty(&self) -> bool {
        if let Some(_) = self.top {
            return false;
        } else {
            return true;
        }
    }

    fn push(&mut self, value: i32) -> Result<Option<usize>, StackError> {
        if self.is_full() {
            return Err(StackError::StackFull);
        }
        if let Some(top) = self.top.as_mut() {
            *top += 1;
            self.stak[*top] = value;
            return Ok(Some(*top));
        } else {
            self.top = Some(0);
            self.stak[0] = value;
            return Ok(Some(0));
        }
    }

    fn pop(&mut self) -> Result<Option<usize>, StackError> {
        if self.is_empty() {
            return Err(StackError::StackEmpty);
        }
        if let Some(top) = self.top.as_mut() {
            if *top == 0 {
                self.top = None;
                return Ok(None);
            } else {
                *top -= 1;
                return Ok(Some(*top));
            }
        } else {
            return Err(StackError::StackEmpty);
        }
    }
    fn display(&self) {
        if let Some(top) = self.top {
            println!("{:?}", &self.stak[..=top]);
        } else {
            println!("Stack is Emplty");
        }
    }

    fn track_results(r: Result<Option<usize>, StackError>) {
        match r {
            Ok(top) => match top {
                Some(top) => println!("pushed to stack at index {top}"),
                None => println!("Stack has been emptyed"),
            },
            Err(StackError::StackFull) => println!("Stack is full"),
            Err(StackError::StackEmpty) => println!("Stack is empty"),
        }
    }
}

pub fn stack_array() {
    let mut s = Stack::new();
    Stack::track_results(s.push(5));
    Stack::track_results(s.push(7));
    Stack::track_results(s.push(9));
    s.display();
    Stack::track_results(s.pop());
    s.display();
}
