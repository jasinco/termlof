use std::{error, io, process::{self, Command}, thread, sync::mpsc};
use terminal::Action;
use tui::{
    backend::CrosstermBackend,
    layout,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};
use crate::{config, ytdlp};
pub fn start_term() -> Result<(), Box<dyn error::Error>> {
    
    //use terminal instead of crossterm, and using same api, so do not worried
    let stdout = terminal::stdout();
    stdout.act(Action::EnableRawMode)?;
    stdout.act(Action::EnterAlternateScreen)?;
    terminalui(terminal::stdout())?;
    stdout.act(Action::LeaveAlternateScreen)?;
    stdout.act(Action::DisableRawMode)?;
    stdout.act(Action::ShowCursor)?;
    Ok(())
}


enum Playstat{
    Url(String),
    Pause
}

fn terminalui(stdout:terminal::Terminal<io::Stdout>) -> Result<(), Box<dyn error::Error>> {
    let backend = CrosstermBackend::new(terminal::stdout());
    let mut term = Terminal::new(backend)?;
    
    //Read termlof.toml

    let mut termconf = config::parse_default()?;
    let mut selection:Vec<&str> = termconf.lofilist().into_iter().chain(termconf.musiclist().into_iter()).collect();
    //create ListState which let list can select things
    let mut liststate = ListState::default();
    liststate.select(Some(0));
    let tx = ffplay();
    let mut status = String::new();

    //Start TUI
    loop { 
        term.draw(|f| {
            let size = f.size();
            let chunks = layout::Layout::default()
                .direction(layout::Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        layout::Constraint::Percentage(50),
                        layout::Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(size);
            
            //change things read from toml into List
            let items = selection.clone().into_iter().map(|f|{ListItem::new(f)}).collect::<Vec<ListItem>>();

            let listf = List::new(items)
                .block(Block::default().title("Music List").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");

            let pg = Paragraph::new(status.as_ref())
                .block(Block::default()
                .title("Hints")
                .borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray))
                .alignment(layout::Alignment::Center)
                .wrap(Wrap{trim:true});
            f.render_widget(pg, chunks[1]);
            f.render_stateful_widget(listf, chunks[0], &mut liststate);
        })?;

        //terminal based keyboard function
        if let terminal::Retrieved::Event(Some(terminal::Event::Key(event))) = stdout
            .get(terminal::Value::Event(None))
            .unwrap()
        {
            match event.code {
                terminal::KeyCode::Char('q') => {
                    tx.send(Playstat::Pause)?;
                    drop(tx);

                    break;
                },
                terminal::KeyCode::Char('r')=>{
                    status = "Reload list".to_string();
                    termconf = config::parse_default()?;
                    selection = termconf.lofilist().into_iter().chain(termconf.musiclist().into_iter()).collect();
                },
                terminal::KeyCode::Down=>{
                    if liststate.selected().unwrap() < selection.len()-1{
                        liststate.select(Some(liststate.selected().unwrap()+1));
                    }
                },
                terminal::KeyCode::Up=>{
                    if liststate.selected().unwrap()>0{
                        liststate.select(Some(liststate.selected().unwrap()-1));
                    }
                },
                terminal::KeyCode::Enter=>{
                    let url = termconf.get_val(selection[liststate.selected().unwrap()]);
                    if url.len() != 0{
                        if let Some(yturl) = ytdlp::get_audio_url(&url).ok(){
                            tx.send(Playstat::Pause)?;
                            tx.send(Playstat::Url(yturl))?;
                            status = format!("Playing {}", selection[liststate.selected().unwrap()]);
                        }else{
                            status = "ERROR url".to_string();
                        }
                    }else{
                        status = "Missing url".to_string();
                    }
                },
                _ => {
                    continue;
                }
            }
        }
    }
    Ok(())
}

fn ffplay() -> mpsc::Sender<Playstat>{
    let (tx, rx) = mpsc::channel::<Playstat>();
    thread::spawn(move || {
        while let Ok(playstat) = rx.recv() {
            if let Playstat::Url(url) = playstat {    
                let mut f = Command::new("ffplay").args([url.as_str(), "-nodisp"]).stdout(process::Stdio::null()).stdin(process::Stdio::null()).stderr(process::Stdio::null()).spawn().unwrap();
                while let Playstat::Pause = rx.recv().unwrap(){
                    f.kill().unwrap();
                    break;
                }
            }
           
        }
    });

    return tx;
}
