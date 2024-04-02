use std::time::{Duration, Instant};
use eframe::egui::{Context, ProgressBar, Ui};
use eframe::{egui, Frame};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native("Pomodoro", options,
                       Box::new(|_cc| {
                           Box::<App>::default()
                       }),
    )
}


#[derive(Default)]
struct App {
    state: AppState,
}


#[derive(Default)]
enum AppState {
    #[default]
    Idle,
    Running(RunningState),
}

impl App {
    fn show(&mut self, ui: &mut Ui) {
        ui.heading("Pomodoro");
        self.show_progress_bar(ui);
        self.state = match &self.state {
            AppState::Idle => {
                self.show_idle(ui)
            }
            AppState::Running(state) => {
                self.show_running(state, ui)
            }
        };
    }

    fn show_progress_bar(&self, ui: &mut Ui) {
        ui.add(ProgressBar::new(
            self.get_progress()
        ));
    }

    fn show_idle(&self, ui: &mut Ui) -> AppState {
        self.show_duration(ui, WORK_DURATION.as_secs());
        if ui.button("Start").clicked() {
            return AppState::Running(RunningState { start: Instant::now() });
        }
        AppState::Idle
    }

    fn show_duration(&self, ui: &mut Ui, duration: u64) {
        ui.label(format!("{}:{}", duration / 60, duration % 60));
    }

    fn show_running(&self, state: &RunningState, ui: &mut Ui) -> AppState {
        self.show_duration(ui, (Instant::now() - state.start).as_secs());
        AppState::Running(RunningState { start: state.start })
    }


    fn get_progress(&self) -> f32 {
        match &self.state {
            AppState::Idle => 0.,
            AppState::Running(state) => (Instant::now() - state.start).as_secs_f32() / WORK_DURATION.as_secs_f32()
        }
    }
}


#[derive(Default)]
struct IdleState {}


struct RunningState {
    start: Instant,
}


static WORK_DURATION: Duration = Duration::from_secs(60 * 25);

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(
            ctx, |ui| { self.show(ui) },
        );
    }
}


