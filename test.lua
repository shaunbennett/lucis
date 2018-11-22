root_node = node("root")


-- Create a sphere
sphere_node = sphere("sphere")
sphere_node:scale(2, 2, 2)
sphere_node:translate(0, 0, -20)

root_node:add_child(sphere_node)

print_node(root_node)

render(root_node, "test_image.png", 512, 512, {0, 0, 0}, {0, 0, -1}, {0, 1, 0}, 30)
