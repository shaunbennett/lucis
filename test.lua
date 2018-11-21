test_function("Hello there!")

node1 = create_node("node1")
node2 = create_node("node2")
node3 = create_node("node3")

node2:add_child(node3)
node1:add_child(node2)

print_node(node1)
