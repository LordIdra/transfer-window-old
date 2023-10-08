#version 330

layout (location = 0) in vec2 position;
layout (location = 1) in vec4 color;
out vec4 v_color;

uniform mat3 matrix;

void main() {
    gl_Position = vec4(matrix * vec3(position, 1.0), 1.0);
    v_color = color;
}