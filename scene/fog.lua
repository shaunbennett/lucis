gold = rt.material({0.9, 0.8, 0.4}, {0.8, 0.8, 0.4}, 25)
grass = rt.material({0.1, 0.7, 0.1}, {0.0, 0.0, 0.0}, 0)
blue = rt.material({0.7, 0.6, 1}, {0.5, 0.4, 0.8}, 25)
gray = rt.material({0.6, 0.6, 0.6}, {0.0, 0.0, 0.0}, 0)

ground = rt.material({0.611764706, 0.341176471, 0.031372549}, {0.0, 0.0, 0.0}, 0)
tree_brown = rt.material({.325490196, .207843137, .039215686}, {0.0, 0.0, 0.0}, 0)
tree_texture = rt.textured_material("texture/tree_texture.png", 1.0, 1.0, {0.0, 0.0, 0.0}, 0)

scene = rt.node('scene')
-- scene:translate(0, -6, 0)
-- scene:rotate('x', 10)

-- the floor
floor = rt.mesh('plane', 'obj/plane.obj' )
floor:set_material(ground)
floor:scale(100, 1, 100)

for x=1,6 do
	for z=1,8 do
		sphere = rt.cylinder('fog_cylinder')
		sphere:set_material(tree_texture)
		sphere:scale(0.35, 4, 0.35)
		sphere:translate((2 * x) - 8 + (z%2), 0, -14 - (4*z))
		scene:add_child(sphere)
	end

end


-- sphere = rt.cylinder('fog_cylinder')
-- sphere:set_material(tree_texture)
-- sphere:scale(0.3, 5, 0.3)
-- sphere:translate(0, 0, -10)
-- scene:add_child(sphere)

scene:add_child(floor)

-- The lights
l1 = rt.light({10,30,10}, {0.6, 0.6, 0.6}, {1, 0.0005, 0})
-- l1:set_soft(5, 64)
-- l2 = rt.light({0, 5, 30}, {0.8, 0.8, 0.8}, {1, 0.0005, 0})
lights = { l1 }

rt.render(scene, 'fog.png', 2048, 2048, {0,  6, 1}, {0, 7, -8}, {0, 1, 0}, 30, lights)
