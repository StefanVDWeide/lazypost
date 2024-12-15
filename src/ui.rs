use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};
use serde_json::json;

use crate::app::{App, CurrentScreen, CurrentlyEditing};

/// helper funtction to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn create_title() -> Paragraph<'static> {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "LazyPost: Easy HTTP request from the terminal",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    title
}

fn create_request_history_pane(app: &App) -> List<'static> {
    let mut requests = Vec::<ListItem>::new();

    let request_history_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            "Request History",
            Style::default().fg(Color::Cyan),
        ))
        .style(Style::default());

    for request in app.requests.iter() {
        let span = Span::styled(
            format!("{} | {}", request.method, request.url),
            Style::default().fg(Color::Yellow),
        );

        requests.push(ListItem::new(Line::from(span)))
    }

    let list = List::new(requests).block(request_history_block);

    list
}

fn create_tags_pane(app: &App) -> List<'static> {
    let mut requests = Vec::<ListItem>::new();

    let request_history_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled("Tags", Style::default().fg(Color::Cyan)))
        .style(Style::default());
    for request in app.requests.iter() {
        let span = Span::styled(
            format!("{} | {}", request.method, request.url),
            Style::default().fg(Color::Yellow),
        );

        requests.push(ListItem::new(Line::from(span)))
    }

    let list = List::new(requests).block(request_history_block);

    list
}

fn create_response_pane(app: &App) -> Paragraph<'static> {
    let response_block = Block::default()
        .borders(Borders::ALL)
        .title("Response")
        .style(Style::default());

    let test_value = json!({"foo": "bar", "baz": 1});
    let latest_request = app.requests.last();

    if latest_request.is_none() {
        let response = Paragraph::new(Text::raw(
            serde_json::to_string_pretty(&test_value).unwrap(),
        ))
        .block(response_block);

        response
    } else {
        let response = Paragraph::new(Text::raw(
            serde_json::to_string_pretty(&latest_request.unwrap().response).unwrap(),
        ))
        .block(response_block);

        response
    }
}

fn create_navigation_text(app: &App) -> Vec<Span<'static>> {
    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Url => {
                        Span::styled("Editing URL", Style::default().fg(Color::Green))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    current_navigation_text
}

fn create_key_hints(app: &App) -> Span<'_> {
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
        }
    };

    current_keys_hint
}

fn create_request_pop_up(app: &App, frame: &mut Frame, editing: &CurrentlyEditing) {
    let popup_block = Block::default()
        .title("Enter a URL (currently only supports JSON response)")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(popup_block, area);

    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    let url_block = Block::default().title("URL").borders(Borders::ALL);

    // let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);
    //
    // match editing {
    //     CurrentlyEditing::Key => url_block = url_block.style(active_style),
    //     CurrentlyEditing::Value => value_block = value_block.style(active_style),
    // };

    let url_text = Paragraph::new(app.url_input.clone()).block(url_block);
    frame.render_widget(url_text, popup_chunks[0]);

    // let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
    // frame.render_widget(value_text, popup_chunks[1]);
}

fn create_exit_pop_up(frame: &mut Frame) {
    frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
    let popup_block = Block::default()
        .title("Y/N")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let exit_text = Text::styled(
        "Would you like to exit. All state is lost on exit? (y/n)",
        Style::default().fg(Color::Red),
    );
    // the `trim: false` will stop the text from being cut off when over the edge of the block
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(exit_paragraph, area);
}

pub fn ui(frame: &mut Frame, app: &App) {
    let horizontal_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let middle_panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(horizontal_chunks[1]);

    let history_and_tag_panes = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(middle_panes[0]);

    let title = create_title();

    frame.render_widget(&title, horizontal_chunks[0]);

    let response_pane = create_response_pane(app);
    frame.render_widget(response_pane, middle_panes[1]);

    let tag_list = create_tags_pane(app);
    frame.render_widget(tag_list, history_and_tag_panes[1]);

    let request_history_list = create_request_history_pane(app);
    frame.render_widget(request_history_list, history_and_tag_panes[0]);

    let current_navigation_text = create_navigation_text(app);

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = create_key_hints(app);

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(horizontal_chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    // Check if request pop-up needs to be shown
    if let Some(editing) = &app.currently_editing {
        create_request_pop_up(app, frame, editing)
    }

    // Check if the exit pop-up needs to be shown
    if let CurrentScreen::Exiting = app.current_screen {
        create_exit_pop_up(frame)
    }
}
