use std::fmt::format;

use rusty_engine::{game, prelude::{bevy::utils::label, *}};
#[derive(Resource)]
struct GameState
{
    high_score: i32,
    score: i32,
    enemy_index: i32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}
impl Default for GameState
{
    fn default() -> Self {
        Self
        {
            high_score: 0,
            score: 0,
            enemy_index: 0,
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

    // UI
    let high_score = game.add_text("High Score", "High Score 100");
    high_score.translation = Vec2::new(-300.0,-100.0);
    let score = game.add_text("Score ", "Score 0");
    score.translation = Vec2::new(300.0,100.0);

    //
    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine : &mut Engine, game_state: &mut GameState)
{
    // Collision
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
            game_state.score += 1;
            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Score : {}",game_state.score);
            if game_state.score > game_state.high_score
            {
                game_state.high_score = game_state.score;
                let highscore = engine.texts.get_mut("high_score").unwrap();
                highscore.value = format!("High Score : {}",game_state.score);
            }
        }
    }
    // Input 
    const MOVEMENT_SPEED : f32 = 100.0;
    if engine.keyboard_state.pressed_any(&[KeyCode::Up,KeyCode::W])
    {
        let player = engine.sprites.get_mut("player").unwrap();
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }
    // Mouse Handle
    if engine.mouse_state.just_pressed(MouseButton::Left)
    {
        if let Some(mouse_location) = engine.mouse_state.location()
        {
            let label = format!("Enemy{}",game_state.enemy_index);
            let enemy = engine.add_sprite(label.clone(), SpritePreset::RacingBarrelRed);
            enemy.translation = mouse_location;
            game_state.enemy_index +=1;
            enemy.collision = true;
        }
    }
}