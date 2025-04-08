shader_type spatial;

uniform vec3 color : source_color = vec3(0.0);
uniform sampler2D noise_texture;

void fragment() {
	ALBEDO.rgb = color;
	NORMAL_MAP = texture(noise_texture, UV).rgb;
	//ALBEDO.rg = UV;
}

//void light() {
//	// Called for every pixel for every light affecting the material.
//	// Uncomment to replace the default light processing function with this one.
//}
