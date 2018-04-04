#version 330

in vec3 offset;

void main()
{
    gl_Position = vec4(offset, 1.0);
}

---

#version 330

out vec4 color;

void main()
{
    color = vec4(0.4, 0.4, 0.8, 1.0);
}

