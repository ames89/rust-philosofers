package main

import (
	"fmt"
	"math/rand"
	"sync"
	"time"
)

const (
	numPhilosophers = 5
	maxEatCycles    = 3 // Cuántas veces come cada filósofo antes de terminar
)

// Philosopher representa a un filósofo con sus palillos
type Philosopher struct {
	id             int
	name           string
	leftChopstick  *sync.Mutex // Puntero al Mutex del palillo izquierdo
	rightChopstick *sync.Mutex // Puntero al Mutex del palillo derecho
	timesEaten     int
}

// dine simula el ciclo completo de vida (pensar/comer) del filósofo
func (p *Philosopher) dine(wg *sync.WaitGroup) {
	defer wg.Done() // Asegura que llamemos a Done() cuando el filósofo termine

	for p.timesEaten < maxEatCycles {
		p.think()
		p.eat()
	}
	fmt.Printf("%s terminó de comer y se fue a meditar.\n", p.name)
}

// think simula la acción de pensar
func (p *Philosopher) think() {
	fmt.Printf("%s está pensando.\n", p.name)
	// Pausa aleatoria para simular el pensamiento
	thinkTime := time.Duration(rand.Intn(1500)+100) * time.Millisecond
	time.Sleep(thinkTime)
}

// eat simula la acción de intentar comer, adquiriendo los palillos
func (p *Philosopher) eat() {
	fmt.Printf("%s tiene hambre e intenta tomar los palillos (vez %d).\n", p.name, p.timesEaten+1)

	// *** Estrategia Anti-Deadlock: Ordenación de Bloqueo ***
	// Bloquea primero el palillo con el índice menor para evitar deadlock.
	// Asumimos que el palillo 'i' está a la izquierda del filósofo 'i'.
	leftChopstickID := p.id
	rightChopstickID := (p.id + 1) % numPhilosophers

	var firstChopstick, secondChopstick *sync.Mutex
	var firstID, secondID int

	if leftChopstickID < rightChopstickID {
		firstChopstick = p.leftChopstick
		firstID = leftChopstickID
		secondChopstick = p.rightChopstick
		secondID = rightChopstickID
		// fmt.Printf("%s (debug): Orden -> Izquierda (%d) luego Derecha (%d)\n", p.name, firstID, secondID) // Debug
	} else {
		firstChopstick = p.rightChopstick
		firstID = rightChopstickID
		secondChopstick = p.leftChopstick
		secondID = leftChopstickID
		// fmt.Printf("%s (debug): Orden -> Derecha (%d) luego Izquierda (%d)\n", p.name, firstID, secondID) // Debug
	}

	// Intenta tomar el primer palillo (el de menor índice)
	fmt.Printf("%s intenta tomar palillo %d...\n", p.name, firstID)
	firstChopstick.Lock()
	fmt.Printf("%s TOMÓ palillo %d. Ahora intenta %d...\n", p.name, firstID, secondID)
	// ¡Importante! Usar defer para asegurar que se libere el lock aunque algo falle
	defer firstChopstick.Unlock()

	// Intenta tomar el segundo palillo (el de mayor índice)
	secondChopstick.Lock()
	fmt.Printf("%s TOMÓ palillo %d.\n", p.name, secondID)
	// Defer para liberar el segundo lock
	defer secondChopstick.Unlock()

	// Si llegamos aquí, tiene ambos palillos
	fmt.Printf("*** %s está comiendo (vez %d)! ***\n", p.name, p.timesEaten+1)
	// Simula el tiempo de comer
	eatTime := time.Duration(rand.Intn(1000)+100) * time.Millisecond
	time.Sleep(eatTime)

	p.timesEaten++
	fmt.Printf("%s terminó de comer (vez %d) y suelta los palillos.\n", p.name, p.timesEaten)
	// Los 'defer' se ejecutarán aquí en orden inverso, liberando los locks.
}

func main() {
	fmt.Println("Problema de los Filósofos Comensales - Inicio (Go)")

	// Inicializar el generador de números aleatorios
	rand.Seed(time.Now().UnixNano())

	// Crear los palillos (Mutex)
	chopsticks := make([]*sync.Mutex, numPhilosophers)
	for i := 0; i < numPhilosophers; i++ {
		chopsticks[i] = new(sync.Mutex) // Crea un nuevo Mutex
	}

	// Crear los filósofos
	philosophers := make([]*Philosopher, numPhilosophers)
	for i := 0; i < numPhilosophers; i++ {
		philosophers[i] = &Philosopher{
			id:             i,
			name:           fmt.Sprintf("Filósofo %d", i+1),
			leftChopstick:  chopsticks[i],                     // Palillo a la izquierda tiene el mismo índice
			rightChopstick: chopsticks[(i+1)%numPhilosophers], // Palillo a la derecha es el siguiente (circular)
		}
	}

	// WaitGroup para esperar que todas las goroutines terminen
	var wg sync.WaitGroup
	wg.Add(numPhilosophers) // Indica que esperamos a 'numPhilosophers' goroutines

	// Lanzar una goroutine por cada filósofo
	fmt.Println("La cena ha comenzado...")
	for i := 0; i < numPhilosophers; i++ {
		go philosophers[i].dine(&wg) // Pasa el WaitGroup a la goroutine
	}

	// Esperar a que todos los filósofos terminen (wg.Done() sea llamado numPhilosophers veces)
	wg.Wait()

	fmt.Println("La cena ha terminado.")
	fmt.Println("Problema de los Filósofos Comensales - Fin (Go)")
}
