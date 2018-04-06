#version 330

uniform mat4 transform;
uniform mat3 normal_matrix;

in vec3 offset;
in vec3 normal;
out vec3 world_normal;

void main()
{
    world_normal = normal_matrix * normal;
    gl_Position = transform * vec4(offset, 1.0);
}

---

#version 330

uniform mat4 light_direction_matrix;
uniform mat4 light_color_matrix;
uniform vec4 albedo;

in vec3 world_normal;
out vec4 color;

void main()
{
    vec4 light_contributions = max(light_direction_matrix * vec4(-world_normal, 1.0), 0.0);
    vec4 light_color = light_color_matrix * light_contributions;
    color = vec4(light_color.rgb, 1.0) * albedo;
}

