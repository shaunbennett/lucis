## File Structure
Rust modules are built around the file structure of a program. Since this is my first rust project, it's important to think about my
module layout to make sure I'm working as effectively as possible.

### Breakdown
The raytracer will be broken into several core modules

1. geometry
    - primitive types (sphere, cube, cylinder, cone, mesh)
    - rays
    - intersection
2. scene
    - lua
    - material
    - texturing
    - scene tree
