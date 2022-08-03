use std::slice::Chunks;

use crossterm::terminal::enable_raw_mode;
use tui::{
    layout::{Constraint, Direction},
    widgets::Paragraph,
};
use xlsxwriter::DateTime;

const DB_PATH: str = "/database.json"; //this will be a simple list of all the different items stored

struct Item {
    //TODO maybe implement stntrading shit here?
    name: String,
    category: String,
    is_craftable: bool,
    related_item: Item,
    mptf_price: u32,
    mptf_converted_price_ref: u32,
    mptf_converted_price_keys: u32,
    mptf_converted_price_pretty: (u16, u32),
    scrap_tf_price_ref: u32,
    scrap_tf_price_key: f32,
    scrap_tf_price_pretty: (u16, u32),
    last_updated: DateTime<Utc>,
}
pub enum errors {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}
enum Event<I> {
    Input(I),
    Tick,
}
enum MainMenuItems {
    Home,
    Items,
    PriceList,
    Visualization,
    Settings,
    Quit,
}
impl From<MainMenuItems> for usize {
    fn from(input: MainMenuItems) -> usize {
        match input {
            MainMenuItems::Home => 0,
            MainMenuItems::Items => 1,
            MainMenuItems::PriceList => 2,
            MainMenuItems::Visualization => 3,
            MainMenuItems::Quit => 4,
            MainMenuItems::Settings => 5,
        }
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode idiot!");
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec![
        "Home",
        "Items",
        "PriceList",
        "Visualizations",
        "Settings",
        "Quit",
    ];
    let mut active_menu_item: MainMenuItems = MainMenuItems::Home;
    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);
        });
        let copyright = Paragraph::new("FUCKING NONE LOLE!!! HAHAHA!!!")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(alignment::center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Copyright")
                    .border_type(BorderType::Plain),
            );
    }
}
