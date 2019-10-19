use std::io;
use termion::raw::IntoRawMode;
use tui::{Frame, Terminal};
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Layout, Constraint, Direction, Rect, Alignment};
use tui::widgets::{Widget, Block, Borders, Text, Paragraph, Table, Row, Tabs};
use tui::style::{Color, Modifier, Style};


fn render_paragraph<B>(f: &mut Frame<B>, text: &[Text], layout: &Rect, alignment: &Alignment)
    where
        B: Backend
{
    Paragraph::new(text.iter())
        .alignment(*alignment)
        .wrap(true)
        .render(f, *layout);
}

fn create_layout(parent: &Rect, direction: Direction, percentages: &[u16], margin: u16) -> Vec<Rect>
{
    let constraints: Vec<Constraint> = percentages.iter()
        .map(|percentage| Constraint::Percentage(*percentage) ).collect();

    Layout::default()
        .direction(direction)
        .margin(margin)
        .constraints(constraints)
        .split(*parent)
}

fn draw_all_layout<B>(f: &mut Frame<B>)
    where
        B: Backend
{
    let layout_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(15), // player header
                Constraint::Percentage(10), // stats
                Constraint::Percentage(70), // main
                Constraint::Percentage(5)  // footer
            ].as_ref()
        )
        .split(f.size());

    draw_player_header(f, layout_chunks[0]);

    draw_player_stats(f, layout_chunks[1]);

    draw_main(f, layout_chunks[2]);

    draw_footer(f, layout_chunks[3]);
}

// ####### PLAYER HEADER ########
fn draw_player_header<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    Block::default()
        .title("Character")
        .borders(Borders::ALL)
        .render(f, layout_chunk);

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10), // Picture
                Constraint::Percentage(30), // Name
                Constraint::Percentage(30), // Free Space
                Constraint::Percentage(40)  // Short / Long Rests
            ].as_ref()
        )
        .split(layout_chunk);

    let text = [
        Text::styled("\nDandelion\n", Style::default().fg(Color::White).modifier(Modifier::BOLD)),
        Text::styled("Rock Gnome Wizard Lvl 3\n", Style::default())
    ];

    render_paragraph(f, &text, &inner_layout[1], &Alignment::Left);

    draw_picture(f, inner_layout[0]);

    draw_rests(f, inner_layout[3]);
}

fn draw_picture<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    Block::default()
        .borders(Borders::ALL)
        .render(f, layout_chunk);
}

fn draw_rests<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(50), // Short Rest
                Constraint::Percentage(50), // Long Rest
            ].as_ref()
        )
        .split(layout_chunk);

    draw_short_rest(f, inner_layout[0]);
    draw_long_rest(f, inner_layout[1]);
}

fn draw_short_rest<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    let button_size = Rect { height: 3, ..layout_chunk };
    Block::default()
        .borders(Borders::ALL).render(f, button_size);

    let inner_short_rest_layout = Layout::default()
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(100), // Short Rest
            ].as_ref()
        )
        .split(layout_chunk);

    let short_rest = [
        Text::styled("S", Style::default().fg(Color::White).modifier(Modifier::UNDERLINED)),
        Text::styled("hort rest ⛺", Style::default().fg(Color::White)),
    ];

    render_paragraph(f, &short_rest, &inner_short_rest_layout[0], &Alignment::Center);
}

fn draw_long_rest<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    let button_size = Rect { height: 3, ..layout_chunk };
    Block::default()
        .borders(Borders::ALL).render(f, button_size);

    let inner_long_rest_layout = Layout::default()
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(100), // Short Rest
            ].as_ref()
        )
        .split(layout_chunk);

    let long_rest = [
        Text::styled("L", Style::default().fg(Color::White).modifier(Modifier::UNDERLINED)),
        Text::styled("ong rest 🌖", Style::default().fg(Color::White)),
    ];

    render_paragraph(f, &long_rest, &inner_long_rest_layout[0], &Alignment::Center);
}
// ####### END PLAYER HEADER ########

// ####### PLAYER STATS ########
fn draw_player_stats<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    Block::default()
        .title("Stats")
        .borders(Borders::ALL)
        .render(f, layout_chunk);

    let inner_layout = Layout::default()
        .margin(1)
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(70), // Main stats
                Constraint::Percentage(30), // HP
            ].as_ref()
        )
        .split(layout_chunk);

    draw_stats(f, inner_layout[0]);

    draw_hitpoints(f, inner_layout[1]);
}

fn draw_stats<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    let stats_text = [
        Text::styled("STR: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("11 (+0) | ", Style::default()),
        Text::styled("DEX: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("17 (+3) | ", Style::default()),
        Text::styled("CON: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("13 (+1) | ", Style::default()),
        Text::styled("INT: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("19 (+4) | ", Style::default()),
        Text::styled("WIS: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("12 (+1) | ", Style::default()),
        Text::styled("CHA: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("11 (+0)", Style::default()),
        Text::raw("\n"),
        Text::styled("Armor class: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("14 | ", Style::default()),
        Text::styled("Initiative: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("+3 | ", Style::default()),
        Text::styled("Proficiency bonus: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("+2 | ", Style::default()),
        Text::styled("Walking speed: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("25 ft", Style::default()),
    ];

    render_paragraph(f, &stats_text, &layout_chunk, &Alignment::Left);
}

fn draw_hitpoints<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    let hp_text = [
        Text::styled("Hit Points: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("14 ", Style::default()),
        Text::styled("/", Style::default().modifier(Modifier::BOLD)),
        Text::styled(" 17 \n", Style::default()),
        Text::styled("H", Style::default().modifier(Modifier::UNDERLINED)),
        Text::styled("eal | ", Style::default()),
        Text::styled("D", Style::default().modifier(Modifier::UNDERLINED)),
        Text::styled("amage", Style::default()),
    ];

    render_paragraph(f, &hp_text, &layout_chunk, &Alignment::Right);
}

// ####### END PLAYER STATS ########

// ####### MAIN ########
fn draw_main<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    let inner_layout = create_layout(
        &layout_chunk, Direction::Horizontal, &[25, 25, 50], 1);

    draw_left_column(f, inner_layout[0]);

    draw_center_column(f, inner_layout[1]);

    draw_main_panel(f, inner_layout[2]);
}

fn draw_left_column<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    let inner_layout = create_layout(
        &layout_chunk, Direction::Vertical, &[35, 65], 1);

    draw_saving_throws(f, inner_layout[0]);

    draw_proficiencies(f, inner_layout[1]);
}

fn draw_saving_throws<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    Block::default()
        .title("Saving Throws")
        .borders(Borders::ALL)
        .render(f, layout_chunk);

    let inner_layout = create_layout(
        &layout_chunk, Direction::Vertical, &[80, 20], 2);

    let saving_throws = [
        Text::styled("( ) STR: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("+0 | ", Style::default()),
        Text::styled("( ) DEX: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("+3 \n", Style::default()),
        Text::styled("( ) CON: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("+1 | ", Style::default()),
        Text::styled("(*) INT: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("+6 \n", Style::default()),
        Text::styled("(*) WIS: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("+3 | ", Style::default()),
        Text::styled("( ) CHA: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("+0", Style::default()),
    ];

    render_paragraph(f, &saving_throws, &inner_layout[0], &Alignment::Center);

    let advantages = [
        Text::styled("Advantage on ", Style::default()),
        Text::styled("INT WIS CHA ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("against Magic", Style::default()),
    ];

    render_paragraph(f, &advantages, &inner_layout[1], &Alignment::Left);
}

fn draw_proficiencies<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    Block::default()
        .title("Proficiencies & Languages")
        .borders(Borders::ALL)
        .render(f, layout_chunk);

    let inner_layout = create_layout(
        &layout_chunk, Direction::Vertical, &[100], 2);

    let proficiencies = [
        Text::styled("ARMOR \n", Style::default().modifier(Modifier::BOLD).fg(Color::White)),
        Text::styled("None \n\n", Style::default()),
        Text::styled("WEAPONS \n", Style::default().modifier(Modifier::BOLD).fg(Color::White)),
        Text::styled("Crossbow, Light, Dagger, Dart, Quarterstaff, Sling \n\n", Style::default()),
        Text::styled("TOOLS \n", Style::default().modifier(Modifier::BOLD).fg(Color::White)),
        Text::styled("Tinker's Tools \n\n", Style::default()),
        Text::styled("LANGUAGES \n", Style::default().modifier(Modifier::BOLD).fg(Color::White)),
        Text::styled("Common, Dwarvish, Elvish, Gnomish", Style::default()),
    ];

    render_paragraph(f, &proficiencies, &inner_layout[0], &Alignment::Left);
}

fn draw_center_column<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    let inner_layout = create_layout(
        &layout_chunk, Direction::Vertical, &[80, 20], 1);

    draw_skills(f, inner_layout[0]);

    draw_senses(f, inner_layout[1]);
}

fn draw_skills<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    Block::default()
        .title("Skills")
        .borders(Borders::ALL)
        .render(f, layout_chunk);

    let inner_layout = create_layout(
        &layout_chunk, Direction::Vertical, &[100], 1);

    let row_style = Style::default().fg(Color::White);
    Table::new(
        ["Prof", "Mod", "Skill", "Bonus"].into_iter(),
        vec![
            Row::StyledData(["   ", "DEX", "Acrobatics", "+3"].into_iter(), row_style),
            Row::StyledData(["   ", "DEX", "Animal Handling", "+1"].into_iter(), row_style),
            Row::StyledData([" ⭐️ ", "DEX", "Arcana", "+6"].into_iter(), row_style),
            Row::StyledData(["   ", "DEX", "Athletics", "+0"].into_iter(), row_style),
            Row::StyledData(["   ", "DEX", "Deception", "+0"].into_iter(), row_style),
            Row::StyledData([" ⭐️ ", "DEX", "History", "+6"].into_iter(), row_style),
            Row::StyledData(["   ", "DEX", "Insight", "+1"].into_iter(), row_style),
            Row::StyledData(["   ", "DEX", "Intimidation", "+0"].into_iter(), row_style),
            Row::StyledData([" ⭐️ ", "DEX", "Investigation", "+6"].into_iter(), row_style),
            Row::StyledData(["   ", "DEX", "Medicine", "+1"].into_iter(), row_style),
            Row::StyledData(["   ", "DEX", "Nature", "+4"].into_iter(), row_style),
            Row::StyledData(["   ", "DEX", "Perception", "+1"].into_iter(), row_style),
            Row::StyledData(["   ", "DEX", "Performance", "+0"].into_iter(), row_style),
            Row::StyledData(["   ", "DEX", "Religion", "+4"].into_iter(), row_style),
            Row::StyledData(["   ", "DEX", "Sleight of Hand", "+3"].into_iter(), row_style),
            Row::StyledData([" ⭐️ ", "DEX", "Stealth", "+5"].into_iter(), row_style),
            Row::StyledData(["   ", "DEX", "Survival", "+1"].into_iter(), row_style),
        ].into_iter()
    )
        .header_style(Style::default().fg(Color::Yellow))
        .widths(&[4, 3, 15, 4])
        .style(Style::default().fg(Color::White))
        .column_spacing(2)
        .render(f, inner_layout[0]);
}

fn draw_senses<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    Block::default()
        .title("Senses")
        .borders(Borders::ALL)
        .render(f, layout_chunk);

    let inner_layout = create_layout(
        &layout_chunk, Direction::Vertical, &[80, 20], 1);

    let senses = [
        Text::styled("11   Passive WIS (Perception) \n", Style::default().fg(Color::White)),
        Text::styled("16   Passive INT (Investigation) \n", Style::default().fg(Color::White)),
        Text::styled("11   Passive WIS (Insight) \n", Style::default().fg(Color::White)),
    ];

    render_paragraph(f, &senses, &inner_layout[0], &Alignment::Left);

    let darkvision = [
        Text::styled("Darkvision 60 ft. ", Style::default()),
    ];

    render_paragraph(f, &darkvision, &inner_layout[1], &Alignment::Center);
}

fn draw_main_panel<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    Block::default()
        .borders(Borders::ALL)
        .render(f, layout_chunk);

    let inner_layout = create_layout(
        &layout_chunk, Direction::Vertical, &[10, 90], 1);

    Tabs::default()
        .titles(&["Actions", "Spells", "Equipment", "Features & Traits", "Description"])
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(1)
        .render(f, inner_layout[0]);

    draw_spells_tab(f, inner_layout[1]);
}

fn draw_spells_tab<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    let inner_layout = create_layout(
        &layout_chunk, Direction::Vertical, &[10, 90], 2);

    draw_modifiers(f, inner_layout[0]);

    draw_spells_list(f, inner_layout[1]);
}

fn draw_modifiers<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    let modifiers_text = [
        Text::styled("Modifier: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("+4   ", Style::default()),
        Text::styled("Spell Attack: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("+6   ", Style::default()),
        Text::styled("Save DC: ", Style::default().modifier(Modifier::BOLD)),
        Text::styled("14 ", Style::default()),
    ];

    render_paragraph(f, &modifiers_text, &layout_chunk, &Alignment::Center);
}

fn draw_spells_list<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    Block::default()
        .title("draw_spells_list")
        .borders(Borders::ALL)
        .render(f, layout_chunk);

    let inner_layout = create_layout(
        &layout_chunk, Direction::Vertical, &[100], 2);

    let row_style = Style::default().fg(Color::White);
    Table::new(
        ["Lvl", "Name", "Time", "Range", "HIT/DC", "Effect", "Notes"].into_iter(),
        vec![
            Row::StyledData(["C", "Fire Bolt", "1A", "120ft", "+6", "1d10 🔥", "V/S"].into_iter(), row_style),
            Row::StyledData(["C", "Mage Hand", "1A", "30ft", "-", "Utility", "D: 1m, V/S"].into_iter(), row_style),
            Row::StyledData(["C", "Prestidigitation", "1A", "10ft", "-", "Utility", "D: 1m, V/S"].into_iter(), row_style),
            Row::StyledData(["1", "Burning Hands", "1A", "Self", "DEX 14", "3d6 🔥", "15ft cone, V/S"].into_iter(), row_style),
            Row::StyledData(["1", "Find Familiar", "1h", "10ft", "-", "Summoning", "V/S/M"].into_iter(), row_style),
            Row::StyledData(["1", "Identify", "1m", "Touch", "-", "Detection", "V/S/M"].into_iter(), row_style),
            Row::StyledData(["1", "Illusory Script", "1m", "Touch", "-", "Communication", "D: 10d, S/M"].into_iter(), row_style),
            Row::StyledData(["1", "Mage Armor", "1A", "Touch", "-", "Buff*", "D: 8h, V/S/M"].into_iter(), row_style),
            Row::StyledData(["1", "Magic Missile", "1A", "120ft", "-", "1d4+1 ☄", "V/S"].into_iter(), row_style),
            Row::StyledData(["2", "Burning Hands", "1A", "Self", "DEX 14", "4d6 🔥", "15ft cone, V/S"].into_iter(), row_style),
            Row::StyledData(["2", "Magic Missile", "1A", "120ft", "-", "1d4+1 ☄", "Count: +1, V/S"].into_iter(), row_style),
        ].into_iter()
    )
        .header_style(Style::default().fg(Color::Yellow))
        .widths(&[2, 20, 3, 6, 6, 8, 15])
        .style(Style::default().fg(Color::White))
        .column_spacing(2)
        .render(f, inner_layout[0]);
}
// ####### END MAIN ########

// ####### FOOTER ########
fn draw_footer<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    Block::default()
        .title("draw_footer")
        .borders(Borders::ALL)
        .render(f, layout_chunk);
}
// ####### END FOOTER ########

pub fn draw_app() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    terminal.draw(|mut f| {
        draw_all_layout(&mut f);
    })
}