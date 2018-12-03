-- 83,53,10
-- brown = rt.material({.325490196, .207843137, .039215686}, {0.0, 0.0, 0.0}, 50)
brown = rt.textured_material("texture/tree_texture.png", 1.0, 1.0, {0.0, 0.0, 0.0}, 0)
red = rt.material({1, 0, 0}, {0.5, 0.4, 0.8}, 25)

scene = rt.node('scene')
scene:rotate('X', 23)
scene:translate(0, -2, -15)

cyl = rt.cylinder('c')
cyl:scale(0.3, 2.5, 0.3)
cyl:set_material(brown)

rotPoint = rt.node('rotPoint')
rotPoint:scale(1/0.3, 1/2.5, 1/0.3)
-- Translate up to the point
rotPoint:translate(0, 0.9, 0)

-- rotSphere = rt.node('rotSphere')
-- rotSphere:scale(0.3, 0.3, 0.3)
-- rotSphere:set_material(brown)

cyl2 = rt.cylinder('c')
-- cyl2:scale(0.25/0.3, 1.5/0.3, 0.25/0.3)
cyl2:scale(0.25, 1.5, 0.25)
cyl2:rotate('z', 30)
cyl2:set_material(brown)

-- cyl3 = rt.cylinder('c')
-- cyl3:scale(0.20/0.3, 1.3/0.3, 0.20/0.3)
-- cyl3:rotate('z', -20)
-- cyl3:set_material(brown)

-- cyl4 = rt.cylinder('c')
-- cyl4:scale(0.20/0.3, 1.6/0.3, 0.20/0.3)
-- cyl4:rotate('x', -25)
-- cyl4:set_material(brown)

rotPoint:add_child(cyl2)
-- rotSphere:add_child(cyl3)
-- rotSphere:add_child(cyl4)
-- rotPoint:add_child(rotSphere)
cyl:add_child(rotPoint)
scene:add_child(cyl)


l1 = rt.light({10,10,20}, {0.2, 0.2, 0.2}, {1, 0.0005, 0})
l2 = rt.light({0, 5, 30}, {0.8, 0.8, 0.8}, {1, 0.0005, 0})
lights = {l1, l2}

scene:translate(-6, 2, 15)
scene:rotate('X', 1)
scene:translate(6, -2, -15)
rt.render(scene, 'lcyl.png', 4000, 4000, 
	  {0, 0, 0,}, {0, 0, -1}, {0, 1, 0}, 50, lights)
