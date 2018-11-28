## Feature List

### Core
- [x] Can output an image
- [x] Scenes can be loaded through lua files
- [x] Can specify lua file through command line argument
- [x] Can load an obj file into a Mesh structure
- [ ] Configuration can be loaded through lua
- [ ] Errors (i.e. lua issues) are passed up through Result and handled

### Ray Tracing
- [x] Intersection with SceneNode is working
- [x] Spheres can be rendered
- [x] Cubes can be rendered
- [x] Meshes can be rendered
- [x] Cylinders can be rendered
- [ ] Cones can be rendered (Need Normals)
- [x] Phong lighting is implemented
- [x] Shadow rays for simple shadows
- [ ] Supersampling (for AA)
- [ ] Texture Mapping
- [ ] L-System modelling (See [here](https://github.com/shaunbennett/lindenmayer))
- [ ] Bump Mapping
- [x] Soft shadow lighting
- [ ] Spotlight based lighting
- [ ] Volumetric solids (for fog)

### Performance
- [x] Bounding volumes on meshes
- [x] Multithreading
- [ ] Spacial structure for node tree

### Noted Extras
- [x] A background is rendered on every scene
- [x] Objects are rendered front to back (don't show hidden surfaces)
- [x] Hierarchical transforms work down the node tree
