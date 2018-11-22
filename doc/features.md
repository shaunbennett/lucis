## Feature List

### Core
- [x] Can output an image
- [x] Scenes can be loaded through lua files
- [ ] Can specify lua file through command line argument
- [ ] Can load an obj file into a Mesh structure
- [ ] Configuration can be loaded through lua

### Ray Tracing
- [x] Intersection with SceneNode is working
- [x] Spheres can be rendered
- [ ] Cubes can be rendered
- [ ] Meshes can be rendered
- [ ] Bounding volumes on meshes
- [ ] Cylinders can be rendered
- [ ] Cones can be rendered
- [ ] Phong lighting is implemented
- [ ] Shadow rays for simple shadows
- [ ] Supersampling (for AA)
- [ ] Texture Mapping
- [ ] L-System modelling
- [ ] Bump Mapping
- [ ] Spotlight lighting with soft shadows
- [ ] Volumetric solids (for fog)

### Performance
- [ ] Bounding volumes on meshes
- [ ] Multithreading
- [ ] Spacial structure for node tree

### Noted Extras
- [x] A background is rendered on every scene
- [x] Objects are rendered front to back (don't show hidden surfaces)
- [x] Hierarchical transforms work down the node tree