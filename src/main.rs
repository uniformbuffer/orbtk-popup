
use orbtk::prelude::*;
mod popup;
use popup::{Popup,RelativePosition};

static STACK_ID: &'static str = "STACK";
static BTN_ID: &'static str = "BUTTON";
static CMB_ID: &'static str = "COMBO BOX";
static TARGET_ID: &'static str = "TARGET";

#[derive(Copy, Clone)]
enum PopUpAction {
    Show,
    Hide,
    UpdateRelativePosition
}

#[derive(Default, AsAny)]
struct MainViewState {
    action: Option<PopUpAction>,
    show_popup: bool,
    popup: Option<Entity>,
}

impl MainViewState {
    fn toggle_popup(&mut self) {
        if self.show_popup {
            self.action = Some(PopUpAction::Hide);
        } else {
            self.action = Some(PopUpAction::Show);
        }
        self.show_popup = !self.show_popup;
    }

    fn update_relative_position(&mut self) {
        self.action = Some(PopUpAction::UpdateRelativePosition);
    }
}

impl State for MainViewState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        let target_entity = ctx.entity_of_child(TARGET_ID).unwrap();

        let popup = create_popup(target_entity, "Popup text", &mut ctx.build_context());
        //println!("Popup entity from MainView: {}",popup.0);
        ctx.build_context().append_child_to_overlay(popup).expect("Failed to add popup to overlay");
        self.popup = Some(popup);
        self.show_popup = true;
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                PopUpAction::Show => {
                    if let Some(popup) = self.popup {
                        let mut p = ctx.get_widget(popup);
                        p.set("visibility", Visibility::Visible);
                        p.update_dirty(true);
                        change_button_title("Click me to hide popup", ctx);
                        println!("Popup showed !");
                    }
                }
                PopUpAction::Hide => {
                    if let Some(popup) = self.popup {
                        ctx.get_widget(popup).set("visibility", Visibility::Hidden);
                        change_button_title("Click me to show popup", ctx);
                        println!("Popup hided !");
                    }
                }
               PopUpAction::UpdateRelativePosition => {
                    if let Some(popup) = self.popup {
                        println!("Updating relative position");
                        let cmb = ctx.entity_of_child(CMB_ID).unwrap();
                        let selected_index: usize = ctx.get_widget(cmb).clone("selected_index");
                        let relative_position: RelativePosition = ctx.get_widget(popup).clone_or_default("relative_position");
                        let distance = relative_position.get_distance();

                        match selected_index {
                            0 => ctx.get_widget(self.popup.unwrap()).set("relative_position",RelativePosition::Bottom(distance)),
                            1 => ctx.get_widget(self.popup.unwrap()).set("relative_position",RelativePosition::Top(distance)),
                            2 => ctx.get_widget(self.popup.unwrap()).set("relative_position",RelativePosition::Left(distance)),
                            3 => ctx.get_widget(self.popup.unwrap()).set("relative_position",RelativePosition::Right(distance)),
                            _ => panic!()
                        }
                    }
                }
            }
            self.action = None;
        }
    }
}

fn create_popup(target: Entity, text: &str, build_context: &mut BuildContext) -> Entity {
    Popup::new()
        // Entity as target
        .target(target.0)

        // Point as target
        //.target(Point::new(200.0,200.0))

        //Specify the popup position relative to the target (the button in this case)
        .relative_position(RelativePosition::Bottom(1.0))

        //Specify the distance from the target
        .distance(10.0)

        //.open(true)
        .width(150.0)
        .height(150.0)
        .child(
            Container::new()
                .background("#FFFFFF")
                .border_radius(3.0)
                .border_width(2.0)
                .border_brush("#000000")
                .padding(8.0)
                .child(
                    TextBlock::new()
                        .h_align("center")
                        .v_align("top")
                        .foreground("#000000")
                        .text(text)
                        .build(build_context),
                )
                .build(build_context),
        )
        .build(build_context)
}

fn change_button_title(title: &str, ctx: &mut Context) {
    let btn = ctx.entity_of_child(BTN_ID).unwrap();
    ctx.get_widget(btn)
        .set::<String16>("text", String16::from(title));
}

widget!(MainView<MainViewState>);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").margin(16.0).child(
            Grid::new()
                .rows(Rows::create().push(200).push(200).push(200).push(200))
                .columns(Columns::create().push(200).push(200).push(200))
                .id(STACK_ID)
                .child(
                    Button::new()
                        .id(BTN_ID)
                        .attach(Grid::row(0))
                        .attach(Grid::column(0))
                        .v_align("center")
                        .h_align("center")
                        .text("Click me to hide popup")
                        .on_click(move |states, _| -> bool {
                            states.get_mut::<MainViewState>(id).toggle_popup();
                            true
                        })
                        .build(ctx),
                )
                .child(
                    ComboBox::new()
                        .id(CMB_ID)
                        .attach(Grid::row(0))
                        .attach(Grid::column(1))
                        .v_align("center")
                        .h_align("center")
                        //.width(250.0)
                        .on_changed(move |states, entity, property| {
                            println!("Changed {}",property);
                            match property{
                                "selected_index"=>{
                                    states.get_mut::<MainViewState>(entity).update_relative_position();
                                }
                                _=>()
                            }

                        })
                        .items_builder(|bc,index|{
                            match index {
                                0 => TextBlock::new().text("Bottom").build(bc),
                                1 => TextBlock::new().text("Top").build(bc),
                                2 => TextBlock::new().text("Left").build(bc),
                                3 => TextBlock::new().text("Right").build(bc),
                                _ => panic!()
                            }
                        })
                        .count(4)
                        .selected_index(0)
                        .build(ctx),
                )
                .child(
                    Container::new()
                        .id(TARGET_ID)
                        .attach(Grid::row(2))
                        .attach(Grid::column(1))
                        .background("#0000FF")
                        .v_align("stretch")
                        .h_align("stretch")
                        .child(
                            TextBlock::new()
                            .text("Target")
                            .v_align("center")
                            .h_align("center")
                            .build(ctx)
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - Popup example")
                .position((100.0, 100.0))
                .size(750, 750.0)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}


/*
use orbtk::prelude::*;

widget!(MainView);

impl Template for MainView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        let s = self.name("MainView")
            .child(
            Stack::new()
                .id(STACK_ID)
                .h_align("center")
                .spacing(16.0)
                .child(
                    Button::new()
                        .id(BTN_ID)
                        .v_align("top")
                        .h_align("center")
                        .text("Click me to show popup")
                        .width(250.0)
                        .build(ctx),
                )
                .build(ctx)
            );

        let container = Container::new()
            .background("#dfebf5")
            .width(200.0)
            .height(200.0)
            .position((200,200))
            .child(
                TextBlock::new()
                    .foreground("#3b434a")
                    .text("Overlay")
                    .v_align("center")
                    .h_align("center")
                    .build(ctx),
            )
            .build(ctx);

        ctx.append_child_to_overlay(container).unwrap();
        s
    }
}

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - overlay example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
*/
