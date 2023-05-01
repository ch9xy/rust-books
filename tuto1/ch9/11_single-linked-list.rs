#[derive(Debug)]
struct LinkedList<T: std::fmt:: Debug + std::marker:: Copy> {
    head: pointer<T>,
}

#[derive(Debug)]
struct Node<T: std::fmt:: Debug + std::marker:: Copy>  {
    element: T,
    next: pointer<T>,
}

type pointer<T> = Option<Box<Node<T>>>;

impl<T: std::fmt:: Debug + std::marker:: Copy> LinkedList<T> {
    fn create_empty_list() -> LinkedList<T> {
        LinkedList {head: None}
    }

    fn add(&mut self, element: T) {
      /*   match self.head {
            None => self.head = Some(Box::new(Node {
                element: element, next: None
                }))
            Some(previous_head) => {
                let new_head = Some(Box::new(Node{
                    element: element,
                    next: previous_head,
                }))
                self.head = new_head;
            }
*/
  
            let previous_head = self.head.take();
            let new_head = Box::new(Node {
                element: element, next: previous_head
            });
            self.head = Some(new_head);
        }

        fn remove(&mut self) -> Option<T> {
            let previous_head = self.head.take();
            match previous_head {
                Some(old_head) => {
                    self.head = old_head.next;
                    Some(old_head.element)
                }
                None => None
            }
        }
        fn peek(&self) -> Option<T> {
            match &self.head {
                Some(H) => Some(H.element),
                None => None,
            }
        }

        fn printing(&self) {
            let mut list_traversal = &self.head;
            while true {
                match list_traversal {
                    Some(Node) => {
                        println!("{:?}", Node.element);
                        list_traversal = &Node.next
                    },
                    None => break,
                }
            }
        }
    }


fn main () {
    let mut list = LinkedList::create_empty_list();

    list.add(5);
    list.add(7);
    list.add(15);

    println!("The current list is {:?}", list);

    list.remove();
    println!("The current list is {:?}", list);

    println!("The peeked value is {:?}", list.peek());
    list.printing();
}