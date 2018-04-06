#version 330

in vec3 offset;
in vec3 normal;

uniform mat4 transform;
uniform vec3 model_space_shadow_direction;


void main()
{
    float shadow_length = step(0.0, -dot(normal, model_space_shadow_direction)) * 10.0;
    gl_Position = transform * vec4(offset + model_space_shadow_direction * shadow_length, 1.0);
}

---

#version 330

out vec4 color;

void main()
{
    color = vec4(0.0, 0.0, 0.0, 1.0);
}

