# Things to do
[x] Figure out how to get double precision in shaders without completely destroying performance
[x] Move camera freely while locked onto an object (remain in inertial frame)
[x] Keys to speed up / slow down simulation
[x] Select different objects by double clicking near them w/ prioritisation algorithm
[x] Fix weird camera switch flickering
[x] Key press recenters camera to selected object
[x] Hide orbits of selected object when zoomed in far enough
[x] Icons to represent planets/stars/spacecraft when zoomed out
[x] Icon precedence algorithm
[x] Selected icon takes precedence over all others
[x] Highlight point on orbit when hovering near it
[x] Clicking when a point is highlighted brings up a menu
[x] Add button to menu to warp to that point
[x] Add button that creates a new burn at that point
[x] Figure out what time we need to warp to when warp button pressed
[x] Algorithm to warp to the specific point
[x] Find out why error gets larger with time in warp
[x] Allow warping over multiple orbits
[x] Figure out why there are jumps happening between conic sections???
[x] ECS transition
[ ] Orbit tessellation
[ ] Add time in toolbar
[ ] Figure out how to draw orbit direction symbols for adjusting burn
[ ] Draw symbols prograde, retrograde, radial in and radial out
[ ] Figure out how to detect if user is clicking a symbol
[ ] API to add and execute burns using simple integrator
[ ] Function to recalculate trajectory so we can redraw vessel trajectory as burn is created
[ ] When symbol clicked, adjust parameters of the burn

Technical debt tracker
[x] Better method for hyperbola Kepler equation
[x] check get_time_since_last_periapsis nonsense is actually needed
[x] Kepler solver failing to converge
[ ] Janky method to find how many orbits completed in elliptical orbit (just floordiv or smth instead - can be method)

ECS
- Each component is a representation of state of one part of an entity
- An entity can have multiple components represented by options of components (not IDs)
- Vectors of entity IDs act as a fast accessor to the entities that have a certain component

Components
- PositionComponent (absolute position + position relative to camera)
- VelocityComponent
- MassComponent
- ParentComponent
- CelestialBodyComponent (radius + color + SOI + children)
- TrajectoryComponent

Systems
- CameraUpdateSystem
- TrajectoryPredictionSystem
- TrajectoryUpdateSystem
- TrajectoryRenderSystem
- CelestialBodyRenderSystem
- ObjectIconSystem
