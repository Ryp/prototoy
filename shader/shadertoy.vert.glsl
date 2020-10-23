#version 300 es
precision highp float;

void main()
{
    vec2 vertices[3];

    vertices[0] = vec2(-1.0, 1.0);
    vertices[1] = vec2( 3.0, 1.0);
    vertices[2] = vec2(-1.0,-3.0);

    gl_Position = vec4(vertices[gl_VertexID], 0.0, 1.0);
}
