use stick::Stick;

pub struct Philosofer {
    name: String,
    pub left: Stick,
    pub right: Stick,
}

// Define methods within an impl block
impl Philosofer {
    // This is an associated function (often used as a constructor)
    pub fn new(name: String, left: Stick, right: Stick) -> Philosofer {
        Philosofer { name, left, right }
    }

    // retorna el nombre del filosofo como una referencia de String
    pub fn get_name(&self) -> &String {
        &self.name
    }

    // comer
    pub fn eat(&mut self) {
        // Logic for the philosopher eating
        println!("{} está comiendo.", self.name);
        // In a real scenario, you would need logic to acquire the sticks (locks)
    }

    // pensar,
    pub fn think(&mut self) {
        // Logic for the philosopher thinking
        println!("{} está pensando.", self.name);
        // In a real scenario, you would need logic to release the sticks (locks)
    }

    // intentar comer
    pub fn try_to_eat(&mut self) {
        self.left.status.lock().unwrap();
        self.right.status.lock().unwrap();
        self.eat();
        self.left.status.unlock();
        self.right.status.unlock();
        self.think();
    }


    // soltar palillos
}
