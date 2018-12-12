root_node = rt.node("root")

red = rt.material({1.0, 0.0, 0.0}, {0.1, 0.1, 0.1}, 10)
blue = rt.material({0.0, 0.0, 1.0}, {0.1, 0.1, 0.1}, 10)

-- Create a sphere
sphere_node = rt.sphere("sphere")
sphere_node:set_material(blue)
sphere_node:scale(1, 1, 1)
sphere_node:translate(0, 0, -15)

sphere_node2 = rt.sphere("sphere2")
sphere_node2:set_material(red)
sphere_node2:scale(1.8, 1.8, 1.8)
sphere_node2:translate(0, 0, -20)

-- sphere_node:add_child(sphere_child)
root_node:add_child(sphere_node)
root_node:add_child(sphere_node2)

-- rt.print(root_node)

light1 = rt.light({0, 0, 0}, {0.6, 0.6, 0.6}, {1, 0, 0})
ambient_lighting = {0.2, 0.2, 0.2}

rt.render(root_node, "test_image.png", 4000, 4000, {0, 0, 0}, {0, 0, -1}, {0, 1, 0}, 30, ambient_lighting, {light1})
