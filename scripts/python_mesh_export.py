import bpy

# Select the first (and hopefully only) mesh
bpy.data.objects[0].select = True

# Create a copy for the shadow geometry
bpy.ops.object.duplicate()
bpy.data.objects[1].name = "Shadow"

# Remove the edge-split modifier from the shadow geometry
bpy.ops.object.modifier_remove(modifier="EdgeSplit")

# Export both objects
bpy.ops.export_scene.obj(
        filepath="OUTPUTPATH", use_materials=False, use_triangles=True)
