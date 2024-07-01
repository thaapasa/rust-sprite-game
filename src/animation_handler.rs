use std::cmp::min;

use ggez::graphics::Image;

use crate::game_assets::GameAssets;

#[derive(Debug)]
pub struct Animation {
    pub tileset_image: fn(&GameAssets) -> &Image,
    frame_count: usize,
    frame_duration: f32,
    current_frame: usize,
    elapsed_time: f32,
    loop_animation: bool,
}

impl Animation {
    pub fn player_idle() -> Animation {
        return Animation {
            tileset_image: |a| &a.player_idle_tiles,
            frame_count: 8,
            frame_duration: 0.13,
            current_frame: 0,
            elapsed_time: 0.0,
            loop_animation: true,
        };
    }

    pub fn player_walking() -> Animation {
        return Animation {
            tileset_image: |a| &a.player_walk_tiles,
            frame_count: 8,
            frame_duration: 0.05,
            current_frame: 0,
            elapsed_time: 0.0,
            loop_animation: true,
        };
    }

    pub fn player_running() -> Animation {
        return Animation {
            tileset_image: |a| &a.player_run_tiles,
            frame_count: 8,
            frame_duration: 0.05,
            current_frame: 0,
            elapsed_time: 0.0,
            loop_animation: true,
        };
    }

    /// Update the animation based on elapsed time.
    pub fn update(&mut self, elapsed: f32) {
        self.elapsed_time += elapsed;
        while self.elapsed_time >= self.frame_duration {
            self.current_frame += 1;
            self.elapsed_time -= self.frame_duration
        }
    }

    /// Return the current frame position
    pub fn get_current_frame(&self) -> usize {
        return if self.loop_animation {
            self.current_frame % self.frame_count
        } else {
            min(self.frame_count - 1, self.current_frame)
        };
    }
}
