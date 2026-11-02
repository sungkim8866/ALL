//! The window (egui). Korean UI.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use eframe::egui::{self, Color32, RichText};

use crate::config::Settings;
use crate::engine::{Command, ConnStatus, EngineHandle, PairPrompt, TransferState, UiState};
use crate::geometry::Side;
use crate::platform::{InputStatus, Platform};
use crate::proto::Os;

const ACCENT: Color32 = Color32::from_rgb(0x3B, 0x82, 0xF6);
const GOOD: Color32 = Color32::from_rgb(0x22, 0xA0, 0x5B);
const WARN_BG: Color32 = Color32::from_rgb(0xFF, 0xF4, 0xD6);
const WARN_FG: Color32 = Color32::from_rgb(0x7A, 0x52, 0x00);

pub fn run(handle: EngineHandle, platform: Arc<dyn Platform>) -> eframe::Result {
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png")).ok();
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("DeskLink")
        .with_app_id("DeskLink")
        .with_inner_size([520.0, 760.0])
        .with_min_inner_size([420.0, 520.0])
        .with_drag_and_drop(true);
    if let Some(icon) = icon {
        viewport = viewport.with_icon(icon);
    }
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "DeskLink",
        options,
        Box::new(move |cc| {
            install_fonts(&cc.egui_ctx);
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            let ctx = cc.egui_ctx.clone();
            handle.shared.set_repaint(move || ctx.request_repaint());
            Ok(Box::new(App::new(handle, platform)))
        }),
    )
}

/// egui's built-in fonts have no Hangul; borrow the system's Korean font.
fn install_fonts(ctx: &egui::Context) {
    let candidates: &[(&str, u32)] = if cfg!(target_os = "macos") {
        &[
            ("/System/Library/Fonts/AppleSDGothicNeo.ttc", 0),
            ("/System/Library/Fonts/Supplemental/AppleGothic.ttf", 0),
            ("/Library/Fonts/Arial Unicode.ttf", 0),
        ]
    } else if cfg!(target_os = "windows") {
        &[
            ("C:\\Windows\\Fonts\\malgun.ttf", 0),
            ("C:\\Windows\\Fonts\\gulim.ttc", 0),
        ]
    } else {
        &[
            ("/usr/share/fonts/truetype/nanum/NanumGothic.ttf", 0),
            ("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc", 1),
            ("/usr/share/fonts/opentype/unifont/unifont.otf", 0),
        ]
    };
    let mut fonts = egui::FontDefinitions::default();
    for (path, index) in candidates {
        if let Ok(bytes) = std::fs::read(path) {
            let mut data = egui::FontData::from_owned(bytes);
            data.index = *index;
            fonts.font_data.insert("korean".into(), Arc::new(data));
            for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
                fonts.families.entry(family).or_default().push("korean".into());
            }
            break;
        }
    }
    ctx.set_fonts(fonts);
}

struct App {
    handle: EngineHandle,
    platform: Arc<dyn Platform>,
    state: UiState,
    edit: Settings,
    manual_addr: String,
    code_input: String,
    focus_code: bool,
    last_prompt: Option<u64>,
    quitting: bool,
}

impl App {
    fn new(handle: EngineHandle, platform: Arc<dyn Platform>) -> App {
        let state = handle.shared.snapshot();
        let edit = state.settings.clone();
        App {
            handle,
            platform,
            state,
            edit,
            manual_addr: String::new(),
            code_input: String::new(),
            focus_code: false,
            last_prompt: None,
            quitting: false,
        }
    }

    fn send(&self, c: Command) {
        self.handle.send(c);
    }

    fn apply_settings(&mut self) {
        if self.edit != self.state.settings {
            self.send(Command::UpdateSettings(self.edit.clone()));
            self.state.settings = self.edit.clone();
        }
    }
}

fn side_word(side: Side) -> &'static str {
    match side {
        Side::Left => "왼쪽",
        Side::Right => "오른쪽",
        Side::Top => "위쪽",
        Side::Bottom => "아래쪽",
    }
}

fn os_name(os: Os) -> &'static str {
    match os {
        Os::Mac => "Mac",
        Os::Windows => "Windows",
        Os::Linux => "Linux",
        Os::Other => "",
    }
}

/// "이름 · Mac" style label.
fn device_label(name: &str, os: Os) -> RichText {
    RichText::new(format!("{name}  ·  {}", os_name(os)))
}

fn human_size(n: u64) -> String {
    let f = n as f64;
    if f >= 1e9 {
        format!("{:.1} GB", f / 1e9)
    } else if f >= 1e6 {
        format!("{:.1} MB", f / 1e6)
    } else if f >= 1e3 {
        format!("{:.0} KB", f / 1e3)
    } else {
        format!("{n} B")
    }
}

fn open_path(path: &std::path::Path) {
    let _ = std::fs::create_dir_all(path);
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(path).spawn();
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("explorer").arg(path).spawn();
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let _ = std::process::Command::new("xdg-open").arg(path).spawn();
}

fn open_accessibility_settings() {
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .spawn();
}

fn pick_files() -> Option<Vec<PathBuf>> {
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        rfd::FileDialog::new().set_title("보낼 파일 선택").pick_files()
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        None
    }
}

fn pick_folder() -> Option<PathBuf> {
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        rfd::FileDialog::new().set_title("받은 파일을 저장할 폴더").pick_folder()
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        None
    }
}

fn card<R>(ui: &mut egui::Ui, title: &str, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::new()
        .fill(Color32::WHITE)
        .stroke(egui::Stroke::new(1.0, Color32::from_gray(225)))
        .corner_radius(10.0)
        .inner_margin(14.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(title).size(15.0).strong());
            ui.add_space(6.0);
            add(ui)
        })
        .inner
}

fn banner(ui: &mut egui::Ui, text: &str, add: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(WARN_BG)
        .corner_radius(8.0)
        .inner_margin(10.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(text).color(WARN_FG));
            ui.horizontal_wrapped(add);
        });
    ui.add_space(8.0);
}

impl eframe::App for App {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.state = self.handle.shared.snapshot();
        if self.state.quit && !self.quitting {
            self.quitting = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        // A pairing request brings the window to the front.
        let prompt = self.state.pairing.as_ref().map(|p| p.conn());
        if prompt.is_some() && prompt != self.last_prompt {
            self.code_input.clear();
            self.focus_code = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        }
        self.last_prompt = prompt;

        // Closing the window keeps DeskLink running (minimized).
        if ctx.input(|i| i.viewport().close_requested()) && !self.quitting && self.state.settings.keep_running_on_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }

        // Files dropped on the window are sent.
        let dropped: Vec<PathBuf> = ctx.input(|i| i.raw.dropped_files.iter().map(|f| f.path().to_path_buf()).collect());
        if !dropped.is_empty() {
            self.send(Command::SendFiles(dropped));
        }
        ctx.request_repaint_after(Duration::from_millis(500));
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(Color32::from_gray(245)).inner_margin(14.0))
            .show(ui, |ui| {
                egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                    self.header(ui);
                    self.banners(ui);
                    self.connection(ui);
                    ui.add_space(10.0);
                    self.layout(ui);
                    ui.add_space(10.0);
                    self.files(ui, &ctx);
                    ui.add_space(10.0);
                    self.settings(ui);
                    ui.add_space(10.0);
                    self.footer(ui);
                });
            });
        self.pairing_dialog(&ctx);
        self.apply_settings();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.send(Command::Shutdown);
        self.platform.shutdown();
    }
}

impl App {
    fn header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("DeskLink").size(26.0).strong().color(ACCENT));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let (text, color) = match &self.state.conn {
                    ConnStatus::Connected { .. } => ("● 연결됨", GOOD),
                    ConnStatus::Connecting(_) => ("● 연결 중…", Color32::from_rgb(0xD9, 0x8E, 0x04)),
                    ConnStatus::Idle => ("● 연결 안 됨", Color32::GRAY),
                };
                ui.label(RichText::new(text).color(color).strong());
            });
        });
        ui.label(
            RichText::new(format!(
                "{} ({}) · 마우스·키보드·클립보드·파일을 함께 써요",
                self.state.settings.device_name,
                os_name(self.state.os)
            ))
            .color(Color32::from_gray(110)),
        );
        ui.add_space(10.0);
    }

    fn banners(&mut self, ui: &mut egui::Ui) {
        match &self.state.input {
            InputStatus::NeedsPermission => {
                let platform = self.platform.clone();
                banner(
                    ui,
                    "마우스·키보드를 공유하려면 '손쉬운 사용' 권한이 필요합니다. 시스템 설정 › 개인정보 보호 및 보안 › 손쉬운 사용에서 DeskLink를 켜 주세요. (켠 뒤에도 안 되면 DeskLink를 다시 실행하세요)",
                    |ui| {
                        if ui.button("권한 요청").clicked() {
                            platform.request_permissions();
                        }
                        if ui.button("시스템 설정 열기").clicked() {
                            open_accessibility_settings();
                        }
                    },
                );
            }
            InputStatus::Failed(e) => banner(ui, &format!("입력 공유를 시작하지 못했습니다: {e}"), |_| {}),
            InputStatus::Unsupported => banner(ui, "이 운영체제에서는 마우스·키보드 공유를 지원하지 않습니다 (클립보드·파일만 가능).", |_| {}),
            InputStatus::Ready => {}
        }
        if let Some(e) = self.state.listen_error.clone() {
            banner(ui, &e, |_| {});
        }
        if let Some(n) = self.state.notice.clone() {
            let h = self.handle.clone();
            banner(ui, &n, |ui| {
                if ui.small_button("닫기").clicked() {
                    h.shared.update(|s| s.notice = None);
                }
            });
        }
    }

    fn connection(&mut self, ui: &mut egui::Ui) {
        let state = self.state.clone();
        card(ui, "연결", |ui| {
            if let ConnStatus::Connected { name, os, addr, .. } = &state.conn {
                ui.horizontal(|ui| {
                    ui.label(device_label(name, *os).size(16.0).strong());
                    ui.label(RichText::new(addr).color(Color32::GRAY));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("연결 끊기").clicked() {
                            self.send(Command::Disconnect);
                        }
                    });
                });
                if state.controlling {
                    ui.label(RichText::new(format!("지금 {name}을(를) 조종하고 있어요. 양쪽 Shift를 함께 누르면 바로 돌아옵니다.")).color(ACCENT));
                } else if state.controlled {
                    ui.label(RichText::new(format!("{name}이(가) 이 컴퓨터를 조종하고 있어요.")).color(ACCENT));
                } else if state.settings.share_input {
                    ui.label(format!(
                        "마우스를 화면 {} 끝으로 밀면 {name}(으)로 넘어갑니다.",
                        side_word(state.settings.peer_side)
                    ));
                }
                ui.add_space(6.0);
                ui.separator();
            } else if let ConnStatus::Connecting(a) = &state.conn {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(format!("{a}에 연결 중…"));
                });
                ui.separator();
            }

            ui.label(RichText::new("같은 네트워크의 컴퓨터").strong());
            let others: Vec<_> = state
                .nearby
                .iter()
                .filter(|n| match &state.conn {
                    ConnStatus::Connected { id, .. } => id != &n.id,
                    _ => true,
                })
                .cloned()
                .collect();
            if others.is_empty() {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(
                        RichText::new("다른 컴퓨터에서 DeskLink를 실행하면 여기에 나타나요")
                            .color(Color32::GRAY),
                    );
                });
            }
            for n in &others {
                ui.horizontal(|ui| {
                    ui.label(device_label(&n.name, n.os));
                    ui.label(RichText::new(&n.addr).color(Color32::GRAY));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if n.paired {
                            if ui.button("연결").clicked() {
                                self.send(Command::ConnectPeer(n.id.clone()));
                            }
                        } else if ui
                            .add(egui::Button::new(RichText::new("페어링").color(Color32::WHITE)).fill(ACCENT))
                            .clicked()
                        {
                            self.send(Command::ConnectPeer(n.id.clone()));
                        }
                    });
                });
            }

            ui.add_space(6.0);
            ui.collapsing("IP 주소로 직접 연결", |ui| {
                ui.label(
                    RichText::new("목록에 안 보이면 상대 컴퓨터 DeskLink 아래쪽에 표시된 IP를 입력하세요.")
                        .color(Color32::GRAY),
                );
                ui.horizontal(|ui| {
                    let r = ui.add(egui::TextEdit::singleline(&mut self.manual_addr).hint_text("예: 192.168.0.12").desired_width(200.0));
                    let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                    if (ui.button("연결").clicked() || enter) && !self.manual_addr.trim().is_empty() {
                        self.send(Command::ConnectAddr(self.manual_addr.trim().to_string()));
                    }
                });
            });

            if !state.peers.is_empty() {
                ui.collapsing(format!("페어링된 컴퓨터 ({})", state.peers.len()), |ui| {
                    for p in &state.peers {
                        ui.horizontal(|ui| {
                            ui.label(device_label(&p.name, p.os));
                            ui.label(
                                RichText::new(format!("보안 코드 {}", crate::transport::fingerprint(&p.public_key)))
                                    .small()
                                    .color(Color32::GRAY),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button("삭제").clicked() {
                                    self.send(Command::Forget(p.id.clone()));
                                }
                            });
                        });
                    }
                });
            }
        });
    }

    fn layout(&mut self, ui: &mut egui::Ui) {
        let peer_name = match &self.state.conn {
            ConnStatus::Connected { name, .. } => name.clone(),
            _ => "상대 컴퓨터".to_string(),
        };
        card(ui, "화면 배치", |ui| {
            ui.label(
                RichText::new("상대 컴퓨터가 책상 위 어느 쪽에 있나요? 칸을 눌러 고르세요.").color(Color32::GRAY),
            );
            ui.add_space(4.0);
            let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 170.0), egui::Sense::hover());
            let me = egui::Rect::from_center_size(rect.center(), egui::vec2(120.0, 70.0));
            let slot = |side: Side| -> egui::Rect {
                let (w, h) = (100.0, 52.0);
                let c = me.center();
                match side {
                    Side::Left => egui::Rect::from_center_size(egui::pos2(me.left() - 8.0 - w / 2.0, c.y), egui::vec2(w, h)),
                    Side::Right => egui::Rect::from_center_size(egui::pos2(me.right() + 8.0 + w / 2.0, c.y), egui::vec2(w, h)),
                    Side::Top => egui::Rect::from_center_size(egui::pos2(c.x, me.top() - 6.0 - 20.0), egui::vec2(w, 40.0)),
                    Side::Bottom => egui::Rect::from_center_size(egui::pos2(c.x, me.bottom() + 6.0 + 20.0), egui::vec2(w, 40.0)),
                }
            };
            let painter = ui.painter().clone();
            painter.rect_filled(me, 8.0, Color32::from_rgb(0xE8, 0xF0, 0xFE));
            painter.rect_stroke(me, 8.0, egui::Stroke::new(1.5, ACCENT), egui::StrokeKind::Inside);
            painter.text(
                me.center(),
                egui::Align2::CENTER_CENTER,
                "이 컴퓨터",
                egui::FontId::proportional(14.0),
                Color32::from_gray(40),
            );
            for side in [Side::Left, Side::Right, Side::Top, Side::Bottom] {
                let r = slot(side);
                let resp = ui.interact(r, egui::Id::unique(("desklink-layout-slot", side as u8)), egui::Sense::click());
                let selected = self.edit.peer_side == side;
                let (fill, stroke, text_color) = if selected {
                    (ACCENT, ACCENT, Color32::WHITE)
                } else if resp.hovered() {
                    (Color32::from_gray(235), Color32::from_gray(170), Color32::from_gray(80))
                } else {
                    (Color32::from_gray(250), Color32::from_gray(210), Color32::from_gray(150))
                };
                painter.rect_filled(r, 8.0, fill);
                painter.rect_stroke(r, 8.0, egui::Stroke::new(1.0, stroke), egui::StrokeKind::Inside);
                let label = if selected { peer_name.as_str() } else { "여기?" };
                painter.text(r.center(), egui::Align2::CENTER_CENTER, label, egui::FontId::proportional(13.0), text_color);
                if resp.clicked() {
                    self.edit.peer_side = side;
                }
            }
            ui.label(format!(
                "{}이(가) 이 컴퓨터의 {}에 있어요. 상대 컴퓨터에서는 반대쪽({})을 고르세요.",
                peer_name,
                side_word(self.edit.peer_side),
                side_word(self.edit.peer_side.opposite())
            ));
        });
    }

    fn files(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let connected = matches!(self.state.conn, ConnStatus::Connected { .. });
        let hovering = ctx.input(|i| !i.raw.hovered_files.is_empty());
        let transfers = self.state.transfers.clone();
        let download_dir = self.state.download_dir.clone();
        card(ui, "파일 보내기", |ui| {
            let fill = if hovering { Color32::from_rgb(0xE8, 0xF0, 0xFE) } else { Color32::from_gray(250) };
            egui::Frame::new()
                .fill(fill)
                .stroke(egui::Stroke::new(1.0, if hovering { ACCENT } else { Color32::from_gray(210) }))
                .corner_radius(8.0)
                .inner_margin(14.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.vertical_centered(|ui| {
                        ui.label(if connected {
                            "여기로 파일을 끌어다 놓으면 상대 컴퓨터로 보냅니다"
                        } else {
                            "연결되면 파일을 보낼 수 있어요"
                        });
                        if ui.add_enabled(connected, egui::Button::new("파일 선택…")).clicked() {
                            if let Some(files) = pick_files() {
                                self.send(Command::SendFiles(files));
                            }
                        }
                    });
                });
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("받은 파일: {}", download_dir.display())).small().color(Color32::GRAY));
                if ui.small_button("열기").clicked() {
                    open_path(&download_dir);
                }
            });
            if !transfers.is_empty() {
                ui.add_space(4.0);
                for t in transfers.iter().rev().take(8) {
                    ui.horizontal(|ui| {
                        ui.label(if t.outgoing { "↑" } else { "↓" });
                        ui.label(&t.name);
                        match &t.state {
                            TransferState::Active => {
                                let f = if t.size > 0 { t.done as f32 / t.size as f32 } else { 0.0 };
                                ui.add(egui::ProgressBar::new(f).desired_width(120.0).show_percentage());
                            }
                            TransferState::Done => {
                                ui.label(RichText::new(format!("완료 · {}", human_size(t.size))).color(GOOD));
                                if let (false, Some(p)) = (t.outgoing, &t.path) {
                                    if ui.small_button("폴더").clicked() {
                                        open_path(p.parent().unwrap_or(p));
                                    }
                                }
                            }
                            TransferState::Failed(e) => {
                                ui.label(RichText::new(format!("실패: {e}")).color(Color32::from_rgb(0xC0, 0x30, 0x30)));
                            }
                        }
                    });
                }
                if transfers.iter().any(|t| t.state != TransferState::Active) && ui.small_button("목록 지우기").clicked() {
                    self.send(Command::ClearFinishedTransfers);
                }
            }
        });
    }

    fn settings(&mut self, ui: &mut egui::Ui) {
        let os = self.state.os;
        card(ui, "설정", |ui| {
            ui.checkbox(&mut self.edit.share_input, "마우스·키보드 공유 (화면 끝으로 넘어가기)");
            ui.checkbox(&mut self.edit.share_clipboard, "클립보드 공유 (글자·이미지 복사/붙여넣기)");
            ui.checkbox(
                &mut self.edit.swap_ctrl_cmd,
                "Ctrl ↔ Cmd 바꾸기 (맥↔윈도우 단축키를 같은 손가락으로: Cmd+C = Ctrl+C)",
            );
            ui.checkbox(
                &mut self.edit.caps_lock_as_hangul,
                if os == Os::Mac {
                    "Caps Lock을 상대 컴퓨터의 한/영 키로 쓰기"
                } else {
                    "Caps Lock을 상대 컴퓨터의 한/영 키로 쓰기 (한/영 키는 항상 전달돼요)"
                },
            );
            ui.horizontal(|ui| {
                ui.label("넘어가기 전 멈춤");
                ui.add(egui::Slider::new(&mut self.edit.switch_delay_ms, 0..=500).suffix(" ms"));
            });
            ui.label(
                RichText::new("0이면 화면 끝에 닿자마자 넘어가요. 실수로 자주 넘어가면 값을 올리세요.")
                    .small()
                    .color(Color32::GRAY),
            );
            ui.horizontal(|ui| {
                ui.label("상대 화면에서 포인터 속도");
                ui.add(egui::Slider::new(&mut self.edit.pointer_speed, 0.3..=3.0).fixed_decimals(1).suffix("×"));
            });
            ui.checkbox(&mut self.edit.keep_running_on_close, "창을 닫아도 계속 실행 (최소화)");
            ui.horizontal(|ui| {
                ui.label("이 컴퓨터 이름");
                let r = ui.add(egui::TextEdit::singleline(&mut self.edit.device_name).desired_width(180.0));
                if r.changed() && self.edit.device_name.chars().count() > 40 {
                    self.edit.device_name = self.edit.device_name.chars().take(40).collect();
                }
            });
            ui.horizontal(|ui| {
                ui.label("받은 파일 저장 위치");
                if ui.button("변경…").clicked() {
                    if let Some(dir) = pick_folder() {
                        self.edit.download_dir = Some(dir);
                    }
                }
                if self.edit.download_dir.is_some() && ui.small_button("기본값").clicked() {
                    self.edit.download_dir = None;
                }
            });
        });
    }

    fn footer(&mut self, ui: &mut egui::Ui) {
        let ips: Vec<String> = if_addrs::get_if_addrs()
            .map(|v| {
                v.into_iter()
                    .filter(|i| !i.is_loopback() && i.ip().is_ipv4())
                    .map(|i| i.ip().to_string())
                    .collect()
            })
            .unwrap_or_default();
        ui.collapsing("기록", |ui| {
            egui::ScrollArea::vertical().max_height(160.0).stick_to_bottom(true).show(ui, |ui| {
                for line in &self.state.log {
                    ui.label(RichText::new(line).small().monospace());
                }
            });
        });
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new(format!(
                    "IP {} · 포트 {} · 보안 코드 {} · v{}",
                    if ips.is_empty() { "-".to_string() } else { ips.join(", ") },
                    self.state.listen_port.map(|p| p.to_string()).unwrap_or("-".into()),
                    self.state.fingerprint,
                    env!("CARGO_PKG_VERSION")
                ))
                .small()
                .color(Color32::GRAY),
            );
        });
        ui.horizontal(|ui| {
            if ui.button("DeskLink 종료").clicked() {
                self.quitting = true;
                self.send(Command::Shutdown);
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }

    fn pairing_dialog(&mut self, ctx: &egui::Context) {
        let Some(prompt) = self.state.pairing.clone() else { return };
        egui::Modal::new(egui::Id::unique("desklink-pairing")).show(ctx, |ui| {
            ui.set_width(340.0);
            match prompt {
                PairPrompt::ShowCode { peer, code, .. } => {
                    ui.label(RichText::new("페어링 요청").size(18.0).strong());
                    ui.add_space(4.0);
                    ui.label(format!("{peer}에서 이 컴퓨터와 연결하려고 해요."));
                    ui.label("상대 컴퓨터에 아래 코드를 입력하세요.");
                    ui.add_space(10.0);
                    ui.vertical_centered(|ui| {
                        ui.label(
                            RichText::new(format!("{} {}", &code[..3], &code[3..]))
                                .size(40.0)
                                .strong()
                                .monospace()
                                .color(ACCENT),
                        );
                    });
                    ui.add_space(10.0);
                    ui.label(
                        RichText::new("모르는 컴퓨터라면 취소하세요.").small().color(Color32::GRAY),
                    );
                    if ui.button("취소").clicked() {
                        self.send(Command::CancelPairing);
                    }
                }
                PairPrompt::EnterCode { peer, .. } => {
                    ui.label(RichText::new("코드 입력").size(18.0).strong());
                    ui.add_space(4.0);
                    ui.label(format!("{peer} 화면에 표시된 6자리 코드를 입력하세요."));
                    ui.add_space(8.0);
                    let r = ui.add(
                        egui::TextEdit::singleline(&mut self.code_input)
                            .hint_text("123 456")
                            .font(egui::FontId::monospace(28.0))
                            .desired_width(200.0),
                    );
                    if self.focus_code {
                        r.request_focus();
                        self.focus_code = false;
                    }
                    let digits = crate::transport::normalize_code(&self.code_input);
                    let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        let ok = ui.add_enabled(digits.len() == 6, egui::Button::new(RichText::new("확인").color(Color32::WHITE)).fill(ACCENT));
                        if (ok.clicked() || enter) && digits.len() == 6 {
                            self.send(Command::SubmitCode(digits.clone()));
                            self.code_input.clear();
                        }
                        if ui.button("취소").clicked() {
                            self.send(Command::CancelPairing);
                        }
                    });
                }
            }
        });
    }
}
