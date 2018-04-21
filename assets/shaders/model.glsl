#version 330

uniform mat4 transform;
uniform mat3 normal_matrix;

in vec3 offset;
in vec3 normal;
out vec3 world_normal;
out vec3 model_pos;
out vec3 model_normal;

void main()
{
    world_normal = normal_matrix * normal;
    model_pos = offset;
    model_normal = normal;
    gl_Position = transform * vec4(offset, 1.0);
}

---

#version 330

uniform sampler2D colormap;
uniform mat4 light_direction_matrix;
uniform mat4 light_color_matrix;
uniform vec4 albedo;
uniform float specular_power;
uniform vec3 specular_color;

// TODO(claire): Use per-fragment view vector
uniform vec3 view_vector;

in vec3 world_normal;
in vec3 model_pos;
in vec3 model_normal;
out vec4 color;

void main()
{
    // Texturing
    vec3 blending = normalize(max(abs(model_normal), 0.00001));
    blending /= dot(blending, vec3(1.0, 1.0, 1.0));
    vec4 color_x = texture(colormap, model_pos.yz) * blending.x;
    vec4 color_y = texture(colormap, model_pos.xz) * blending.y;
    vec4 color_z = texture(colormap, model_pos.xy) * blending.z;
    vec4 color_sample = color_x + color_y + color_z;

    // Diffuse
    vec4 light_contributions = max(light_direction_matrix * vec4(-world_normal, 1.0), 0.0);
    vec4 light_color = light_color_matrix * light_contributions;
    color = vec4(light_color.rgb, 1.0) * albedo * color_sample;

    // TODO(claire): Fix this god damn mess
    // Specular
    vec3 light_dir = -light_direction_matrix[0].xyz;
    vec3 h = normalize(light_dir + view_vector);
    float h_dot_n = dot(h, world_normal);
    float eight_pi = 25.13274122872;
    float conservation = (specular_power + 8.0) / eight_pi;
    float light = pow(h_dot_n, specular_power) * conservation;
    color += vec4(specular_color * light, 0.0);
}

