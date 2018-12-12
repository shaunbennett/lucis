-- test for hierarchical ray-tracers.
-- Thomas Pflaum 1996

gold = rt.material({0.9, 0.8, 0.4}, {0.8, 0.8, 0.4}, 25)
grass = rt.material({0.1, 0.7, 0.1}, {0.0, 0.0, 0.0}, 0)
blue = rt.material({0.7, 0.6, 1}, {0.5, 0.4, 0.8}, 25)

scene = rt.node('scene')
-- scene:rotate('X', 23)
-- scene:translate(6, -2, -15)

c = rt.cone('c')
-- c:rotate('X', 180 + 50)
c:translate(0, 0, -5)
c:set_material(gold)

scene:add_child(c)

-- The lights
l1 = rt.light({200,200,400}, {0.8, 0.8, 0.8}, {1, 0, 0})
-- l2 = rt.light({0, 5, -20}, {0.4, 0.4, 0.8}, {1, 0, 0})
lights = {l1}
ambient_lighting = {0.2, 0.2, 0.2}

rt.render(scene, 'cone.png', 1024, 1024, 
	  {0, 0, 0}, {0, 0, -1}, {0, 1, 0}, 50, ambient_lighting, lights)
