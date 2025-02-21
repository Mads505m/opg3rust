// Definition af en generisk stack med en fast maksimal størrelse.
struct Stack<T> {
    elements: Vec<T>,
    max_size: usize,
}

impl<T> Stack<T> {
    // Opretter en ny stack med en specificeret maksimal størrelse.
    pub fn new(max_size: usize) -> Self {
        Stack{
            elements: Vec::with_capacity(max_size),
            max_size,
        }
    }

    // push: Tilføjer et element til toppen af stacken.
    pub fn push(&mut self, element: T) -> Result<(), String> {
        if self.elements.len() >= self.max_size {
            return Err(format!("too many elements: {}", self.elements.len()));

        } else {
            self.elements.push(element);
            Ok(())
        }

    }

    // pop: Fjerner og returnerer det øverste element fra stacken.
    pub fn pop(&mut self) -> Option<T> {
       self.elements.pop()
    }

    // peek: Returnerer en reference til det øverste element uden at fjerne det.
    pub fn peek (&self) -> Option<&T> {
        self.elements.last()
    }

    // is_empty: Returnerer true, hvis stacken er tom.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // print_stack: Udskriver hele stackens indhold fra top til bund.
    pub fn print_stack(&self)
    where
        T: std::fmt::Debug,
    {
        println!("Stackens indhold:");
        for elem in self.elements.iter().rev() {
            println!("{:?}", elem);
        }
    }

}


fn main() {
    // Test af en stack med i32-værdier og en maksimal størrelse på 3
    println!("--- Test af en stack med i32 ---");
    let mut stack: Stack<i32> = Stack::new(3);

    //test om stack er tom -- burde skrive true
    println!("Er stacken tom? {}", stack.is_empty());

    // tilføjer elemenenter med push
    match stack.push(10) {
        Ok(()) => println!("Pushed 10"),
        Err(e) => println!("Fejl: {}", e),
    }
    match stack.push(20) {
        Ok(()) => println!("Pushed 20"),
        Err(e) => println!("Fejl: {}", e),
    }
    match stack.push(30) {
        Ok(()) => println!("Pushed 30"),
        Err(e) => println!("Fejl: {}", e),
    }
    // Forsøg at tilføje et fjerde element, som skal give en fejl.
    match stack.push(40) {
        Ok(()) => println!("Pushed 40"),
        Err(e) => println!("Fejl: {}", e),
    }

    // Udskriv stackens indhold
    stack.print_stack();

    // Kig på det øverste element med peek
    if let Some(top) = stack.peek() {
        println!("Peek: Det øverste element er {}", top);
    }

    // Fjern et element med pop
    if let Some(popped) = stack.pop() {
        println!("Pop: Fjernede element {}", popped);
    }

    // Udskriv stackens indhold efter pop
    stack.print_stack();


    //test om stack er tom -- burde skrive false
    println!("Er stacken tom? {}", stack.is_empty());
}

