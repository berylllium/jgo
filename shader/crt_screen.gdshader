shader_type canvas_item;

uniform sampler2D screen_texture : hint_screen_texture, filter_linear_mipmap;

const float curvature = 4.0;
const float vignette_factor = 1.75;

void fragment() {
    vec2 centered_uv = SCREEN_UV * 2.0 - 1.0;
	vec2 uv_offset = centered_uv.yx / curvature;
	vec2 warped_uv = centered_uv + centered_uv * uv_offset * uv_offset;
	vec3 cutoff = vec3(step(abs(warped_uv.x), 1.0) * step(abs(warped_uv.y), 1.0));
	
	vec3 scanlines = vec3(sin(2.0 * warped_uv.y * 200.0) * 0.2 + 0.75);
	
	vec3 vignette = vec3(length(pow(abs(centered_uv), vec2(4.0)) / 6.0));
	
	vec3 screen_color = texture(screen_texture, (warped_uv + 1.0) / 2.0).rgb * cutoff * scanlines;
	screen_color -= vignette * vignette_factor;
	
	// Greenwash lol.
	screen_color.g = (screen_color.r + screen_color.g + screen_color.b) / 3.0;
	screen_color.r = 0.0;
	screen_color.b = 0.0;
	
	COLOR = vec4(screen_color, 1.0);
}