use evolution_simulation::World;

#[macroquad::main("Evolution simulation")]
async fn main() {
    let world = World::new(10);
    world.run().await;
}
