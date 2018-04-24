#version 330

uniform mat4 transform;

in vec3 offset;
out vec2 texcoord;

void main()
{
    texcoord = offset.xy + vec2(0.5, 0.5);
    gl_Position = transform * vec4(offset, 1.0);
}

---

#version 330

uniform sampler2D colormap;
uniform vec4 tint;

in vec2 texcoord;
out vec4 color;

void main()
{
    color = texture(colormap, texcoord) * tint;
}
