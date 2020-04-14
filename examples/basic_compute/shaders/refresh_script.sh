#!/bin/bash
      
inotifywait -e close_write,moved_to,create -m . |
while read -r directory events filename; do
  if [ "$filename" = "shader.vert" ]; then
    glslc --target-env=vulkan1.2 --target-spv=spv1.3 -fshader-stage=vert shader.vert -o vert.spv
    echo "recompiled vertex shader."
  fi
  if [ "$filename" = "shader.frag" ]; then
    glslc --target-env=vulkan1.2 --target-spv=spv1.3 -fshader-stage=frag shader.frag -o frag.spv
    echo "recompiled fragment shader."
  fi
  if [ "$filename" = "calculate.comp" ]; then
    glslc --target-env=vulkan1.2 --target-spv=spv1.3 -fshader-stage=comp calculate.comp -o calculate.spv
    echo "recompiled calculate shader."
  fi
  if [ "$filename" = "integrate.comp" ]; then
    glslc --target-env=vulkan1.2 --target-spv=spv1.3 -fshader-stage=comp integrate.comp -o integrate.spv
    echo "recompiled integrate shader."
  fi

done
