use rusty_engine::{game, prelude::*};
#[derive(Resource)]
struct GameState
{
    high_score: i32,
    curr_score: i32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}
impl Default for GameState
{
    fn default() -> Self {
        Self
        {
            high_score: 0,
            curr_score: 0,
            enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(1.0,TimerMode::Once),
        }
    }
}
fn main() {
    let mut game = Game::new();
    let player = game.add_sprite("player",SpritePreset::RacingCarGreen);
    // player
    player.translation = Vec2::new(100.0,-10.4);
    player.rotation = UP;
    player.scale = 1.0;
    player.layer = 1.0;
    player.collision = true;

    let enemy = game.add_sprite("enemy", SpritePreset::RacingBarrelRed);
    enemy.translation = Vec2::new(-100.0,-10.4);
    enemy.collision = true;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine : &mut Engine, game_state: &mut GameState)
{
    engine.show_colliders = true;
    for event in engine.collision_events.drain(..)
    {
        println!("{:#?}",event);
        if event.state == CollisionState::Begin && event.pair.one_starts_with("enemy")
        {
            for label in [event.pair.0,event.pair.1]
            {
                if label != "player"{
                    engine.sprites.remove(&label);
                }
            }
            game_state.curr_score += 1;
            println!("current score : {}",game_state.curr_score);

        }

    }

    let player = engine.sprites.get_mut("player").unwrap();
    player.translation.x += 100.0 * engine.delta_f32;
}