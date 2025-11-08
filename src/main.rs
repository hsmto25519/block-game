use macroquad::prelude::*;
use std::cmp::max;
use ::rand::thread_rng;
use ::rand::Rng;

// --- Game Logic Copied From Your Original Code ---

#[derive(Clone, Copy)]
struct Block {
    x: usize,
    y: usize,
}

const LEVEL_THRESHOLD_SCORE: usize = 10;
const UPPER_LEVEL: usize = 15;

/// Moves all blocks down by one row.
fn move_blocks(blocks: &mut Vec<Block>) {
    for b in blocks.iter_mut() {
        b.y += 1;
    }
    blocks.retain(|b| b.y < 10); // remove off-screen blocks
}

/// Randomly spawns a new block at the top.
fn spawn_block(blocks: &mut Vec<Block>, width: usize, score: usize) {
    // We must be specific and use macroquad's re-export of rand
    // let mut rng = macroquad::rand::thread_rng();
    let mut rng = thread_rng();

    // Level 0-9
    let level = (score / LEVEL_THRESHOLD_SCORE).min(UPPER_LEVEL);
    
    // Spawn chance increases from 0.3 (30%) to 0.66 (66%)
    let spawn_chance = 0.3 + (level as f32 * 0.04); 
    
    // Max blocks to spawn this tick. Increases from 1 up to 4.
    let max_spawns = 1 + (level / 3);

    for _ in 0..max_spawns {
        // The Rng trait (for .gen and .gen_range) is in scope from the prelude
        if rng.r#gen::<f32>() < spawn_chance {
            blocks.push(Block {
                x: rng.r#gen_range(0..width),
                y: 0,
            });
        }
    }
}

/// Checks if any block hits the player position.
fn detect_collision(blocks: &[Block], player_x: usize, height: usize) -> bool {
    blocks.iter().any(|b| b.x == player_x && b.y == height - 1)
}

// --- New GUI Code ---

// This function configures the window
fn window_conf() -> Conf {
    Conf {
        window_title: "Block Dodger".to_string(),
        window_width: 640,  // Window width in pixels
        window_height: 480, // Window height in pixels
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // ---- Game setup ----
    const GAME_WIDTH: usize = 20;  // Your grid width
    const GAME_HEIGHT: usize = 10; // Your grid height
    let mut player_x = GAME_WIDTH / 2;
    let mut blocks: Vec<Block> = Vec::new();
    let mut score = 0;
    
    // We need to scale your grid units (e.g., 20) to pixel units (e.g., 640)
    let block_size_w = screen_width() / GAME_WIDTH as f32;
    let block_size_h = screen_height() / GAME_HEIGHT as f32;

    let mut game_over = false;

    // Timer for game ticks (replaces tokio::sleep)
    let mut last_update = get_time(); 

    // ---- Game loop ----
    loop {
        if !game_over {
            // 1. Handle Input (replaces handle_input)
            if is_key_pressed(KeyCode::Left) && player_x > 0 {
                player_x -= 1;
            }
            if is_key_pressed(KeyCode::Right) && player_x < GAME_WIDTH - 1 {
                player_x += 1;
            }
            if is_key_pressed(KeyCode::Escape) {
                break; // Exit the loop
            }

            // 2. Update Game State (runs on a timer, not every frame)
            let level = (score / LEVEL_THRESHOLD_SCORE).min(UPPER_LEVEL) as u64;
            let current_speed_s = max(250 - (level * 20), 1) as f64 / 1000.0;

            if get_time() - last_update > current_speed_s {
                last_update = get_time(); // Reset timer

                move_blocks(&mut blocks);
                spawn_block(&mut blocks, GAME_WIDTH, score);
                
                if detect_collision(&blocks, player_x, GAME_HEIGHT) {
                    game_over = true;
                }
                score += 1;
            }
        }

        // 3. Draw Frame (replaces draw_frame)
        clear_background(LIGHTGRAY); // Clear the window

        // Draw blocks
        for block in &blocks {
            draw_rectangle(
                block.x as f32 * block_size_w, // x pos in pixels
                block.y as f32 * block_size_h, // y pos in pixels
                block_size_w,                  // width in pixels
                block_size_h,                  // height in pixels
                BLACK,
            );
        }

        // Draw player
        draw_rectangle(
            player_x as f32 * block_size_w,
            (GAME_HEIGHT - 1) as f32 * block_size_h,
            block_size_w,
            block_size_h,
            BLUE,
        );

        // Draw score
        let level_display = (score / LEVEL_THRESHOLD_SCORE).min(UPPER_LEVEL) + 1;
        draw_text(
            &format!("Score: {} | Level: {}", score, level_display),
            10.0, 20.0, 30.0, DARKGRAY,
        );

        if game_over {
             draw_text(
                &format!("Game Over! Final Score: {}", score),
                screen_width() / 2.0 - 150.0, 
                screen_height() / 2.0, 
                40.0, RED,
            );
        }

        // This replaces your sleep() and handles window events
        next_frame().await
    }
}