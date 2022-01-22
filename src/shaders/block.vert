#version 140

        in vec3 position;
        in vec2 tex_coords;
        in float opacity;

        out vec2 v_tex_coords;
        out float v_opacity;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            v_opacity = opacity;
            gl_Position = matrix * vec4(position, 1.0);
        }