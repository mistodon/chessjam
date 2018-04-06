#!/bin/bash

set -e

mkdir -p "assets/meshes"

readonly sourcepath=$(cd "source_assets/meshes" && pwd)
readonly destpath=$(cd "assets/meshes" && pwd)

for meshfile in $(ls "source_assets/meshes"); do
    echo "import bpy;bpy.ops.export_scene.obj(filepath=\"${destpath}/${meshfile%.blend}.obj\", use_materials=False, use_triangles=True)" | /Applications/Blender/blender.app/Contents/MacOS/blender "$sourcepath/$meshfile" -b --python-console
done
