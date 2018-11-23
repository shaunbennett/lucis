-- test for hierarchical ray-tracers.
-- Thomas Pflaum 1996

gold = rt.material({0.9, 0.8, 0.4}, {0.8, 0.8, 0.4}, 25)
grass = rt.material({0.1, 0.7, 0.1}, {0.0, 0.0, 0.0}, 0)
blue = rt.material({0.7, 0.6, 1}, {0.5, 0.4, 0.8}, 25)

scene = rt.node('scene')
scene:rotate('X', 23)
scene:translate(6, -2, -15)

-- the arc
arc = rt.node('arc')
arc:translate(0,0,-10)
arc:rotate('Y', 60)
-- p1 = gr.cube('p1')
-- arc:add_child(p1)
-- p1:set_material(gold)
-- p1:scale(0.8, 4, 0.8)
-- p1:translate(-2.4, 0, -0.4)

-- p2 = gr.cube('p2')
-- arc:add_child(p2)
-- p2:set_material(gold)
-- p2:scale(0.8, 4, 0.8)
-- p2:translate(1.6, 0, -0.4)

s = rt.sphere('s')
s:set_material(gold)
s:scale(4, 0.6, 0.6)
s:translate(0, 4, 0)


arc:add_child(s)
scene:add_child(arc)

-- the floor
-- plane = gr.mesh( 'plane', 'Assets/plane.obj' )
-- scene:add_child(plane)
-- plane:set_material(grass)
-- plane:scale(30, 30, 30)

-- sphere
-- poly = gr.mesh( 'poly', 'Assets/dodeca.obj' )
-- scene:add_child(poly)
-- poly:translate(-2, 1.618034, 0)
-- poly:set_material(blue)

-- The lights
l1 = rt.light({200,200,400}, {0.8, 0.8, 0.8}, {1, 0, 0})
l2 = rt.light({0, 5, -20}, {0.4, 0.4, 0.8}, {1, 0, 0})

rt.render(scene, 'hier.png', 3000, 3000, 
	  {0, 0, 0,}, {0, 0, -1}, {0, 1, 0}, 50, {l1, l2})
