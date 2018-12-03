gold = rt.material({0.9, 0.8, 0.4}, {0.8, 0.8, 0.4}, 25)
grass = rt.material({0.1, 0.7, 0.1}, {0.0, 0.0, 0.0}, 0)
blue = rt.material({0.7, 0.6, 1}, {0.5, 0.4, 0.8}, 25)
gray = rt.material({0.6, 0.6, 0.6}, {0.0, 0.0, 0.0}, 0)
tree = rt.textured_material("texture/tree_texture.png", 1.0, 1.0, {0.0, 0.0, 0.0}, 0)

ground = rt.material({0.611764706, 0.341176471, 0.031372549}, {0.0, 0.0, 0.0}, 0)
tree_brown = rt.material({.325490196, .207843137, .039215686}, {0.0, 0.0, 0.0}, 0)
tree_texture = rt.textured_material("texture/tree_texture.png", 1.0, 1.0, {0.0, 0.0, 0.0}, 0)
ground_texture = rt.textured_material("texture/cracked.jpg", 0.2, 0.2, {0.0, 0.0, 0.0}, 0)

light_gray = rt.material({0.3, 0.3, 0.3}, {0.0, 0.0, 0.0}, 0)
dark_gray = rt.material({0.05, 0.05, 0.05}, {0.0, 0.0, 0.0}, 0)

scene = rt.node('scene')
-- scene:translate(0, -6, 0)
-- scene:rotate('x', 10)

-- the floor
floor = rt.mesh('plane', 'obj/plane.obj' )
floor:set_material(ground_texture)
floor:scale(30, 1, 30)
floor:translate(10, 0, -20)

for x=1,4 do
	for z=1,4 do
		tr = dofile('generated/tree'..(((z-1)*4)+x)..'.lua')
		tr:scale(0.6, 0.6, 0.6)
		tr:translate((3 * x) - 7 + (z%2), 0, -10 - (4*z))
		scene:add_child(tr)
	end
end


person = rt.node('person')
person:rotate('y', 30)
person:scale(0.8, 0.8, 0.8)
person:translate(2, 0, -12)
man = rt.mesh('man', 'obj/man.obj')
man:set_material(light_gray)
flashlight = rt.mesh('flashlight', 'obj/flashlight.obj')
flashlight:set_material(dark_gray)

person:add_child(man)
person:add_child(flashlight)



scene:add_child(person)


-- sphere = rt.cylinder('fog_cylinder')
-- sphere:set_material(tree_texture)
-- sphere:scale(0.3, 5, 0.3)
-- sphere:translate(0, 0, -10)
-- scene:add_child(sphere)

scene:add_child(floor)

-- The lights
l1 = rt.light({10,25,10}, {0.6, 0.6, 0.6}, {1, 0.0005, 0})
l1:set_soft(4, 64)
-- l2 = rt.light({0, 5, 30}, {0.8, 0.8, 0.8}, {1, 0.0005, 0})
lights = { l1 }

rt.render(scene, 'fog.png', 3840, 3840, {0.4,  4.5, 1}, {0.4, 5.5, -8}, {0, 1, 0}, 30, lights)
-- rt.render(scene, 'fog.png', 512, 512, {0,  0, 1}, {0, 0, -1}, {0, 1, 0}, 30, lights)
