use evolution_simulation::World;

#[macroquad::main("Evolution simulation")]
async fn main() {
    let world = World::new(60, 5);
    world.run().await;
}
