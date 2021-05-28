#version 330 core

out vec4 FragColor;

in float Color;
in vec3 ourColor;
in vec2 TexCoord;

uniform sampler2D ourTexture;

void main()
{
    FragColor = texture(ourTexture, TexCoord) * Color;//Color;
}