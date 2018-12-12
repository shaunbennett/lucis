root_node = rt.node("root")

red = rt.material({1.0, 0.0, 0.0}, {0.1, 0.1, 0.1}, 10)
blue = rt.material({0.0, 0.0, 1.0}, {0.1, 0.1, 0.1}, 10)
white = rt.material({1.0, 1.0, 1.0}, {0.1, 0.1, 0.1}, 10)

-- Create a sphere
sphere_node = rt.sphere("sphere")
sphere_node:set_material(blue)
sphere_node:scale(0.5, 0.5, 0.5)
sphere_node:translate(-1, 0.5, -10)

sphere_node2 = rt.sphere("sphere2")
sphere_node2:set_material(red)
sphere_node2:scale(1.0, 1.0, 1.0)
sphere_node2:translate(1, 1.0, -15)

root_node:add_child(sphere_node)
root_node:add_child(sphere_node2)

floor = rt.mesh('floor', 'obj/plane.obj')
floor:set_material(white)
floor:scale(100,100,100)

root_node:add_child(floor)

root_node:rotate('x', 10)
root_node:translate(0, -3, 0)

light1 = rt.light({5, 6, 5}, {0.8, 0.8, 0.8}, {1, 0, 0})
light1:set_soft(3, 128)
ambient_lighting = {0.2, 0.2, 0.2}

rt.render(root_node, "soft_shadows.png", 2000, 2000, {0, 0, 0}, {0, 0, -1}, {0, 1, 0}, 30, {0, 0, 0}, ambient_lighting, {light1})
