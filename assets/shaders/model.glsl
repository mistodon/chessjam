#version 330

uniform mat4 transform;

in vec3 offset;

void main()
{
    gl_Position = transform * vec4(offset, 1.0);
}

---

#version 330

uniform vec4 tint;

out vec4 color;

void main()
{
    color = tint;
}

