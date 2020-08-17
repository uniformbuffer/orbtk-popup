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
    fn default()->Self {Self::Entity(u32::default())}
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


/// The `PopupState` handles the open and close behavior of the `Popup` widget.
#[derive(Default, AsAny)]
pub struct PopupState {}

impl PopupState
{
    fn update_position(&mut self, _registry: &mut Registry, ctx: &mut Context)
    {
        if let Some(target) = ctx.widget().try_clone::<PopupTarget>("target") {

            let current_bounds: Rectangle = ctx.widget().clone("bounds");

            let real_target_bounds = match target
            {
                PopupTarget::Entity(entity)=>
                {
                    //WARNING: this is true only if called during post_layout_update, otherwise the bounds will refere to space available to the widget, not the effective size
                    let target_bounds: Rectangle = ctx.get_widget(entity.into()).clone("bounds");

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

            let popup_position = match relative_position
            {
                RelativePosition::Left(distance)=>
                {
                    let target_y_center = real_target_bounds.y() + real_target_bounds.height()/2.0;
                    (real_target_bounds.x() - current_bounds.width() - distance,target_y_center - current_bounds.height()/2.0)
                }
                RelativePosition::Right(distance)=>
                {
                    let target_y_center = real_target_bounds.y() + real_target_bounds.height()/2.0;
                    (real_target_bounds.x() + real_target_bounds.width() + distance,target_y_center - current_bounds.height()/2.0)
                }
                RelativePosition::Top(distance)=>
                {
                    let target_x_center = real_target_bounds.x() + real_target_bounds.width()/2.0;
                    (target_x_center - current_bounds.width()/2.0,real_target_bounds.y() - current_bounds.height() - distance)
                }
                RelativePosition::Bottom(distance)=>
                {
                    let target_x_center = real_target_bounds.x() + real_target_bounds.width()/2.0;
                    (target_x_center - current_bounds.width()/2.0,real_target_bounds.y() + real_target_bounds.height() + distance)
                }
            };

            if let Some(bounds) = ctx.widget().try_get_mut::<Rectangle>("bounds")
            {
                bounds.set_x(popup_position.0);
                bounds.set_y(popup_position.1);
            }
            else{println!("Cannot set popup position: missing \"bounds\" property");}
        }
        else {println!("Target not found");}
    }
}

impl State for PopupState {
    fn init(&mut self, _registry: &mut Registry, _ctx: &mut Context) {
        println!("Init");
        /*
        ctx.widget().set("visibility", Visibility::Hidden);
        self.update_position(registry,ctx);
        let popup = ctx.entity;
        println!("Popup entity from PopupState: {}",popup.0);
        ctx.build_context().append_child_to_overlay(popup).expect("Failed to add popup to overlay");
        ctx.widget().update_dirty(true);
        */
    }
/*
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {

    }
*/
    fn update_post_layout(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.update_position(registry,ctx);
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
        relative_position: RelativePosition,

        /// Sets or shares the popup distance from the target.
        distance: f64
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
            .distance(1.0)
            .on_mouse_down(|_, _| true)
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(RectangleRenderObject)
    }
}
