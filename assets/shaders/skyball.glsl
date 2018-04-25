#version 330

uniform mat4 transform;

in vec3 offset;
in vec3 normal;
out vec2 texcoord;

void main()
{
    texcoord = offset.xy + vec2(0.5, 0.5);
    gl_Position = transform * vec4(offset, 1.0);
}

---

#version 330

uniform sampler2D colormap;
uniform float saturation;

in vec2 texcoord;

out vec4 color;

void main()
{
    color = texture(colormap, texcoord);
    float grey = dot(color.rgb, vec3(0.3, 0.6, 0.1));
    color = vec4(mix(vec3(grey, grey, grey), color.rgb, saturation), color.a);
}

