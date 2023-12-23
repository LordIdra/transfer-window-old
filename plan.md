Fetaure tracker
[x] Figure out how to get double precision in shaders without completely destroying performance
[x] Move camera freely while locked onto an object (remain in inertial frame)
[x] Keys to speed up / slow down simulation
[x] Select different objects by double clicking near them w/ prioritisation algorithm
[x] Fix weird camera switch flickering
[x] Key press recenters camera to selected object
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
[x] Orbit tessellation
[x] Add time in toolbar
[x] Move toolbar to better location
[X] Draw burn segments
[x] Better algorithm for segment colors
[x] API to add and execute burns using simple integrator
[x] Function to recalculate trajectory so we can redraw vessel trajectory as burn is created
[x] Fix tail thing
[x] Selected icon takes precedence
[x] Allow overwriting burns
[x] Icon to indicate burn position
[x] Snap to burn icon instead of orbit point
[x] Possibility to select burn (more enums :D)
[x] Fix deallocation (lol)
[x] Fix icon overlap for icons on same body
[x] Fix not being able to select orbit points on sections of hyperbola?
[x] Deselect burn by clicking elsewhere as with orbit points
[x] Allow burns to be deleted via top left button
[x] Show info about the burn in top left
[x] Select burn immediately after creation
[x] Allow pausing
[x] Paused text
[x] Add a few seconds to warp time
[x] Implement and write tests for black-box solver
[ ] System to compute trajectories for one object at a time (ie spacecraft)
[ ] Cache system to compute trajectories for the entire system
[ ] Figure out how to draw orbit direction symbols for adjusting burn
[ ] Draw symbols prograde, retrograde, radial in and radial out
[ ] Figure out how to detect if user is dragging a symbol (dot product?)
[ ] Animate symbol dragging
[ ] When symbol clicked, adjust parameters of the burn

I am going to quit coding and go live in a hut in the woods tracker
[x] Wtf is up with being able to flip the zoom negative how the hell is that shit possible (it seems to have just gone away???)
[x] What the fuck is happening with the deterministic prediction algorithm behaving non-deterministically (SOLVED IT)
[ ] Ultra sophisticated tessellation initial point algorithm (fml)

Bugs + technical debt tracker
[x] Better method for hyperbola Kepler equation
[x] check get_time_since_last_periapsis nonsense is actually needed
[x] Kepler solver failing to converge
[x] Janky method to find how many orbits completed in elliptical orbit (just floordiv or smth instead - can be method)
[x] Would be better for Segment to store rc-refcell-xyz
[x] The 40 second delay between SOI changes aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
[x] L A R G E hyperbola when creating burn on an existing hyperbola?????????????????? (it was the modulus thing lol)
[ ] Spacecraft can't change SOI while performing burn
[ ] Spacecraft burns aren't included in predictions
[x] Use velocity perpendicular instead of displacement unit for trajectory visualisation to prevent different thicknesses
[x] Weird desync that happens when creating a burn while another burn is happening

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

Untermairhof01