#version 330 core

in vec4 v_color;
in vec2 v_texture_coordinate;

uniform sampler2D texture_sampler;

void main() {
    vec4 texture_color = texture(texture_sampler, v_texture_coordinate);
    float alpha = texture_color.a * v_color.a;
    gl_FragColor = vec4(v_color.rgb * alpha, alpha);
}