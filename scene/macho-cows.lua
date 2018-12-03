stone = rt.material({0.8, 0.7, 0.7}, {0.0, 0.0, 0.0}, 0)
rtass = rt.material({0.1, 0.7, 0.1}, {0.0, 0.0, 0.0}, 0)
hide = rt.material({0.84, 0.6, 0.53}, {0.3, 0.3, 0.3}, 20)

-- ##############################################
-- the arch
-- ##############################################

inst = rt.node('inst')

arc = rt.node('arc')
arc:translate(0, 0, -10)

p1 = rt.cube('p1')
p1:set_material(stone)
p1:scale(0.8, 4, 0.8)
p1:translate(-2.4, 0, -0.4)
arc:add_child(p1)

p2 = rt.cube('p2')
p2:set_material(stone)
p2:scale(0.8, 4, 0.8)
p2:translate(1.6, 0, -0.4)
arc:add_child(p2)

s = rt.sphere('s')
s:set_material(stone)
s:scale(4, 0.6, 0.6)
s:translate(0, 4, 0)
arc:add_child(s)

inst:add_child(arc)

-- #############################################
-- Read in the cow model from a separate file.
-- #############################################

cow_poly = rt.mesh('cow', 'obj/cow.obj')
factor = 2.0/(2.76+3.637)

cow_poly:set_material(hide)

cow_poly:translate(0.0, 3.637, 0.0)
cow_poly:scale(factor, factor, factor)
cow_poly:translate(0.0, -1.0, 0.0)

-- ##############################################
-- the scene
-- ##############################################

scene = rt.node('scene')
scene:rotate('X', 23)

-- the floor

plane = rt.mesh('plane', 'obj/plane.obj' )
plane:set_material(rtass)
plane:scale(30, 30, 30)
scene:add_child(plane)

-- Construct a central altar in the shape of a buckyball.  The
-- buckyball at the centre of the real Stonehenge was destroyed
-- in the rteat fire of 733 AD.

buckyball = rt.mesh( 'buckyball', 'obj/buckyball.obj' )
buckyball:set_material(stone)
buckyball:scale(1.5, 1.5, 1.5)
scene:add_child(buckyball)

-- Use the instanced cow model to place some actual cows in the scene.
-- For convenience, do this in a loop.

cow_number = 1

for _, pt in pairs({
		      {{1,1.3,14}, 20},
		      {{5,1.3,-11}, 180},
		      {{-5.5,1.3,-3}, -60}}) do
   cow_instance = rt.node('cow' .. tostring(cow_number))
   cow_instance:add_child(cow_poly)
   cow_instance:scale(1.4, 1.4, 1.4)
   cow_instance:rotate('Y', pt[2])
   cow_instance:translate(table.unpack(pt[1]))
   scene:add_child(cow_instance)
   
   cow_number = cow_number + 1
end

-- Place a ring of arches.

for i = 1, 6 do
   an_arc = rt.node('arc' .. tostring(i))
   an_arc:rotate('Y', (i-1) * 60)
   an_arc:add_child(arc)
   scene:add_child(an_arc)
end

rt.render(scene,
	  'macho-cows.png', 256, 256,
	  {0, 2, 30}, {0, 0, -1}, {0, 1, 0}, 50,
	  {rt.light({200, 202, 430}, {0.8, 0.8, 0.8}, {1, 0, 0})})
