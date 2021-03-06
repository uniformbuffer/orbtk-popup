use orbtk::prelude::*;

/// The target of the popup, that can be an entity or a fixed point
#[derive(Clone,Debug,PartialEq)]
pub enum PopupTarget
{
    Entity(u32),
    Point(Point)
}

impl Default for PopupTarget
{
    fn default()->Self {Self::Point(Point::new(100.0,100.0))}
}

impl From<u32> for PopupTarget {
    fn from(entity: u32) -> Self {
        Self::Entity(entity)
    }
}

impl From<Point> for PopupTarget {
    fn from(point: Point) -> Self {
        Self::Point(point)
    }
}

impl IntoPropertySource<PopupTarget> for u32 {
    fn into_source(self) -> PropertySource<PopupTarget> {
        PropertySource::Value(self.into())
    }
}

impl IntoPropertySource<PopupTarget> for Point {
    fn into_source(self) -> PropertySource<PopupTarget> {
        PropertySource::Value(self.into())
    }
}

/// Relative position to the target
#[derive(Clone,Debug,PartialEq)]
pub enum RelativePosition
{
    Top(f64),
    Bottom(f64),
    Left(f64),
    Right(f64)
}

impl Default for RelativePosition
{
    fn default()->Self {Self::Bottom(1.0)}
}
impl RelativePosition
{
    pub fn get_distance(&self)->f64
    {
        match self
        {
            Self::Top(distance)=>*distance,
            Self::Bottom(distance)=>*distance,
            Self::Left(distance)=>*distance,
            Self::Right(distance)=>*distance,
        }
    }
    pub fn to_top(self)->Self {Self::Top(self.get_distance())}
    pub fn to_bottom(self)->Self {Self::Bottom(self.get_distance())}
    pub fn to_left(self)->Self {Self::Left(self.get_distance())}
    pub fn to_right(self)->Self {Self::Right(self.get_distance())}
}

/*
impl From<usize> for RelativePosition {
    fn from(index: usize) -> Self {
        match entity
        {
            0 => Self::Top,
            1 => Self::Botton,
            2 => Self::Left,
            3 => Self::Right,
            _ => panic!()
        }
    }
}
*/
into_property_source!(RelativePosition);

/// The `PopupAction` represent actions that can be sent to `PopupState`.
pub enum PopupAction
{
    UpdatePosition,
    UpdateVisibility
}

/// The `PopupState` handles the open and close behavior of the `Popup` widget.
#[derive(Default, AsAny)]
pub struct PopupState
{
    actions: Vec<PopupAction>
}

impl PopupState
{
    pub fn update_position(&mut self) {self.actions.push(PopupAction::UpdatePosition);}
    pub fn update_visibility(&mut self) {self.actions.push(PopupAction::UpdateVisibility);}

    fn update_position_internal(&mut self, _registry: &mut Registry, ctx: &mut Context)
    {
        if let Some(target) = ctx.widget().try_clone::<PopupTarget>("target") {

            let current_bounds: Rectangle = ctx.widget().clone("bounds");
            let current_constraint: Constraint = ctx.widget().clone("constraint");

            let real_target_bounds = match target
            {
                PopupTarget::Entity(entity)=>
                {
                    let target_position: Point = ctx.get_widget(entity.into()).clone("position");

                    //WARNING: this is true only if called during post_layout_update, otherwise the bounds will refere to space available to the widget, not the effective size
                    let mut target_bounds: Rectangle = ctx.get_widget(entity.into()).clone("bounds");
                    target_bounds.set_position(target_position);
                    target_bounds
                }
                PopupTarget::Point(mut point)=>
                {
                    point.set_x(point.x()+current_bounds.width()/2.0);
                    point.set_y(point.y()+current_bounds.height()/2.0);
                    Rectangle::new(point,(0.0,0.0))
                }
            };

            let relative_position: RelativePosition = ctx.widget().clone_or_default("relative_position");

            let new_popup_bounds = match relative_position
            {
                RelativePosition::Left(distance)=>
                {
                    let current_v_align: Alignment = ctx.widget().clone("v_align");

                    let x = real_target_bounds.x() - current_bounds.width() - distance;
                    let y = current_v_align.align_position(
                        real_target_bounds.height(),
                        current_bounds.height(),
                        real_target_bounds.y(),
                        real_target_bounds.y()+real_target_bounds.height()
                    );

                    let width = current_bounds.width();
                    let height = current_v_align.align_measure(
                        real_target_bounds.height(),
                        current_bounds.height(),
                        0.0,
                        0.0
                    );

                    Rectangle::new((x,y),current_constraint.perform((width,height)))
                }
                RelativePosition::Right(distance)=>
                {
                    let current_v_align: Alignment = ctx.widget().clone("v_align");

                    let x = real_target_bounds.x() + real_target_bounds.width() + distance;
                    let y = current_v_align.align_position(
                        real_target_bounds.height(),
                        current_bounds.height(),
                        real_target_bounds.y(),
                        real_target_bounds.y()+real_target_bounds.height()
                    );

                    let width = current_bounds.width();
                    let height = current_v_align.align_measure(
                        real_target_bounds.height(),
                        current_bounds.height(),
                        0.0,
                        0.0
                    );

                    Rectangle::new((x,y),current_constraint.perform((width,height)))
                }
                RelativePosition::Top(distance)=>
                {
                    let current_h_align: Alignment = ctx.widget().clone("h_align");

                    let x = current_h_align.align_position(
                        real_target_bounds.width(),
                        current_bounds.width(),
                        real_target_bounds.x(),
                        real_target_bounds.x()+real_target_bounds.width()
                    );
                    let y = real_target_bounds.y() - current_bounds.height() - distance;
                    let width = current_h_align.align_measure(
                        real_target_bounds.width(),
                        current_bounds.width(),
                        0.0,
                        0.0
                    );
                    let height = current_bounds.height();

                    Rectangle::new((x,y),current_constraint.perform((width,height)))
                }
                RelativePosition::Bottom(distance)=>
                {
                    let current_h_align: Alignment = ctx.widget().clone("h_align");

                    let x = current_h_align.align_position(
                        real_target_bounds.width(),
                        current_bounds.width(),
                        real_target_bounds.x(),
                        real_target_bounds.x()+real_target_bounds.width()
                    );
                    let y = real_target_bounds.y() + real_target_bounds.height() + distance;
                    let width = current_h_align.align_measure(
                        real_target_bounds.width(),
                        current_bounds.width(),
                        0.0,
                        0.0
                    );
                    let height = current_bounds.height();

                    Rectangle::new((x,y),current_constraint.perform((width,height)))
                }
            };

            ctx.widget().set::<Rectangle>("bounds",new_popup_bounds);
        }
        else {println!("Target not found");}
    }
}

impl State for PopupState {
    fn init(&mut self, _registry: &mut Registry, _ctx: &mut Context) {
        self.update_position();
    }

    fn update_post_layout(&mut self, registry: &mut Registry, ctx: &mut Context) {
        let actions: Vec<PopupAction> = self.actions.drain(..).collect();
        for action in actions {
            match action {
                PopupAction::UpdatePosition=>self.update_position_internal(registry,ctx),
                PopupAction::UpdateVisibility=>
                {
                    let mut widget = ctx.widget();
                    let visibility = widget.get_mut::<Visibility>("visibility");
                    match visibility
                    {
                        Visibility::Hidden=>*visibility = Visibility::Collapsed,
                        _=>()
                    }
                }
            }
        }
    }

}

widget!(
    /// The `Popup` is used to display content that floats over the main content.
    Popup<PopupState> : MouseHandler {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the target id to place the popup.
        target: PopupTarget,

        /// Sets or shares the popup position relative to the target.
        relative_position: RelativePosition
    }
);

impl Template for Popup {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Popup")
            .style("popup")
            .padding(0.0)
            .background("transparent")
            .border_radius(0.0)
            .border_width(0.0)
            .border_brush("transparent")
            .on_mouse_down(|_, _| true)
            .on_changed_filter(vec!["relative_position","target","visibility","v_align","h_align"])
            .on_changed(move |states, entity, property| {
                match property{
                    "relative_position"|"target"|"v_align"|"h_align"=>states.get_mut::<PopupState>(entity).update_position(),
                    "visibility"=>states.get_mut::<PopupState>(entity).update_visibility(),
                    _=>()
                }

            })
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(RectangleRenderObject)
    }
}
