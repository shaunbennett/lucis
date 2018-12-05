<p align="center"><img src="https://github.com/shaunbennett/lucis/raw/master/render/sample.png" width="256"><img src="https://github.com/shaunbennett/lucis/raw/master/render/fog_noman.png" width="256"></p>

# lucis
Lucis is a basic ray tracer written in rust. Scenes can be created using lua scripting, based off primitive types and manipulated using hierarchical transformations.

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
### Light Manipulation
By default, lights act as a point light meaning they will only generate hard shadows. You can can modify the light to act a soft light

|Command|Description|
|----|----|
|_light_:set_soft(_**radius**_, _**samples**_)|Set a light to be a soft light with radius _**radius**_ and _**samples**_ light samples.
