void mainImage(out vec4 fragColor, in vec2 fragCoord)
{
    vec2 positionUV = fragCoord / iResolution.xy;

    vec3 color = vec3(0.5, 0.5, 0.5);
    fragColor = vec4(pow(color, vec3(2.2)), 1.0);
}
