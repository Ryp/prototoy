out vec4 glFragColor;

// OpenGL ES
void main()
{
    glFragColor.w = 1.0;
    mainImage(glFragColor, gl_FragCoord.xy);
}

// OpenGL
// void main()
// {
//     gl_FragColor.w = 1.0;
//     mainImage(gl_FragColor, gl_FragCoord.xy);
// }
