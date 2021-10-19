pub fn run() {
    test_int_queue();
    test_char_queue();
    test_str_queue();
}

fn test_char_queue() {
    let mut char_queue = QueueChar::new();
    println!(
        "\nEmpty {}, length {}",
        char_queue.is_empty(),
        char_queue.length()
    );

    let full_name = "Gonçalo Candeias Amaro";
    let mut names = full_name.split_whitespace();

    char_queue.enqueue(String::from(names.next().unwrap()));
    char_queue.enqueue(String::from(names.next().unwrap()));
    char_queue.enqueue(String::from(names.next().unwrap()));
    println!(
        "\nEmpty {}, length {}, first {:?}",
        char_queue.is_empty(),
        char_queue.length(),
        char_queue.peek()
    );

    char_queue.dequeue();
    println!(
        "Empty {}, length {}, first {:?}",
        char_queue.is_empty(),
        char_queue.length(),
        char_queue.peek()
    );
}

struct QueueChar<char> {
    queue: Vec<char>,
}

impl QueueChar<char> {
    fn new() -> Self {
        QueueChar { queue: Vec::new() }
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn enqueue(&mut self, item: String) {
        for i in item.chars() {
            self.enqueue_char(i)
        }
    }

    fn enqueue_char(&mut self, item: char) {
        if self.length() >= 9 {
            // Gonçalo+Candeias+Amaro=20
            self.dequeue();
        }
        self.queue.push(item);
    }

    fn dequeue(&mut self) -> char {
        self.queue.remove(0)
    }
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn peek(&self) -> Option<&char> {
        self.queue.first()
    }
}

fn test_int_queue() {
    let mut int_queue = QueueI32::new();
    println!(
        "\nEmpty {}, length {}",
        int_queue.is_empty(),
        int_queue.length()
    );

    int_queue.enqueue(23);
    int_queue.enqueue(32);
    int_queue.enqueue(232);
    println!(
        "\nEmpty {}, length {}, first {:?}",
        int_queue.is_empty(),
        int_queue.length(),
        int_queue.peek()
    );

    int_queue.dequeue();
    println!(
        "Empty {}, length {}, first {:?}",
        int_queue.is_empty(),
        int_queue.length(),
        int_queue.peek()
    );
}

struct QueueI32<i32> {
    queue: Vec<i32>,
}

impl QueueI32<i32> {
    fn new() -> Self {
        QueueI32 { queue: Vec::new() }
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn enqueue(&mut self, item: i32) {
        self.queue.push(item)
    }

    fn dequeue(&mut self) -> i32 {
        self.queue.remove(0)
    }
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn peek(&self) -> Option<&i32> {
        self.queue.first()
    }
}

fn test_str_queue() {
    let mut str_queue = QueueStr::new();
    println!(
        "Empty {}, length {}",
        str_queue.is_empty(),
        str_queue.length()
    );

    let full_name = "Gonçalo Candeias Amaro";
    let mut names = full_name.split_whitespace();

    str_queue.enqueue(String::from(names.next().unwrap()));
    str_queue.enqueue(String::from(names.next().unwrap()));
    str_queue.enqueue(String::from(names.next().unwrap()));
    println!(
        "Empty {}, length {}, first {:?}",
        str_queue.is_empty(),
        str_queue.length(),
        str_queue.peek()
    );

    str_queue.dequeue();
    println!(
        "Empty {}, length {}, first {:?}",
        str_queue.is_empty(),
        str_queue.length(),
        str_queue.peek()
    );
}
struct QueueStr<String> {
    queue: Vec<String>,
}

impl QueueStr<String> {
    fn new() -> Self {
        QueueStr { queue: Vec::new() }
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn enqueue(&mut self, item: String) {
        self.queue.push(item)
    }

    fn dequeue(&mut self) -> String {
        self.queue.remove(0)
    }
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn peek(&self) -> Option<&String> {
        self.queue.first()
    }
}
