#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in float aColor;
layout (location = 2) in vec2 aTexCoord;

out vec2 TexCoord;
out float Color;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;


void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    Color = aColor;
    TexCoord = vec2(aTexCoord.x, aTexCoord.y);
}
