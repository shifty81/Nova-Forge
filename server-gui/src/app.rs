use crate::{
    gui_log::{LogLevel, SharedLog},
    server_thread::{ServerCmd, ServerEvent},
};
use egui::{Color32, RichText, ScrollArea, TextEdit};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};
use tokio::sync::mpsc;

/// How often the GUI requests a repaint even when idle (for log streaming).
const REPAINT_INTERVAL: Duration = Duration::from_millis(250);

/// State for the "Shutdown" dialog.
#[derive(Default)]
struct ShutdownDialog {
    open: bool,
    seconds: String,
    reason: String,
}

/// State for the "Broadcast message" dialog.
#[derive(Default)]
struct BroadcastDialog {
    open: bool,
    message: String,
}

/// State for the "Add admin" dialog.
#[derive(Default)]
struct AdminDialog {
    open: bool,
    username: String,
    role: String,
}

/// The top-level eframe application.
pub struct ServerApp {
    // ── server communication ──────────────────────────────────────────────
    cmd_tx: mpsc::Sender<ServerCmd>,
    event_rx: mpsc::Receiver<ServerEvent>,
    stop_flag: Arc<AtomicBool>,

    // ── state ─────────────────────────────────────────────────────────────
    log: SharedLog,
    players: Vec<String>,
    server_running: bool,
    start_time: Instant,

    // ── command-bar state ─────────────────────────────────────────────────
    command_input: String,

    // ── dialog state ──────────────────────────────────────────────────────
    shutdown_dialog: ShutdownDialog,
    broadcast_dialog: BroadcastDialog,
    admin_add_dialog: AdminDialog,

    // ── log filter ────────────────────────────────────────────────────────
    log_filter: String,
    show_trace: bool,
    show_debug: bool,
    scroll_to_bottom: bool,
}

impl ServerApp {
    pub fn new(
        cmd_tx: mpsc::Sender<ServerCmd>,
        event_rx: mpsc::Receiver<ServerEvent>,
        stop_flag: Arc<AtomicBool>,
        log: SharedLog,
    ) -> Self {
        Self {
            cmd_tx,
            event_rx,
            stop_flag,
            log,
            players: Vec::new(),
            server_running: true,
            start_time: Instant::now(),
            command_input: String::new(),
            shutdown_dialog: ShutdownDialog::default(),
            broadcast_dialog: BroadcastDialog::default(),
            admin_add_dialog: AdminDialog::default(),
            log_filter: String::new(),
            show_trace: false,
            show_debug: true,
            scroll_to_bottom: true,
        }
    }

    // ── helpers ───────────────────────────────────────────────────────────

    fn send_cmd(&self, cmd: ServerCmd) {
        let _ = self.cmd_tx.try_send(cmd);
    }

    fn formatted_uptime(&self) -> String {
        let secs = self.start_time.elapsed().as_secs();
        let h = secs / 3600;
        let m = (secs % 3600) / 60;
        let s = secs % 60;
        format!("{h:02}:{m:02}:{s:02}")
    }

    fn level_color(level: LogLevel) -> Color32 {
        match level {
            LogLevel::Error => Color32::from_rgb(255, 80, 80),
            LogLevel::Warn => Color32::from_rgb(255, 200, 60),
            LogLevel::Info => Color32::from_rgb(160, 210, 255),
            LogLevel::Debug => Color32::from_rgb(140, 200, 140),
            LogLevel::Trace => Color32::from_rgb(160, 160, 160),
            LogLevel::Unknown => Color32::LIGHT_GRAY,
        }
    }

    fn should_show_level(&self, level: LogLevel) -> bool {
        match level {
            LogLevel::Trace => self.show_trace,
            LogLevel::Debug => self.show_debug,
            _ => true,
        }
    }

    // ── sub-panels ────────────────────────────────────────────────────────

    fn draw_header(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Status indicator
            let (status_color, status_label) = if self.server_running {
                (Color32::from_rgb(80, 200, 80), "● RUNNING")
            } else {
                (Color32::from_rgb(200, 80, 80), "● STOPPED")
            };
            ui.colored_label(status_color, status_label);
            ui.separator();
            ui.label(format!("Uptime: {}", self.formatted_uptime()));
            ui.separator();
            ui.label(format!("Players: {}", self.players.len()));
        });
    }

    fn draw_controls(&mut self, ui: &mut egui::Ui) {
        ui.heading("Server Controls");
        ui.add_space(6.0);

        // ── Shutdown section ──────────────────────────────────────────────
        ui.group(|ui| {
            ui.label(RichText::new("Shutdown").strong());
            ui.add_space(4.0);

            if ui
                .add_enabled(self.server_running, egui::Button::new("⏻  Stop Now"))
                .clicked()
            {
                self.send_cmd(ServerCmd::ShutdownImmediate);
            }

            if ui
                .add_enabled(self.server_running, egui::Button::new("⏱  Graceful Shutdown…"))
                .clicked()
            {
                self.shutdown_dialog.open = true;
                self.shutdown_dialog.seconds = "60".into();
                self.shutdown_dialog.reason =
                    "The server is shutting down".into();
            }
        });

        ui.add_space(6.0);

        // ── Players section ───────────────────────────────────────────────
        ui.group(|ui| {
            ui.label(RichText::new("Players").strong());
            ui.add_space(4.0);
            if ui
                .add_enabled(self.server_running, egui::Button::new("📢  Broadcast Message…"))
                .clicked()
            {
                self.broadcast_dialog.open = true;
                self.broadcast_dialog.message.clear();
            }

            if ui
                .add_enabled(self.server_running, egui::Button::new("⛔  Disconnect All"))
                .clicked()
            {
                self.send_cmd(ServerCmd::DisconnectAll);
            }
        });

        ui.add_space(6.0);

        // ── Admin section ─────────────────────────────────────────────────
        ui.group(|ui| {
            ui.label(RichText::new("Admin").strong());
            ui.add_space(4.0);
            if ui.button("➕  Add Admin…").clicked() {
                self.admin_add_dialog.open = true;
                self.admin_add_dialog.username.clear();
                self.admin_add_dialog.role = "moderator".into();
            }
        });

        ui.add_space(6.0);

        // ── Log filter controls ───────────────────────────────────────────
        ui.group(|ui| {
            ui.label(RichText::new("Log Filter").strong());
            ui.add_space(4.0);
            ui.add(
                TextEdit::singleline(&mut self.log_filter)
                    .hint_text("Filter…")
                    .desired_width(f32::INFINITY),
            );
            ui.checkbox(&mut self.show_debug, "Show DEBUG");
            ui.checkbox(&mut self.show_trace, "Show TRACE");
            ui.checkbox(&mut self.scroll_to_bottom, "Auto-scroll");
        });
    }

    fn draw_log_panel(&mut self, ui: &mut egui::Ui) {
        // Header row
        ui.horizontal(|ui| {
            ui.heading("Console Output");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.small_button("Clear").clicked() {
                    self.log.lock().unwrap().clear();
                }
            });
        });
        ui.separator();

        let filter = self.log_filter.to_lowercase();
        let scroll_to_bottom = self.scroll_to_bottom;

        let scroll = ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(scroll_to_bottom);

        scroll.show(ui, |ui| {
            let log = self.log.lock().unwrap();
            for entry in log.iter() {
                if !self.should_show_level(entry.level) {
                    continue;
                }
                if !filter.is_empty()
                    && !entry.text.to_lowercase().contains(&filter)
                {
                    continue;
                }
                let color = Self::level_color(entry.level);
                ui.colored_label(color, &entry.text);
            }
        });
    }

    fn draw_player_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Players");
        ui.separator();

        if self.players.is_empty() {
            ui.label(
                RichText::new("No players online")
                    .color(Color32::GRAY)
                    .italics(),
            );
            return;
        }

        // Collect kick actions separately to avoid holding a borrow on `self`
        // while calling `send_cmd`.
        let mut kick_player: Option<String> = None;

        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for name in &self.players {
                    ui.horizontal(|ui| {
                        ui.label(name);
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                if ui
                                    .add_enabled(
                                        self.server_running,
                                        egui::Button::new("Kick").small(),
                                    )
                                    .clicked()
                                {
                                    kick_player = Some(name.clone());
                                }
                            },
                        );
                    });
                    ui.separator();
                }
            });

        if let Some(name) = kick_player {
            self.send_cmd(ServerCmd::BroadcastMessage {
                msg: format!("{name} was kicked by the server"),
            });
        }
    }

    fn draw_command_bar(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.label(">");
            let response = ui.add(
                TextEdit::singleline(&mut self.command_input)
                    .hint_text("Type a server command…")
                    .desired_width(ui.available_width() - 60.0),
            );

            let enter_pressed = response.lost_focus()
                && ctx.input(|i| i.key_pressed(egui::Key::Enter));
            let send_clicked = ui
                .add_enabled(!self.command_input.is_empty(), egui::Button::new("Send"))
                .clicked();

            if (enter_pressed || send_clicked) && !self.command_input.is_empty() {
                self.handle_command_input();
                response.request_focus();
            }
        });
    }

    /// Parse and dispatch a raw command string typed in the command bar.
    fn handle_command_input(&mut self) {
        let input = self.command_input.trim().to_owned();
        self.command_input.clear();

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        match parts.first().copied().unwrap_or("") {
            "say" | "msg" => {
                let msg = parts.get(1).copied().unwrap_or("").to_owned();
                if !msg.is_empty() {
                    self.send_cmd(ServerCmd::BroadcastMessage { msg });
                }
            },
            "kick" => {
                // Broadcast a notice (same limitation as the player panel button).
                let name = parts.get(1).copied().unwrap_or("?");
                self.send_cmd(ServerCmd::BroadcastMessage {
                    msg: format!("{name} was kicked by the server"),
                });
            },
            "stop" | "quit" => {
                self.send_cmd(ServerCmd::ShutdownImmediate);
            },
            "shutdown" => {
                let secs: u64 = parts
                    .get(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60);
                self.send_cmd(ServerCmd::ShutdownGraceful {
                    seconds: secs,
                    reason: "Server is shutting down".into(),
                });
            },
            other => {
                tracing::warn!(
                    "Unknown GUI command: {other}. Try: say <msg>, kick <player>, stop, shutdown [secs]"
                );
            },
        }
    }

    // ── modal dialogs ─────────────────────────────────────────────────────

    fn draw_shutdown_dialog(&mut self, ctx: &egui::Context) {
        if !self.shutdown_dialog.open {
            return;
        }
        let mut open = self.shutdown_dialog.open;
        egui::Window::new("Graceful Shutdown")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label("Countdown (seconds):");
                ui.add(
                    TextEdit::singleline(&mut self.shutdown_dialog.seconds)
                        .desired_width(80.0),
                );
                ui.label("Reason:");
                ui.text_edit_singleline(&mut self.shutdown_dialog.reason);
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Confirm Shutdown").clicked() {
                        let seconds: u64 = self
                            .shutdown_dialog
                            .seconds
                            .trim()
                            .parse()
                            .unwrap_or(60);
                        let reason = self.shutdown_dialog.reason.clone();
                        self.send_cmd(ServerCmd::ShutdownGraceful { seconds, reason });
                        self.shutdown_dialog.open = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.shutdown_dialog.open = false;
                    }
                });
            });
        self.shutdown_dialog.open = open;
    }

    fn draw_broadcast_dialog(&mut self, ctx: &egui::Context) {
        if !self.broadcast_dialog.open {
            return;
        }
        let mut open = self.broadcast_dialog.open;
        egui::Window::new("Broadcast Message")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label("Message to send to all players:");
                ui.text_edit_multiline(&mut self.broadcast_dialog.message);
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    let can_send = !self.broadcast_dialog.message.trim().is_empty();
                    if ui
                        .add_enabled(can_send, egui::Button::new("Send"))
                        .clicked()
                    {
                        self.send_cmd(ServerCmd::BroadcastMessage {
                            msg: self.broadcast_dialog.message.trim().to_owned(),
                        });
                        self.broadcast_dialog.open = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.broadcast_dialog.open = false;
                    }
                });
            });
        self.broadcast_dialog.open = open;
    }

    fn draw_admin_add_dialog(&mut self, ctx: &egui::Context) {
        if !self.admin_add_dialog.open {
            return;
        }
        let mut open = self.admin_add_dialog.open;
        egui::Window::new("Add Admin")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label("Username:");
                ui.text_edit_singleline(&mut self.admin_add_dialog.username);
                ui.label("Role (admin / moderator):");
                ui.text_edit_singleline(&mut self.admin_add_dialog.role);
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    let can_add = !self.admin_add_dialog.username.trim().is_empty();
                    if ui.add_enabled(can_add, egui::Button::new("Add")).clicked() {
                        use std::str::FromStr;
                        let role = common::comp::AdminRole::from_str(
                            &self.admin_add_dialog.role.to_lowercase(),
                        )
                        .unwrap_or(common::comp::AdminRole::Moderator);
                        self.send_cmd(ServerCmd::AdminAdd {
                            username: self.admin_add_dialog.username.trim().to_owned(),
                            role,
                        });
                        self.admin_add_dialog.open = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.admin_add_dialog.open = false;
                    }
                });
            });
        self.admin_add_dialog.open = open;
    }
}

impl eframe::App for ServerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Drain server events.
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                ServerEvent::Players(p) => self.players = p,
                ServerEvent::Stopped => {
                    self.server_running = false;
                },
            }
        }

        // Request periodic repaint so log stays live.
        ctx.request_repaint_after(REPAINT_INTERVAL);

        // ── Dialogs (drawn before panels so they appear on top) ───────────
        self.draw_shutdown_dialog(ctx);
        self.draw_broadcast_dialog(ctx);
        self.draw_admin_add_dialog(ctx);

        // ── Top header bar ────────────────────────────────────────────────
        egui::TopBottomPanel::top("header")
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(6.0))
            .show(ctx, |ui| {
                self.draw_header(ui);
            });

        // ── Bottom command bar ────────────────────────────────────────────
        egui::TopBottomPanel::bottom("command_bar")
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(6.0))
            .show(ctx, |ui| {
                self.draw_command_bar(ui, ctx);
            });

        // ── Left: server controls ─────────────────────────────────────────
        egui::SidePanel::left("controls")
            .resizable(true)
            .default_width(200.0)
            .min_width(160.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.draw_controls(ui);
                });
            });

        // ── Right: player list ────────────────────────────────────────────
        egui::SidePanel::right("players")
            .resizable(true)
            .default_width(180.0)
            .min_width(120.0)
            .show(ctx, |ui| {
                self.draw_player_panel(ui);
            });

        // ── Centre: log console ───────────────────────────────────────────
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_log_panel(ui);
        });

        // If server stopped, signal the stop flag so main exits cleanly.
        if !self.server_running {
            self.stop_flag.store(true, Ordering::Relaxed);
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // User closed the window — tell the server thread to stop.
        self.stop_flag.store(true, Ordering::Relaxed);
    }
}
