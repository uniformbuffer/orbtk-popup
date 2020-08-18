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

            println!("Target bounds: {:#?}",real_target_bounds);

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
                println!("Popup bounds: {:#?}",bounds);
            }
            else{println!("Cannot set popup position: missing \"bounds\" property");}
        }
        else {println!("Target not found");}
    }
}

impl State for PopupState {
    fn init(&mut self, _registry: &mut Registry, _ctx: &mut Context) {
        self.update_position();
    }
/*
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {

    }
*/
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
            .on_changed_filter(vec!["relative_position","target","visibility"])
            .on_changed(move |states, entity, property| {
                match property{
                    "relative_position"|"target"=>states.get_mut::<PopupState>(entity).update_position(),
                    "visibility"=>states.get_mut::<PopupState>(entity).update_visibility(),
                    _=>()
                }

            })
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(RectangleRenderObject)
    }
}
