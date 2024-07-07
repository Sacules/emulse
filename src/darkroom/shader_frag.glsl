#version 330 core

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

uniform float contrast;

const float PI = 3.141592653589793238462643383279502884197169399375105820974944;
const float max_value = 255.0;

float adjustContrastPixel(float c, float percent) {
    c = c * max_value;
    float d = ((c / max_value - 0.5) * percent + 0.5) * max_value;
    float e = clamp(d, 0.0, max_value);
    return e / max_value;
}

vec3 adjustContrast(vec3 p, float contrast) {
    float percent = pow((100.0 + contrast) / 100.0, 2);
    float new_r = adjustContrastPixel(p.r, percent);
    float new_g = adjustContrastPixel(p.g, percent);
    float new_b = adjustContrastPixel(p.b, percent);

    return vec3(new_r, new_g, new_b);
}

void main() {
    vec4 p = texture2D(tex, v_tex_coords);

    p.rgb = adjustContrast(p.rgb, contrast);

    color = p;
}
