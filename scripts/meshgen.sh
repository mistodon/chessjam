#!/bin/bash

set -e

mkdir -p "assets/meshes"

readonly sourcepath=$(cd "source_assets/meshes" && pwd)
readonly destpath=$(cd "assets/meshes" && pwd)

for meshfile in $(ls "source_assets/meshes"); do
    cat scripts/python_mesh_export.py | sed "s;OUTPUTPATH;${destpath}/${meshfile%.blend}.obj;" | /Applications/Blender/blender.app/Contents/MacOS/blender "$sourcepath/$meshfile" -b --python-console
done
