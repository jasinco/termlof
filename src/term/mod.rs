use std::error;
use terminal::Action;
use tui::{
    backend::CrosstermBackend,
    layout,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

pub fn start_term() -> Result<(), Box<dyn error::Error>> {
    let stdout = terminal::stdout();
    stdout.act(Action::EnableRawMode)?;
    stdout.act(Action::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(terminal::stdout());
    let mut term = Terminal::new(backend)?;
    
    let urls = ["WWWW","WW"];
    let mut lofi_liststate = ListState::default();
    lofi_liststate.select(Some(0));


    loop { 
        term.draw(|f| {
            let size = f.size();
            let chunks = layout::Layout::default()
                .direction(layout::Direction::Horizontal)
                .margin(2)
                .constraints(
                    [
                        layout::Constraint::Percentage(50),
                        layout::Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(size);
            let items = urls.map(|f|{ListItem::new(f)});
            let listf = List::new(items)
                .block(Block::default().title("Music List").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");
            f.render_stateful_widget(listf, chunks[0], &mut lofi_liststate);
        })?;
        if let terminal::Retrieved::Event(Some(terminal::Event::Key(event))) = stdout
            .get(terminal::Value::Event(None))
            .unwrap()
        {
            match event.code {
                terminal::KeyCode::Char('q') => {
                    stdout.act(Action::LeaveAlternateScreen)?;
                    break;
                },
                terminal::KeyCode::Down=>{
                    if lofi_liststate.selected().unwrap() < urls.len()-1{
                        lofi_liststate.select(Some(lofi_liststate.selected().unwrap()+1));
                    }
                },
                terminal::KeyCode::Up=>{
                    if lofi_liststate.selected().unwrap()>0{
                        lofi_liststate.select(Some(lofi_liststate.selected().unwrap()-1));
                    }
                },
                _ => {
                    continue;
                }
            }
        }
    }
    stdout.act(Action::DisableRawMode)?;
    stdout.act(Action::ShowCursor)?;

    Ok(())
}
