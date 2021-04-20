#version 300 es

precision highp float;
precision highp int;

layout(location = 0) in vec3 POSITION;
layout(location = 1) in vec3 NORMAL;
layout(location = 2) in vec3 TANGENT;
layout(location = 3) in vec3 BITANGENT;
layout(location = 4) in vec3 COLOR;
layout(location = 5) in vec2 TEXCOORD_0;
layout(location = 6) in vec2 TEXCOORD_1;
layout(location = 7) in vec2 TEXCOORD_2;
layout(location = 8) in vec2 TEXCOORD_3;

uniform mat4 PROJECTION_MATRIX;
uniform mat4 VIEW_MATRIX;
uniform mat4 MODEL_MATRIX;

out vec3 v_position;
out vec3 v_normal;
out vec3 v_tangent;
out vec3 v_bitangent;
out vec3 v_color;
out vec2 v_texcoord_0;
out vec2 v_texcoord_1;
out vec2 v_texcoord_2;
out vec2 v_texcoord_3;

out vec3 v_world_position;
out vec3 v_world_normal;

out vec3 v_view_position;

void main() {
	v_position = POSITION;
	v_normal = NORMAL;
	v_tangent = TANGENT;
	v_bitangent = BITANGENT;
	v_color = COLOR;
	v_texcoord_0 = TEXCOORD_0;
	v_texcoord_1 = TEXCOORD_1;
	v_texcoord_2 = TEXCOORD_2;
	v_texcoord_3 = TEXCOORD_3;

	v_world_normal = mat3(MODEL_MATRIX) * NORMAL;
	v_world_position = vec3(MODEL_MATRIX * vec4(POSITION, 1.0));

	v_view_position = vec3(VIEW_MATRIX * vec4(v_world_position, 1.0));

	gl_Position = PROJECTION_MATRIX * VIEW_MATRIX * vec4(v_world_position, 1.0);
}
