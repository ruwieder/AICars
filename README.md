# AI Car Simulation with Genetic Algorithm

[AI Car Simulation]

A high-performance simulation of AI-controlled cars navigating a track using neural networks and genetic algorithms. The system evolves car AI through natural selection, where successful cars produce offspring with improved driving abilities.

## Features

- **Massive parallel population**: 20,000 cars simulated simultaneously using Rayon
- **Neural network control**: Each car has a custom neural network that processes track sensors
- **Genetic evolution**: Natural selection with parent selection and mutation
- **Real-time visualization**: Track rendering and fitness metrics display
- **Efficient performance**: Parallel processing for car updates and evolution

## How It Works

1. **Track Generation**: A static track is generated with predefined geometry
2. **Car Initialization**: Each car starts with:
   - Random neural network weights
   - Ray sensors to detect track boundaries
   - Initial position and orientation
3. **Simulation Loop**:
   - Cars move and update based on neural network outputs
   - Collision detection with track boundaries
   - Fitness calculated based on checkpoint reached and distance traveled
4. **Genetic Evolution** (every 10 frames):
   - Dead cars are replaced with new offspring
   - Parents selected based on fitness (roulette wheel selection)
   - Offspring inherit weights from parents with mutation

## Requirements

- Rust 1.65+ (installed via [rustup](https://rustup.rs))
- Graphics-capable system (OpenGL 3.3+)

## Installation & Running

```bash
# Clone the repository
git clone https://github.com/ruwieder/AICars.git
cd ai-car-simulation

# Build and run
cargo run
```

## Controls

- **ESC**: Exit the simulation
- (No other controls - simulation runs automatically)

### Genetic Algorithm
1. **Selection**: Roulette wheel selection based on fitness
2. **Crossover**: Single-point weight crossover between parents
3. **Mutation**: Random weight adjustments (±10% of current value)
4. **Elitism**: Top-performing cars survive unchanged

### Performance Optimizations
- Parallel car updates using Rayon
- Only high-fitness cars drawn each frame
- Minimal memory allocations during simulation

## Performance Notes

The simulation is designed for high performance:
- Handles 20,000 cars at 60+ FPS on modern hardware
- Uses multi-threaded processing for physics updates
- Avoids unnecessary allocations during simulation loop

## Future Improvements

- Dynamic track generation
- Visual representation of neural network
- Adaptive mutation rate based on performance
- Multiple track types and obstacles

---

> **Note**: This project uses [Macroquad](https://github.com/not-fl3/macroquad) for graphics and [Rayon](https://github.com/rayon-rs/rayon) for parallelism. The neural network is implemented from scratch for educational purposes.