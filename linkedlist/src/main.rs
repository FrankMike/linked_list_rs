// Define a Node struct
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// Define a LinkedList struct
#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

// Implement methods for the LinkedList
impl<T: std::fmt::Debug> LinkedList<T> {
    // Create an empty Linked List
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Append a new element at the front to the Linked List
    pub fn push(&mut self, value: T) {
        let new_node = Box::new( Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Remove the element from the front of the Linked List
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    // Check if the Linked List is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Get the length of the Linked List
    pub fn len(&self) -> usize {
        let mut current = &self.head;
        let mut count = 0;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }

    // Display the linked list
    fn display(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{:?} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    
    println!("Length of the Linked List: {}", list.len());

    print!("List: ");
    list.display();

    while let Some(value) = list.pop() {
        println!("Popped value: {}", value);
    }

    println!("Is the Linked List empty? {}", list.is_empty());
}
