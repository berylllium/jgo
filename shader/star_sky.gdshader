shader_type sky;

uniform vec3 stars[1024];
uniform uint star_count;

void sky() {
	for (uint i = 0u; i < star_count; i++) {
		COLOR += vec3(step(0.99999, dot(stars[i], EYEDIR)));
	}
}
