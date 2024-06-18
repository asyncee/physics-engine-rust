#version 330 core
 in vec3 PointColor;
out vec4 FragColor;

void main()
{
    vec2 circCoord = 2.0 * gl_PointCoord - 1.0;
    if (dot(circCoord, circCoord) > 1.0) {
        discard;
    }
    //FragColor = vec4(PointColor, 1.0f);
    FragColor = vec4(0.3, 0.5, 1.0, 1.0f);
}
