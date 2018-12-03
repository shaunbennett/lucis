-- test for hierarchical ray-tracers.
-- Thomas Pflaum 1996


gold = rt.material({0.9, 0.8, 0.4}, {0.8, 0.8, 0.4}, 25)
grass = rt.material({0.1, 0.7, 0.1}, {0.0, 0.0, 0.0}, 0)
blue = rt.material({0.7, 0.6, 1}, {0.5, 0.4, 0.8}, 25)
gray = rt.material({0.6, 0.6, 0.6}, {0.0, 0.0, 0.0}, 0)

scene = rt.node('scene')
scene:rotate('X', 23)
scene:translate(6, -2, -15)

-- the arc
arc = rt.node('arc')
arc:translate(0,0,-10)
arc:rotate('Y', 60)
p1 = rt.cube('p1')
p1:set_material(gold)
p1:scale(0.8, 4, 0.8)
p1:translate(-2.4, 0, -0.4)

p2 = rt.cube('p2')
p2:set_material(gold)
p2:scale(0.8, 4, 0.8)
p2:translate(1.6, 0, -0.4)

s = rt.sphere('s')
s:set_material(gold)
s:scale(4, 0.6, 0.6)
s:translate(0, 4, 0)


arc:add_child(s)
arc:add_child(p1)
arc:add_child(p2)
scene:add_child(arc)


s1 = rt.sphere('s1')
s1:translate(-6, 1, -1)
s1:set_material(gold)
s2 = rt.sphere('s2')
s2:translate(-6, 1, -3)
s2:set_material(gold)
s3 = rt.sphere('s3')
s3:translate(-6, 1, -5)
s3:set_material(gold)

scene:add_child(s1)
scene:add_child(s2)
scene:add_child(s3)

-- the floor
plane = rt.mesh('plane', 'obj/plane.obj' )
plane:set_material(gray)
plane:scale(30, 30, 30)

-- sphere
poly = rt.mesh('poly', 'obj/dodeca.obj' )
poly:translate(-2, 1.618034, 0)
poly:set_material(blue)


scene:add_child(plane)
scene:add_child(poly)

-- The lights
l1 = rt.light({10,10,20}, {0.2, 0.2, 0.2}, {1, 0.0005, 0})
l2 = rt.light({0, 5, 30}, {0.8, 0.8, 0.8}, {1, 0.0005, 0})
lights = {l1, l2}

-- for i=1,360 do
	-- scene:translate(-6, 2, 15)
	-- scene:rotate('X', 1)
	-- scene:translate(6, -2, -15)
	rt.render(scene, 'test2.png', 1000, 1000, 
		  {0, 0, 0,}, {0, 0, -1}, {0, 1, 0}, 50, lights)
	  -- end
