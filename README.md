<p align="center"><img src="https://github.com/shaunbennett/lucis/raw/master/render/soft_shadows.png" width="256"><img src="https://github.com/shaunbennett/lucis/raw/master/render/sample.png" width="256"><img src="https://github.com/shaunbennett/lucis/raw/master/render/fog_noman.png" width="256"></p>

# lucis
Lucis is a basic ray tracer written in rust. Scenes are created using lua scripting, based off primitive types or meshes, and manipulated using hierarchical affine transformations (scale, rotate, translate). Scene modelling examples can be found in the [scene](https://github.com/shaunbennett/lucis/tree/master/scene) directory and example renders can be found in [render](https://github.com/shaunbennett/lucis/tree/master/render).

### Features
- Lua bindings for scene modelling (see [Scripting](#scripting))
- Hierarchical Modelling
- Sphere, Cube, Cone, and Cylinder Primitive Types
- Meshes using [obj format](https://en.m.wikipedia.org/wiki/Wavefront_.obj_file) (only supports triangle faces)
- Bounding volumes on meshes for improved performance
- [Phong Illumination](https://en.m.wikipedia.org/wiki/Phong_reflection_model)
- Shadow rays
- Soft shadows using spherical light sources
- Texture mapping for primitives
- Multithreaded rendering
- Volumetric objects with fog and lighting effects
- Generated background scene behind the render
- Animation rendering (can be done through lua scripts)

## Scripting
### Object Creation
|Command | Description|
|--------|------------|
|rt.node(_**name**_)|Create a new hierarchical node with the name _**name**_|
|rt.sphere(_**name**_)|Create a sphere node centered at (0,0,0) with radius 1 and name _**name**_|
|rt.cube(_**name**_)|Create a cube node with corners (0,0,0), (1,1,1) and name _**name**_|
|rt.cone(_**name**_)|Create a cone node with base (0,0,0), radius 1, height 1 and name _**name**_|
|rt.mesh(_**name**_, _**file_name**_)|Create a mesh node from file _**file_name**_ and name _**name**_|
|rt.material(_**d**_, _**s**_, _**p**_)|Create a phong material with diffuse constants _**d**_, spectral _**s**_ and shininess _**p**_|
|rt.light(_**c**_, _**pos**_, _**f**_)|Create a new light with color _**c**_, position _**pos**_, falloff _**f**_|
|rt.print(_**node**_)|Print a node (and all of its children) to standard out|
|rt.render()||
### Node Manipulation
|Command|Description|
|----|----|
|_node_:translate(_**x**_, _**y**_, _**z**_)|Translate _node_ by (_**x**_, _**y**_, _**z**_)|
|_node_:scale(_**x**_, _**y**_, _**z**_)|Scale _node_ by (_**x**_, _**y**_, _**z**_)|
|_node_:rotate(_**axis**_, _**degrees**_)|Rotate _node_ on axis _**axis**_ by _**degrees**_ degrees|
|_node_:add_child(_**child**_)|Copy the node _**child**_ as a child to _node_|

### Volumetric Solids and Effects
|Command | Description|
|--------|------------|
|rt.volume_box(_**pos**_, _**size**_)|Create a new volumetric box with position _**pos**_ and size _**size**_|
|rt.volume_cone(_**pos**_, _**scale_y**_, _**rot**_)|Create a new volumetric cone with position _**pos**_, y scaling _**scale_y**_, and rotation _**rot**_|
|rt.effect_fog(_**color**_)|Create a fog effect with the fog color _**color**_|
|rt.effect_light(_**color**_)|Create a light effect with the light color _**color**_|
|rt.effect_solid(_**color**_)|Create a solid color effect with the color _**color**_|
|_volume_:set_effect(_**effect**_)|Set the effect for the volumetric solid _volume_|

### Light Manipulation
By default, lights act as a point light meaning they will only generate hard shadows. You can can modify the light to act a soft light
|Command|Description|
|----|----|
|_light_:set_soft(_**radius**_, _**samples**_)|Set a light to be a soft light with radius _**radius**_ and _**samples**_ light samples.

## Usage
Clone to repository and run `cargo build --release`. A binary will be built at `target/release/lucis`. The program can be ran as `lucis <file_name>` where `file_name` is the lua file you would like to run and `lucis` is the path to the binary. For example, try `lucis soft_shadows.lua`.

### TODO List
- [ ] Adaptive Supersampling
- [ ] Spacial partitioning of the hierarchical scene structure for improved performance
- [ ] [Phong shading](https://en.m.wikipedia.org/wiki/Phong_shading) for meshes
- [ ] Texture mapping for meshes
- [ ] Bump mapping
- [ ] Reflections
