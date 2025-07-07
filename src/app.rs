use std::arch::aarch64::vpadd_f32;
use std::f32::consts::TAU;
use eframe::epaint::{vec2, Color32, Pos2, Rect, Stroke, StrokeKind};
use eframe::glow::RED;
use egui::epaint::tessellator::path::rounded_rectangle;
use egui::{CornerRadius, Sense, Shape, Ui, Vec2};
use egui::epaint::RectShape;
use egui::UiKind::ScrollArea;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default, Debug,Clone)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Sector (String);

#[derive(serde::Deserialize, serde::Serialize,Default, Debug,Clone)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Device {
    name: String,
    frequency_min: f32,
    frequency_max: f32,
    remove:bool,
    #[serde(default)]
    sectors: Vec<Sector>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    inProgressDevice : Option<Device>,
    warn: bool,
    devices : Vec<Device>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            inProgressDevice : None,
            warn: false,
            devices: Vec::new(),
            value: 0.0,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(
            ctx,
            |ui| {
                

                // The central panel the region left after adding TopPanel's and SidePanel's
                ui.heading("Client manager");
                ui.horizontal(|ui| {

                    ui.vertical(|ui| {
                        ui.add_space(16.0);

                        if self.inProgressDevice.clone().is_none() && ui.button("new client").clicked() {
                            self.inProgressDevice = Some(Default::default());
                        } else if let Some(ref mut device) = &mut self.inProgressDevice {
                            let response = ui.add(egui::TextEdit::singleline(&mut device.name));


                            if ui.button("finish").clicked() {
                                if self.devices.iter().any(|d| d.name == device.name) {
                                    self.warn = true;
                                } else {
                                    self.devices.push(self.inProgressDevice.take().unwrap());
                                    self.warn = false;
                                }
                            }
                            if self.warn {
                                ui.colored_label(Color32::RED, "try a unique name");
                            }
                        }

                        egui::CollapsingHeader::new("Clients")
                          .show(ui, |ui| {

                              egui::ScrollArea::vertical()
                                .min_scrolled_height(ctx.screen_rect().height()-175.0)
                                .max_height(160.0)
                                .show(ui, |ui| {
                                  for mut device in self.devices.iter_mut() {
                                      egui::CollapsingHeader::new(&device.name).show(ui, |ui| {
                                          ui.add(egui::Slider::new(&mut device.frequency_min, 0.0..=10000.0).text("min range"));
                                          ui.add(egui::Slider::new(&mut device.frequency_max, 0.0..=10000.0).text("max range"));
                                          if ui.button("remove device").clicked() {
                                              device.remove = true
                                          }
                                      });
                                  }
                                  self.devices = self.devices.iter()
                                    .filter(|x| !x.remove)
                                    .cloned()
                                    .collect();
                              })
                          });

                    });
                    ui.separator();
                    ui.vertical(|ui| {
draw_sector_map(ui);
                    })


               });




                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    //powered_by_egui_and_eframe(ui);
                    egui::warn_if_debug_build(ui);
                });
            }
        );
    }
}

fn draw_sector_map(ui: &mut Ui) {

    ;
       let size = Vec2::splat(100.0);
       let (response, painter) = ui.allocate_painter(size, Sense::hover());
       let rect = response.rect;
       let c = rect.max;

       let rectangle= Shape::Rect(RectShape {
                   rect: Rect {min: Pos2::new(c.x - 50.0,c.y - 50.0),
                               max: Pos2::new(c.x,c.x) },
                   corner_radius: CornerRadius::same(0),
                   fill: Color32::RED,
                   stroke: Default::default(),
                   stroke_kind: StrokeKind::Inside,
                   round_to_pixels: None,
                   blur_width: 0.0,
                   brush: None,
               });

                painter.add(rectangle);



    //ui.painter().set()
}


