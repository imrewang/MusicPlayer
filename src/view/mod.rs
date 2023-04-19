use exitfailure::ExitFailure;
use tui::layout::{Constraint, Direction, Layout};

use crate::app::App;
use crate::config::InitTheme;

use self::color::Theme;
use self::music_list::draw_music_list;
use self::play_music_list::draw_play_music_list;
use self::playing_music::draw_playing_music;
pub mod color;
mod display;
mod music_list;
mod play_music_list;
mod playing_music;

pub fn handle_theme(init_theme: InitTheme) -> Theme {
    Theme::new(init_theme)
}

pub fn draw(app: &mut App, theme: &Theme) -> Result<(), ExitFailure> {
    let search_string = app.get_search_string();
    let command_string = app.get_command_string();
    app.terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal) //水平
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref()) //百分比
            .split(f.size());

        draw_music_list(
            //左边竖列
            f,                          // frame: &mut Frame<B>,
            chunks[0],                  //area: Rect,
            theme,                      //theme: &Theme,
            app.window_height as usize, //window_height: usize,
            &app.directory_contents,    //files: &Vec<DirectoryItem>,
            &app.selection_index,       //selected_index: &Option<usize>,
            &search_string,             //search_string: &str,
            &command_string,            // command_string: &str,
            &app.error,                 //error: &Option<String>,
        );

        //Create the list chunks
        let chunks_right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(5)])
            .split(chunks[1]);

        draw_play_music_list(
            //右上方框
            f,                      //frame: &mut Frame<B>,
            chunks_right[0],        //area: Rect,
            &theme,                 // theme: &Theme,
            &app.play_music_list,   //music_list: &Vec<Music>,
            &app.playing_music,     //playing_music: &Option<Music>,
            app.player.is_paused(), //is_paused: bool,
        );

        draw_playing_music(
            //右下角条框
            f,                      //frame: &mut Frame<B>,
            chunks_right[1],        //area: Rect,
            &theme,                 //theme: &Theme,
            &app.playing_music,     //playing_music: &Option<Music>,
            app.player.is_paused(), //is_paused: bool,
            app.player.volume(),    //音量volume: f32,
            &app.play_style,        //play_style: &PlayStyle,
        );
    })?;

    Ok(())
}
