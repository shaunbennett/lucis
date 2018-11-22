root_node = rt.node("root")

sphere_child = rt.sphere("sphere_child")
sphere_child:translate(-4, 0, 0)

-- Create a sphere
sphere_node = rt.sphere("sphere")
sphere_node:scale(1, 1, 1)
sphere_node:translate(0, 0, -20)

sphere_node:add_child(sphere_child)
root_node:add_child(sphere_node)

rt.print(root_node)

rt.render(root_node, "test_image.png", 512, 512, {0, 0, 0}, {0, 0, -1}, {0, 1, 0}, 30)
