# orbtk-popup

Modified OrbTk popup widget (so have the same licence).
This popup is presented using the overlay layer, so it will float over other content.
It is possible to set as target an entity or a point, so can be placed everywhere.
Lastly it have a `relative_position` property that select the position relative to the target.
The relative position could be Top,Bottom,Left,Right.
In future, a radiant based implementation could be used, so the user could place the popup everywhere around the target.

![alt text](https://github.com/uniformbuffer/orbtk-popup/blob/master/image.png?raw=true)

What work:
- Relative position to a target, including distance.
- Target can be a point or an entity.
- Popup show and hidden.
- Visibility correction: if visibility is setted to Hidden, the popup will change it to Collapsed

What not work:
- The popup is decentered. Formulas look like correct. This happen only when the target is an entity; if it is a point it is positioned correctly.
- I have not tested it, but teoretically if the target entity move, the popup is not aware of it, so it will stand still.

What can be improved:
- Radiant based implementation for RelativePosition to improve precision. It's a good idea to keep current RelativePosition enum values, so user do not have to think about radiants. Add a new enum value called Radiant(f64).
