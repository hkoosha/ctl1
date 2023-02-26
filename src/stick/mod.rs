use log::{debug, info, trace, warn};
use stick::{Controller, Event, Listener};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc::error::TrySendError;
use tokio::task::JoinHandle;

struct ControllersState {
    listener: Listener,
    joiners: Vec<JoinHandle<()>>,
    tx: Sender<Ctl1Event>,
}

impl ControllersState {
    pub fn new(tx: Sender<Ctl1Event>) -> Self {
        Self {
            listener: Listener::default(),
            joiners: Vec::with_capacity(2),
            tx,
        }
    }

    pub fn open() -> (Self, Receiver<Ctl1Event>) {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        (Self::new(tx), rx)
    }
}


#[allow(dead_code)]
#[derive(Default, Debug, Clone)]
pub struct CtlState {
    pub exit: Option<bool>,
    pub action_a: Option<bool>,
    pub action_b: Option<bool>,
    pub action_c: Option<bool>,
    pub action_h: Option<bool>,
    pub action_v: Option<bool>,
    pub action_d: Option<bool>,
    pub menu_l: Option<bool>,
    pub menu_r: Option<bool>,
    pub joy: Option<bool>,
    pub cam: Option<bool>,
    pub bumper_l: Option<bool>,
    pub bumper_r: Option<bool>,
    pub trigger_l: Option<f64>,
    pub trigger_r: Option<f64>,
    pub up: Option<bool>,
    pub down: Option<bool>,
    pub left: Option<bool>,
    pub right: Option<bool>,
    pub pov_up: Option<bool>,
    pub pov_down: Option<bool>,
    pub pov_left: Option<bool>,
    pub pov_right: Option<bool>,
    pub hat_up: Option<bool>,
    pub hat_down: Option<bool>,
    pub hat_left: Option<bool>,
    pub hat_right: Option<bool>,
    pub trim_up: Option<bool>,
    pub trim_down: Option<bool>,
    pub trim_left: Option<bool>,
    pub trim_right: Option<bool>,
    pub mic_up: Option<bool>,
    pub mic_down: Option<bool>,
    pub mic_left: Option<bool>,
    pub mic_right: Option<bool>,
    pub joy_x: Option<f64>,
    pub joy_y: Option<f64>,
    pub joy_z: Option<f64>,
    pub cam_x: Option<f64>,
    pub cam_y: Option<f64>,
    pub cam_z: Option<f64>,
    pub slew: Option<f64>,
    pub throttle: Option<f64>,
    pub throttle_l: Option<f64>,
    pub throttle_r: Option<f64>,
    pub volume: Option<f64>,
    pub wheel: Option<f64>,
    pub rudder: Option<f64>,
    pub gas: Option<f64>,
    pub brake: Option<f64>,
    pub mic_push: Option<bool>,
    pub trigger: Option<bool>,
    pub bumper: Option<f64>,
    pub action_m: Option<bool>,
    pub action_l: Option<bool>,
    pub action_r: Option<bool>,
    pub pinky: Option<bool>,
    pub pinky_forward: Option<bool>,
    pub pinky_backward: Option<bool>,
    pub flaps_up: Option<bool>,
    pub flaps_down: Option<bool>,
    pub boat_forward: Option<bool>,
    pub boat_backward: Option<bool>,
    pub autopilot_path: Option<bool>,
    pub autopilot_alt: Option<bool>,
    pub engine_motor_l: Option<bool>,
    pub engine_motor_r: Option<bool>,
    pub engine_fuel_flow_l: Option<bool>,
    pub engine_fuel_flow_r: Option<bool>,
    pub engine_ignite_l: Option<bool>,
    pub engine_ignite_r: Option<bool>,
    pub speedbrake_backward: Option<bool>,
    pub speedbrake_forward: Option<bool>,
    pub china_backward: Option<bool>,
    pub china_forward: Option<bool>,
    pub apu: Option<bool>,
    pub radar_altimeter: Option<bool>,
    pub landing_gear_silence: Option<bool>,
    pub eac: Option<bool>,
    pub autopilot_toggle: Option<bool>,
    pub throttle_button: Option<bool>,
    pub mouse_x: Option<f64>,
    pub mouse_y: Option<f64>,
    pub mouse: Option<bool>,
    pub number: Option<(i8, bool)>,
    pub paddle_left: Option<bool>,
    pub paddle_right: Option<bool>,
    pub pinky_left: Option<bool>,
    pub pinky_right: Option<bool>,
    pub context: Option<bool>,
    pub dpi: Option<bool>,
    pub scroll_x: Option<f64>,
    pub scroll_y: Option<f64>,
    pub scroll: Option<bool>,
    pub action_wheel_x: Option<f64>,
    pub action_wheel_y: Option<f64>,

    pub unknown_event: Option<Event>,
    pub disconnected: bool,
}

impl CtlState {
    pub fn updated(&self, event: Event) -> Self {
        let mut clone = self.clone();

        match event {
            Event::Disconnect => clone.disconnected = true,
            Event::Exit(v) => clone.exit = Some(v),
            Event::ActionA(v) => clone.action_a = Some(v),
            Event::ActionB(v) => clone.action_b = Some(v),
            Event::ActionC(v) => clone.action_c = Some(v),
            Event::ActionH(v) => clone.action_h = Some(v),
            Event::ActionV(v) => clone.action_v = Some(v),
            Event::ActionD(v) => clone.action_d = Some(v),
            Event::MenuL(v) => clone.menu_l = Some(v),
            Event::MenuR(v) => clone.menu_r = Some(v),
            Event::Joy(v) => clone.joy = Some(v),
            Event::Cam(v) => clone.cam = Some(v),
            Event::BumperL(v) => clone.bumper_l = Some(v),
            Event::BumperR(v) => clone.bumper_r = Some(v),
            Event::TriggerL(v) => clone.trigger_l = Some(v),
            Event::TriggerR(v) => clone.trigger_r = Some(v),
            Event::Up(v) => clone.up = Some(v),
            Event::Down(v) => clone.down = Some(v),
            Event::Left(v) => clone.left = Some(v),
            Event::Right(v) => clone.right = Some(v),
            Event::PovUp(v) => clone.pov_up = Some(v),
            Event::PovDown(v) => clone.pov_down = Some(v),
            Event::PovLeft(v) => clone.pov_left = Some(v),
            Event::PovRight(v) => clone.pov_right = Some(v),
            Event::HatUp(v) => clone.hat_up = Some(v),
            Event::HatDown(v) => clone.hat_down = Some(v),
            Event::HatLeft(v) => clone.hat_left = Some(v),
            Event::HatRight(v) => clone.hat_right = Some(v),
            Event::TrimUp(v) => clone.trim_up = Some(v),
            Event::TrimDown(v) => clone.trim_down = Some(v),
            Event::TrimLeft(v) => clone.trim_left = Some(v),
            Event::TrimRight(v) => clone.trim_right = Some(v),
            Event::MicUp(v) => clone.mic_up = Some(v),
            Event::MicDown(v) => clone.mic_down = Some(v),
            Event::MicLeft(v) => clone.mic_left = Some(v),
            Event::MicRight(v) => clone.mic_right = Some(v),
            Event::JoyX(v) => clone.joy_x = Some(v),
            Event::JoyY(v) => clone.joy_y = Some(v),
            Event::JoyZ(v) => clone.joy_z = Some(v),
            Event::CamX(v) => clone.cam_x = Some(v),
            Event::CamY(v) => clone.cam_y = Some(v),
            Event::CamZ(v) => clone.cam_z = Some(v),
            Event::Slew(v) => clone.slew = Some(v),
            Event::Throttle(v) => clone.throttle = Some(v),
            Event::ThrottleL(v) => clone.throttle_l = Some(v),
            Event::ThrottleR(v) => clone.throttle_r = Some(v),
            Event::Volume(v) => clone.volume = Some(v),
            Event::Wheel(v) => clone.wheel = Some(v),
            Event::Rudder(v) => clone.rudder = Some(v),
            Event::Gas(v) => clone.gas = Some(v),
            Event::Brake(v) => clone.brake = Some(v),
            Event::MicPush(v) => clone.mic_push = Some(v),
            Event::Trigger(v) => clone.trigger = Some(v),
            Event::Bumper(v) => clone.bumper_r = Some(v),
            Event::ActionM(v) => clone.action_m = Some(v),
            Event::ActionL(v) => clone.action_l = Some(v),
            Event::ActionR(v) => clone.action_r = Some(v),
            Event::Pinky(v) => clone.pinky = Some(v),
            Event::PinkyForward(v) => clone.pinky_forward = Some(v),
            Event::PinkyBackward(v) => clone.pinky_backward = Some(v),
            Event::FlapsUp(v) => clone.flaps_up = Some(v),
            Event::FlapsDown(v) => clone.flaps_down = Some(v),
            Event::BoatForward(v) => clone.boat_forward = Some(v),
            Event::BoatBackward(v) => clone.boat_backward = Some(v),
            Event::AutopilotPath(v) => clone.autopilot_path = Some(v),
            Event::AutopilotAlt(v) => clone.autopilot_path = Some(v),
            Event::EngineMotorL(v) => clone.engine_motor_l = Some(v),
            Event::EngineMotorR(v) => clone.engine_motor_r = Some(v),
            Event::EngineFuelFlowL(v) => clone.engine_fuel_flow_l = Some(v),
            Event::EngineFuelFlowR(v) => clone.engine_fuel_flow_r = Some(v),
            Event::EngineIgnitionL(v) => clone.engine_ignite_l = Some(v),
            Event::EngineIgnitionR(v) => clone.engine_ignite_r = Some(v),
            Event::SpeedbrakeBackward(v) => clone.speedbrake_backward = Some(v),
            Event::SpeedbrakeForward(v) => clone.speedbrake_forward = Some(v),
            Event::ChinaBackward(v) => clone.china_backward = Some(v),
            Event::ChinaForward(v) => clone.china_forward = Some(v),
            Event::Apu(v) => clone.apu = Some(v),
            Event::RadarAltimeter(v) => clone.radar_altimeter = Some(v),
            Event::LandingGearSilence(v) => clone.landing_gear_silence = Some(v),
            Event::Eac(v) => clone.eac = Some(v),
            Event::AutopilotToggle(v) => clone.autopilot_toggle = Some(v),
            Event::ThrottleButton(v) => clone.throttle_button = Some(v),
            Event::MouseX(v) => clone.mouse_x = Some(v),
            Event::MouseY(v) => clone.mouse_y = Some(v),
            Event::Mouse(v) => clone.mouse = Some(v),
            Event::Number(v0, v1) => clone.number = Some((v0, v1)),
            Event::PaddleLeft(v) => clone.paddle_left = Some(v),
            Event::PaddleRight(v) => clone.paddle_right = Some(v),
            Event::PinkyLeft(v) => clone.pinky_left = Some(v),
            Event::PinkyRight(v) => clone.pinky_right = Some(v),
            Event::Context(v) => clone.context = Some(v),
            Event::Dpi(v) => clone.dpi = Some(v),
            Event::ScrollX(v) => clone.scroll_x = Some(v),
            Event::ScrollY(v) => clone.scroll_y = Some(v),
            Event::Scroll(v) => clone.scroll = Some(v),
            Event::ActionWheelX(v) => clone.action_wheel_x = Some(v),
            Event::ActionWheelY(v) => clone.action_wheel_y = Some(v),
            unknown => {
                warn!("Unknown event: {}", unknown);
                clone.unknown_event = Some(unknown)
            },
        }

        clone
    }
}


#[derive(Clone, Debug)]
pub struct Ctl1Event {
    pub triggering_event: Event,
    pub ctl_name: String,
    pub ctl_id: u64,
}

impl Ctl1Event {
    pub fn new(event: Event,
               ctl_name: String,
               ctl_id: u64, ) -> Self {
        Self {
            triggering_event: event,
            ctl_name,
            ctl_id,
        }
    }
}


async fn read_ctl(listener: &mut Listener,
                  tx: Sender<Ctl1Event>) -> JoinHandle<()> {
    trace!("waiting controller...");

    let mut controller: Controller = listener.await;
    info!("connected, id={:016X}, name={}",controller.id(),controller.name());

    tokio::spawn(async move {
        loop {
            let event = (&mut controller).await;
            let ctl_event = Ctl1Event::new(
                event,
                controller.name().to_string(),
                controller.id(),
            );

            if let Err(err) = tx.try_send(ctl_event) {
                match err {
                    TrySendError::Full(_) => {}
                    TrySendError::Closed(_) => {
                        debug!(
                            "receiver closed for id={:016X}, name={}",
                            controller.id(),
                            controller.name(),
                        );
                        return;
                    }
                }
            }

            if let Event::Disconnect = &event {
                debug!(
                    "controller disconnected id={:016X}, name={}",
                    controller.id(),
                    controller.name(),
                );
                return;
            }
        }
    })
}

pub fn read_ctls() -> (Receiver<Ctl1Event>, JoinHandle<()>) {
    let (mut state, rx) = ControllersState::open();

    let handle = tokio::spawn(async move {
        loop {
            let joiner = read_ctl(&mut state.listener, state.tx.clone()).await;
            state.joiners.push(joiner)
        }
    });

    (rx, handle)
}
