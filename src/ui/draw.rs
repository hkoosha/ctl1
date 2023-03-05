use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Text};
use tui::widgets::{Block, Borders, BorderType, Cell, Paragraph, Row, Table};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerSmartWidget};

use crate::app::{AppState, AppTui};
use crate::stick::CtlState;

pub fn draw<B>(rect: &mut Frame<B>,
               app_tui: &AppTui,
               ui_state: &Arc<Mutex<AppState>>)
    where B: Backend {
    let size = rect.size();
    check_size(&size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(10)].as_ref())
        .split(size);

    let title = draw_title();
    rect.render_widget(title, chunks[0]);

    draw_ctls(ui_state, rect, &chunks[1]);

    if let Some(logger_widget) = draw_log(app_tui) {
        rect.render_widget(logger_widget, chunks[2]);
    }
}

fn draw_ctls<B>(app: &Arc<Mutex<AppState>>,
                rect: &mut Frame<B>,
                chunk: &Rect)
    where B: Backend {
    let app = {
        app.lock().unwrap().clone()
    };

    let mut tables = vec![];
    for (id, current) in app.current.iter() {
        let previous = &app.previous[id];
        let name = &app.name_mapping[id];
        let by_ctl = draw_ctl(name, current, previous);
        tables.push(by_ctl);
    }

    let constraints = vec![Constraint::Min(50); tables.len()];
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(&constraints[..])
        .split(*chunk);

    for (i, p) in tables.into_iter().enumerate() {
        rect.render_widget(p, body_chunks[i]);
    }
}

fn draw_ctl<'a>(name: &str, current: &CtlState, previous: &CtlState) -> Table<'a> {
    let mut rows = Vec::with_capacity(128);

    if current.exit.is_some() {
        rows.push(bool_row("Exit", previous.exit, current.exit));
    }
    if current.action_a.is_some() {
        rows.push(bool_row("Action A", previous.action_a, current.action_a));
    }
    if current.action_b.is_some() {
        rows.push(bool_row("Action B", previous.action_b, current.action_b));
    }
    if current.action_c.is_some() {
        rows.push(bool_row("Action C", previous.action_c, current.action_c));
    }
    if current.action_h.is_some() {
        rows.push(bool_row("Action H", previous.action_h, current.action_h));
    }
    if current.action_v.is_some() {
        rows.push(bool_row("Action V", previous.action_v, current.action_v));
    }
    if current.action_d.is_some() {
        rows.push(bool_row("Action D", previous.action_d, current.action_d));
    }
    if current.menu_l.is_some() {
        rows.push(bool_row("Menu L", previous.menu_l, current.menu_l));
    }
    if current.menu_r.is_some() {
        rows.push(bool_row("Menu R", previous.menu_r, current.menu_r));
    }
    if current.joy.is_some() {
        rows.push(bool_row("Joy", previous.joy, current.joy));
    }
    if current.cam.is_some() {
        rows.push(bool_row("Cam", previous.cam, current.cam));
    }
    if current.bumper_l.is_some() {
        rows.push(bool_row("Bumper L", previous.bumper_l, current.bumper_l));
    }
    if current.bumper_r.is_some() {
        rows.push(bool_row("Bumper R", previous.bumper_r, current.bumper_r));
    }
    if current.trigger_l.is_some() {
        rows.push(f64_row("Trigger L", previous.trigger_l, current.trigger_l));
    }
    if current.trigger_r.is_some() {
        rows.push(f64_row("Trigger R", previous.trigger_r, current.trigger_r));
    }
    if current.up.is_some() {
        rows.push(bool_row("Up", previous.up, current.up));
    }
    if current.down.is_some() {
        rows.push(bool_row("Down", previous.down, current.down));
    }
    if current.left.is_some() {
        rows.push(bool_row("Left", previous.left, current.left));
    }
    if current.right.is_some() {
        rows.push(bool_row("Right", previous.right, current.right));
    }
    if current.pov_up.is_some() {
        rows.push(bool_row("Pov Up", previous.pov_up, current.pov_up));
    }
    if current.pov_down.is_some() {
        rows.push(bool_row("Pov Down", previous.pov_down, current.pov_down));
    }
    if current.pov_left.is_some() {
        rows.push(bool_row("Pov Left", previous.pov_left, current.pov_left));
    }
    if current.pov_right.is_some() {
        rows.push(bool_row("Pov Right", previous.pov_right, current.pov_right));
    }
    if current.hat_up.is_some() {
        rows.push(bool_row("Hat Up", previous.hat_up, current.hat_up));
    }
    if current.hat_down.is_some() {
        rows.push(bool_row("Hat Down", previous.hat_down, current.hat_down));
    }
    if current.hat_left.is_some() {
        rows.push(bool_row("Hat Left", previous.hat_left, current.hat_left));
    }
    if current.hat_right.is_some() {
        rows.push(bool_row("Hat Right", previous.hat_right, current.hat_right));
    }
    if current.trim_up.is_some() {
        rows.push(bool_row("Trim Up", previous.trim_up, current.trim_up));
    }
    if current.trim_down.is_some() {
        rows.push(bool_row("Trim Down", previous.trim_down, current.trim_down));
    }
    if current.trim_left.is_some() {
        rows.push(bool_row("Trim Left", previous.trim_left, current.trim_left));
    }
    if current.trim_right.is_some() {
        rows.push(bool_row("Trim Right", previous.trim_right, current.trim_right));
    }
    if current.mic_up.is_some() {
        rows.push(bool_row("Mic Up", previous.mic_up, current.mic_up));
    }
    if current.mic_down.is_some() {
        rows.push(bool_row("Mic Down", previous.mic_down, current.mic_down));
    }
    if current.mic_left.is_some() {
        rows.push(bool_row("Mic Left", previous.mic_left, current.mic_left));
    }
    if current.mic_right.is_some() {
        rows.push(bool_row("Mic Right", previous.mic_right, current.mic_right));
    }
    if current.joy_x.is_some() {
        rows.push(f64_row("Joy X", previous.joy_x, current.joy_x));
    }
    if current.joy_y.is_some() {
        rows.push(f64_row("Joy Y", previous.joy_y, current.joy_y));
    }
    if current.joy_z.is_some() {
        rows.push(f64_row("Joy Z", previous.joy_z, current.joy_z));
    }
    if current.cam_x.is_some() {
        rows.push(f64_row("Cam X", previous.cam_x, current.cam_x));
    }
    if current.cam_y.is_some() {
        rows.push(f64_row("Cam Y", previous.cam_y, current.cam_y));
    }
    if current.cam_z.is_some() {
        rows.push(f64_row("Cam Z", previous.cam_z, current.cam_z));
    }
    if current.slew.is_some() {
        rows.push(f64_row("Slew", previous.slew, current.slew));
    }
    if current.throttle.is_some() {
        rows.push(f64_row("Throttle", previous.throttle, current.throttle));
    }
    if current.throttle_l.is_some() {
        rows.push(f64_row("Throttle L", previous.throttle_l, current.throttle_l));
    }
    if current.throttle_r.is_some() {
        rows.push(f64_row("Throttle R", previous.throttle_r, current.throttle_r));
    }
    if current.volume.is_some() {
        rows.push(f64_row("Volume", previous.volume, current.volume));
    }
    if current.wheel.is_some() {
        rows.push(f64_row("Wheel", previous.wheel, current.wheel));
    }
    if current.rudder.is_some() {
        rows.push(f64_row("Rudder", previous.rudder, current.rudder));
    }
    if current.gas.is_some() {
        rows.push(f64_row("Gas", previous.gas, current.gas));
    }
    if current.brake.is_some() {
        rows.push(f64_row("Brake", previous.brake, current.brake));
    }
    if current.mic_push.is_some() {
        rows.push(bool_row("Mic Push", previous.mic_push, current.mic_push));
    }
    if current.trigger.is_some() {
        rows.push(bool_row("Trigger", previous.trigger, current.trigger));
    }
    if current.bumper.is_some() {
        rows.push(f64_row("Bumper", previous.bumper, current.bumper));
    }
    if current.action_m.is_some() {
        rows.push(bool_row("Action M", previous.action_m, current.action_m));
    }
    if current.action_l.is_some() {
        rows.push(bool_row("Action L", previous.action_l, current.action_l));
    }
    if current.action_r.is_some() {
        rows.push(bool_row("Action R", previous.action_r, current.action_r));
    }
    if current.pinky.is_some() {
        rows.push(bool_row("Pinky", previous.pinky, current.pinky));
    }
    if current.pinky_forward.is_some() {
        rows.push(bool_row("Pinky Forward", previous.pinky_forward, current.pinky_forward));
    }
    if current.pinky_backward.is_some() {
        rows.push(bool_row("Pinky Backward", previous.pinky_backward, current.pinky_backward));
    }
    if current.flaps_up.is_some() {
        rows.push(bool_row("Flaps Up", previous.flaps_up, current.flaps_up));
    }
    if current.flaps_down.is_some() {
        rows.push(bool_row("Flaps Down", previous.flaps_down, current.flaps_down));
    }
    if current.boat_forward.is_some() {
        rows.push(bool_row("Boat Forward", previous.boat_forward, current.boat_forward));
    }
    if current.boat_backward.is_some() {
        rows.push(bool_row("Boat Backward", previous.boat_backward, current.boat_backward));
    }
    if current.autopilot_path.is_some() {
        rows.push(bool_row("Autopilot Path", previous.autopilot_path, current.autopilot_path));
    }
    if current.autopilot_alt.is_some() {
        rows.push(bool_row("Autopilot Alt", previous.autopilot_alt, current.autopilot_alt));
    }
    if current.engine_motor_l.is_some() {
        rows.push(bool_row("Engine Motor L", previous.engine_motor_l, current.engine_motor_l));
    }
    if current.engine_motor_r.is_some() {
        rows.push(bool_row("Engine Motor R", previous.engine_motor_r, current.engine_motor_r));
    }
    if current.engine_fuel_flow_l.is_some() {
        rows.push(bool_row("Engine Fuel Flow L", previous.engine_fuel_flow_l, current.engine_fuel_flow_l));
    }
    if current.engine_fuel_flow_r.is_some() {
        rows.push(bool_row("Engine Fuel Flow R", previous.engine_fuel_flow_r, current.engine_fuel_flow_r));
    }
    if current.engine_ignite_l.is_some() {
        rows.push(bool_row("Engine Ignite L", previous.engine_ignite_l, current.engine_ignite_l));
    }
    if current.engine_ignite_r.is_some() {
        rows.push(bool_row("Engine Ignite R", previous.engine_ignite_r, current.engine_ignite_r));
    }
    if current.speedbrake_backward.is_some() {
        rows.push(bool_row("Speedbrake Backward", previous.speedbrake_backward, current.speedbrake_backward));
    }
    if current.speedbrake_forward.is_some() {
        rows.push(bool_row("Speedbrake Forward", previous.speedbrake_forward, current.speedbrake_forward));
    }
    if current.china_backward.is_some() {
        rows.push(bool_row("China Backward", previous.china_backward, current.china_backward));
    }
    if current.china_forward.is_some() {
        rows.push(bool_row("China Forward", previous.china_forward, current.china_forward));
    }
    if current.apu.is_some() {
        rows.push(bool_row("APU", previous.apu, current.apu));
    }
    if current.radar_altimeter.is_some() {
        rows.push(bool_row("Radar Altimeter", previous.radar_altimeter, current.radar_altimeter));
    }
    if current.landing_gear_silence.is_some() {
        rows.push(bool_row("Landing Gear Silence", previous.landing_gear_silence, current.landing_gear_silence));
    }
    if current.eac.is_some() {
        rows.push(bool_row("EAC", previous.eac, current.eac));
    }
    if current.autopilot_toggle.is_some() {
        rows.push(bool_row("Autopilot Toggle", previous.autopilot_toggle, current.autopilot_toggle));
    }
    if current.throttle_button.is_some() {
        rows.push(bool_row("Throttle Button", previous.throttle_button, current.throttle_button));
    }
    if current.mouse_x.is_some() {
        rows.push(f64_row("Mouse X", previous.mouse_x, current.mouse_x));
    }
    if current.mouse_y.is_some() {
        rows.push(f64_row("Mouse Y", previous.mouse_y, current.mouse_y));
    }
    if current.mouse.is_some() {
        rows.push(bool_row("Mouse", previous.mouse, current.mouse));
    }
    if current.paddle_left.is_some() {
        rows.push(bool_row("Paddle L", previous.paddle_left, current.paddle_left));
    }
    if current.paddle_right.is_some() {
        rows.push(bool_row("Paddle R", previous.paddle_right, current.paddle_right));
    }
    if current.pinky_left.is_some() {
        rows.push(bool_row("Pinky L", previous.pinky_left, current.pinky_left));
    }
    if current.pinky_right.is_some() {
        rows.push(bool_row("Pinky R", previous.pinky_right, current.pinky_right));
    }
    if current.context.is_some() {
        rows.push(bool_row("Context", previous.context, current.context));
    }
    if current.dpi.is_some() {
        rows.push(bool_row("DPI", previous.dpi, current.dpi));
    }
    if current.scroll_x.is_some() {
        rows.push(f64_row("Scroll X", previous.scroll_x, current.scroll_x));
    }
    if current.scroll_y.is_some() {
        rows.push(f64_row("Scroll Y", previous.scroll_y, current.scroll_y));
    }
    if current.scroll.is_some() {
        rows.push(bool_row("Scroll", previous.scroll, current.scroll));
    }
    if current.action_wheel_x.is_some() {
        rows.push(f64_row("Action Wheel X", previous.action_wheel_x, current.action_wheel_x));
    }
    if current.action_wheel_y.is_some() {
        rows.push(f64_row("Action Wheel Y", previous.action_wheel_y, current.action_wheel_y));
    }

    for (num, _) in current.number.iter() {
        rows.push(bool_row(&format!("Num#{}", num),
                           previous.number.get(num).cloned(),
                           current.number.get(num).cloned()));
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title(name.to_string()),
        )
        .widths(&[Constraint::Length(10), Constraint::Min(20), Constraint::Min(10)])
        .column_spacing(1)
}

fn f64_row(name: &str, prev: Option<f64>, curr: Option<f64>) -> Row {
    let style0 = Style::default().fg(Color::LightCyan);
    let style1 = Style::default().fg(Color::Gray);

    let mut rows = Vec::with_capacity(3);
    rows.push(Cell::from(Span::styled(name.to_string(), style0)));

    match (prev, curr) {
        (Some(prev), Some(curr)) => {
            let prev = (prev * 100.0) as i64;
            let curr = (curr * 100.0) as i64;
            let curr_str = format!("{:.2}", (curr as f64) / 100.0);
            match prev.cmp(&curr) {
                Ordering::Less => {
                    rows.push(Cell::from(Span::styled("+".to_string(), style0)));
                    rows.push(Cell::from(Span::styled(curr_str, style1)));
                }
                Ordering::Equal => {
                    rows.push(Cell::from(Span::styled("=".to_string(), style0)));
                    rows.push(Cell::from(Span::styled(curr_str, style1)));
                }
                Ordering::Greater => {
                    rows.push(Cell::from(Span::styled("-".to_string(), style0)));
                    rows.push(Cell::from(Span::styled(curr_str, style1)));
                }
            }
        }

        (None, Some(curr)) => {
            let curr = format!("{:.3}", curr);
            rows.push(Cell::from(Span::styled("#".to_string(), style0)));
            rows.push(Cell::from(Span::styled(curr, style1)));
        }

        (_, _) => {
            rows.push(Cell::from(Span::styled("?".to_string(), style0)));
            rows.push(Cell::from(Span::styled("?".to_string(), style1)));
        }
    }

    Row::new(rows)
}

fn bool_row<'a>(name: &str, prev: Option<bool>, curr: Option<bool>) -> Row<'a> {
    let style0 = Style::default().fg(Color::LightCyan);
    let style1 = Style::default().fg(Color::Gray);

    let mut rows = Vec::with_capacity(3);
    rows.push(Cell::from(Span::styled(name.to_string(), style0)));

    match (prev, curr) {
        (Some(prev), Some(curr)) => {
            match (prev, curr) {
                (true, true) => {
                    rows.push(Cell::from(Span::styled("=".to_string(), style0)));
                    rows.push(Cell::from(Span::styled("ON", style1)));
                }

                (false, true) => {
                    rows.push(Cell::from(Span::styled("+".to_string(), style0)));
                    rows.push(Cell::from(Span::styled("ON", style1)));
                }

                (true, false) => {
                    rows.push(Cell::from(Span::styled("-".to_string(), style0)));
                    rows.push(Cell::from(Span::styled("OFF", style1)));
                }

                (false, false) => {
                    rows.push(Cell::from(Span::styled("=".to_string(), style0)));
                    rows.push(Cell::from(Span::styled("OFF", style1)));
                }
            }
        }

        (None, Some(curr)) => {
            let curr_string = match curr {
                true => "ON".to_string(),
                false => "OFF".to_string(),
            };
            rows.push(Cell::from(Span::styled("#".to_string(), style0)));
            rows.push(Cell::from(Span::styled(curr_string, style1)));
        }

        (_, _) => {
            rows.push(Cell::from(Span::styled("?".to_string(), style0)));
            rows.push(Cell::from(Span::styled("?".to_string(), style1)));
        }
    }

    Row::new(rows)
}


fn draw_log<'a>(app_tui: &AppTui) -> Option<TuiLoggerSmartWidget<'a>> {
    Some(TuiLoggerSmartWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Magenta))
        .style_info(Style::default().fg(Color::Cyan))
        .output_separator(':')
        .output_timestamp(Some("%H:%M:%S".to_string()))
        .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
        .output_target(true)
        .output_file(true)
        .output_line(true)
        .state(&app_tui.tui))
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Driver1 Ctl")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}


#[allow(dead_code)]
fn styled_paragraph<'a, T>(text: T) -> Paragraph<'a>
    where T:
    Into<Text<'a>>,
{
    Paragraph::new(text)
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, got: {}", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, got: {}", rect.height);
    }
}
