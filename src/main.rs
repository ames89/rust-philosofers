use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Estructura para representar a un filósofo
struct Philosopher {
    id: usize, // Identificador del filósofo
    name: String,
    // Referencias atómicamente contadas a los Mutex de los palillos
    left_chopstick: Arc<Mutex<()>>,
    right_chopstick: Arc<Mutex<()>>,
}

impl Philosopher {
    // Método que simula la vida del filósofo: pensar y comer
    fn dine(&self) {
        for i in 0..3 { // Que coman unas pocas veces para el ejemplo
            self.think();
            self.eat(i + 1);
        }
        println!("{} terminó de comer y se fue a meditar.", self.name);
    }

    fn think(&self) {
        println!("{} está pensando.", self.name);
        // Simula el tiempo de pensar con una pausa aleatoria
        let think_time = Duration::from_millis(rand::random::<u64>() % 3000 + 100);
        thread::sleep(think_time);
    }

    fn eat(&self, round: usize) {
        println!("{} tiene hambre e intenta tomar los palillos (ronda {}).", self.name, round);

        // *** Estrategia Anti-Deadlock: Ordenación de Bloqueo ***
        // Para evitar el deadlock, todos los filósofos intentan bloquear
        // primero el palillo con el índice (ID) más bajo.
        // Obtenemos los IDs "imaginarios" de los Mutex (basados en su dirección de memoria,
        // aunque aquí usaremos el ID del filósofo como proxy para simplificar).
        // Una forma más robusta sería asignar IDs únicos a los palillos.
        // Aquí, simplemente comparamos los IDs de los filósofos adyacentes
        // para decidir el orden. O, más simple, usamos los índices directos
        // de los palillos en el vector original (0 a 4).

        // Identificadores de los palillos (asumimos que el palillo 'i' está a la izquierda del filósofo 'i')
        let left_chopstick_id = self.id;
        let right_chopstick_id = (self.id + 1) % 5; // 5 es el número de filósofos/palillos

        // Bloquea primero el palillo con el ID menor
        let (_first_lock, _second_lock) = if left_chopstick_id < right_chopstick_id {
             // Bloquea izquierda, luego derecha
             println!("{} intenta tomar palillo izquierdo ({})...", self.name, left_chopstick_id);
             let _guard1 = self.left_chopstick.lock().unwrap();
             println!("{} TOMÓ palillo izquierdo ({}). Ahora intenta derecho ({})...", self.name, left_chopstick_id, right_chopstick_id);
             let _guard2 = self.right_chopstick.lock().unwrap();
             println!("{} TOMÓ palillo derecho ({}).", self.name, right_chopstick_id);
             (_guard1, _guard2) // Devolvemos las guardas para mantener el bloqueo
        } else {
            // Bloquea derecha, luego izquierda (para el filósofo 4, que necesita palillo 4 y 0)
            println!("{} intenta tomar palillo derecho ({})...", self.name, right_chopstick_id);
            let _guard2 = self.right_chopstick.lock().unwrap();
            println!("{} TOMÓ palillo derecho ({}). Ahora intenta izquierdo ({})...", self.name, right_chopstick_id, left_chopstick_id);
            let _guard1 = self.left_chopstick.lock().unwrap();
            println!("{} TOMÓ palillo izquierdo ({}).", self.name, left_chopstick_id);
            (_guard1, _guard2) // Devolvemos las guardas para mantener el bloqueo
        };

        // Si llegamos aquí, el filósofo ha conseguido ambos palillos
        println!("*** {} está comiendo (ronda {})! ***", self.name, round);
        // Simula el tiempo de comer
        let eat_time = Duration::from_millis(rand::random::<u64>() % 1000 + 100);
        thread::sleep(eat_time);

        println!("{} terminó de comer (ronda {}) y suelta los palillos.", self.name, round);
        // Los MutexGuard (_guard1, _guard2) salen del alcance aquí,
        // liberando automáticamente los bloqueos (los palillos).
    }
}

fn main() {
    println!("Problema de los Filósofos Comensales - Inicio");

    // Crear los 5 palillos como Mutex dentro de Arcs
    let chopsticks: Vec<_> = (0..5).map(|_| Arc::new(Mutex::new(()))).collect();

    // Crear los 5 filósofos
    let philosophers: Vec<Philosopher> = (0..5).map(|i| {
        let name = format!("Filósofo {}", i + 1);
        // Clonar los Arcs para que cada filósofo tenga su propia referencia contada
        let left_chopstick = Arc::clone(&chopsticks[i]);
        // El palillo derecho es el siguiente en el círculo (con módulo)
        let right_chopstick = Arc::clone(&chopsticks[(i + 1) % 5]);

        Philosopher {
            id: i, // Asignamos un ID simple basado en el índice
            name,
            left_chopstick,
            right_chopstick,
        }
    }).collect();

    // Vector para guardar los handles de los hilos
    let mut handles = vec![];

    // Lanzar un hilo por cada filósofo
    for philosopher in philosophers {
        // Crear un Arc del filósofo para moverlo al hilo de forma segura
        let philosopher_arc = Arc::new(philosopher);
        let handle = thread::spawn(move || {
            // El hilo ejecuta el método `dine` del filósofo
            philosopher_arc.dine();
        });
        handles.push(handle);
    }

    // Esperar a que todos los hilos de los filósofos terminen
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Problema de los Filósofos Comensales - Fin");
}