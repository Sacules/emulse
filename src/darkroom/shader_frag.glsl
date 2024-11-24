#version 330 core

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

uniform int invert;
uniform float contrast;
uniform float saturation;
uniform float brightness;
uniform float temperature;

const float PI = 3.141592653589793238462643383279502884197169399375105820974944;
const float max_value = 255.0;
const vec3 kSRGB_luminance_factors = vec3(0.2126, 0.7152, 0.0722);

vec3 invertPixel(vec3 p) {
    return vec3(1.0 - p.r, 1.0 - p.g, 1.0 - p.b);
}

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

vec3 adjustSaturation(vec3 p, float saturation) {
    vec3 intensity = vec3(dot(p, kSRGB_luminance_factors));
    return mix(intensity, p, saturation);
}

vec3 adjustBrightness(vec3 p, float brightness) {
    return p * brightness;
}

vec3 adjustWhiteBalance(vec3 p, float temperature) {
    // Values from: http://blenderartists.org/forum/showthread.php?270332-OSL-Goodness&p=2268693&viewfull=1#post2268693
    mat3x3 m;

    if (temperature <= 6500.0) {
        m = mat3x3(vec3(0.0, -2902.1955373783176, -8257.7997278925690),
                vec3(0.0, 1669.5803561666639, 2575.2827530017594),
                vec3(1.0, 1.3302673723350029, 1.8993753891711275));
    } else {
        m = mat3x3(vec3(1745.0425298314172, 1216.6168361476490, -8257.7997278925690),
                vec3(-2666.3474220535695, -2173.1012343082230, 2575.2827530017594),
                vec3(0.55995389139931482, 0.70381203140554553, 1.8993753891711275));
    }

    return p * mix(clamp(vec3(m[0] / (vec3(clamp(temperature, 1000.0, 40000.0)) + m[1]) + m[2]), vec3(0.0), vec3(1.0)), vec3(1.0), smoothstep(1000.0, 0.0, temperature));
}

void main() {
    vec4 p = texture2D(tex, v_tex_coords);
    if (invert == 1) {
        p.rgb = invertPixel(p.rgb);
    }

    p.rgb = adjustContrast(p.rgb, contrast);
    p.rgb = adjustSaturation(p.rgb, saturation);
    p.rgb = adjustBrightness(p.rgb, brightness);
    p.rgb = adjustWhiteBalance(p.rgb, temperature);

    color = p;
}
