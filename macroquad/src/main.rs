use evolution_simulation::World;

#[macroquad::main("Evolution simulation")]
async fn main() {
    let world = World::new(20, 4);
    world.run().await;
}
