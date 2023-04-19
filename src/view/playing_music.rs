use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Modifier, Style};
use tui::text::Span;
use tui::widgets::{
    Block, //与所有上层小部件一起使用的基本小部件。 它可用于在小部件周围显示框边框和/或添加标题。
    BorderType,
    Borders, //可以组成的位标志基本上在块小部件上设置可见边界。
    Gauge,   //显示任务进度的小部件。
};
use tui::Frame;

use crate::app::PlayStyle;
use crate::music::Music;

use super::color::Theme;

pub fn draw_playing_music<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    theme: &Theme,
    playing_music: &Option<Music>,
    is_paused: bool,
    volume: f32,
    play_style: &PlayStyle,
) {
    let mut play_style_icon = "🔂";
    let mut label = "";
    let mut percent = 0;
    match play_style {
        PlayStyle::SingleCycle => play_style_icon = "🔁",
        _ => {}
    }

    let mut block_title: Vec<Span> = vec![Span::styled(
        " Playing ",
        Style::default().fg(theme.play_music_list_title_color),
    )];

    let mut gauge_title: Vec<Span> = Vec::new();

    if let Some(music) = playing_music {
        if is_paused {
            label = " ⏸ ";
        } else {
            label = " ▶ ";
        }

        percent = ((music.play_position.as_secs_f32() / music.total_duration.as_secs_f32())
            * (100 as f32))
            .round() as u16;
        if percent > 100 {
            percent = 100;
        }

        block_title.push(Span::styled(
            "💽 ",
            Style::default().fg(theme.list_icon_color),
        ));
        block_title.push(Span::styled(
            format!("{} ", music.name),
            Style::default()
                .fg(theme.playing_music_name_color)
                .add_modifier(Modifier::BOLD),
        ));

        let play_dur = music.play_position.as_secs();
        let total_dur = music.total_duration.as_secs();
        gauge_title.push(Span::styled(
            format!(
                " [ {}m {}s : {}m {}s ] {} ",
                play_dur / 60,
                play_dur % 60,
                total_dur / 60,
                total_dur % 60,
                play_style_icon,
            ),
            Style::default().fg(theme.list_music_color),
        ))
    }

    // Volume
    {
        block_title.push(Span::styled(
            match volume {
                v if v >= 0.7 => "🔊 ",
                v if v >= 0.3 => "🔉",
                v if v > 0.0 => "🔈",
                _ => "🎐",
            },
            Style::default().fg(theme.volume_icon_color),
        ));
        let volume = if volume > 0.0 { volume } else { 0.0 };
        block_title.push(Span::styled(
            format!("{:3.0}% ", volume * 100.0),
            Style::default().fg(theme.volume_value_color),
        ));
    }

    // Playing music block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.playing_music_border_color))
        .title(block_title)
        .title_alignment(Alignment::Center);
    frame.render_widget(block, area);

    let inner_rect = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
    let gauge = Gauge::default() //A widget to display a task progress.
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(theme.gauge_border_color))
                .title(gauge_title),
        )
        .gauge_style(Style::default().fg(theme.gauge_color))
        .label(Span::styled(
            label,
            Style::default().fg(theme.gauge_label_color),
        ))
        .percent(percent);
    frame.render_widget(gauge, inner_rect);
}
