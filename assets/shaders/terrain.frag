#version 300 es

#ifdef GL_FRAGMENT_PRECISION_HIGH
	precision highp float;
#else
	precision mediump float;
#endif

precision highp int;

struct PointLight {
	vec3 position;  // meters
	vec3 color;     // 
};

uniform bool USE_DIFFUSE_TEX;
uniform sampler2D DIFFUSE_TEX;

uniform bool USE_NORMAL_TEX;
uniform sampler2D NORMAL_TEX;

uniform bool USE_METALLIC_ROUGHNESS_TEX;
uniform sampler2D METALLIC_ROUGHNESS_TEX;
uniform float METALLIC;
uniform float ROUGHNESS;

uniform bool USE_OCCLUSION_TEX;
uniform sampler2D OCCLUSION_TEX;
uniform float OCCLUSION;

uniform vec3 CAMERA_POS;

in vec3 v_position;
in vec3 v_normal;
in vec3 v_tangent;
in vec3 v_bitangent;
in vec3 v_color;
in vec2 v_texcoord_0;
in vec2 v_texcoord_1;
in vec2 v_texcoord_2;
in vec2 v_texcoord_3;

in vec3 v_world_position;
in vec3 v_world_normal;

in vec3 v_view_position;

layout(location = 0) out lowp vec4 frag_color;

const float PI = 3.14159265359;

vec3 getNormalFromMap();
float DistributionGGX(vec3 N, vec3 H, float roughness);
float GeometrySchlickGGX(float NdotV, float roughness);
float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness);
vec3 fresnelSchlick(float cosTheta, vec3 F0);

void main() {
	PointLight light;
	light.position = vec3(-2, 10.0, 5.0);
	// light.position = vec3(0.0, 100.0, 0.0);
	light.color = vec3(5.0, 5.0, 5.0);

	vec3 N = getNormalFromMap();
	vec3 V = normalize(CAMERA_POS - v_world_position);

	vec3 albedo = vec3(0.0);
	if (USE_DIFFUSE_TEX) {
		albedo = texture(DIFFUSE_TEX, v_texcoord_0).rgb;
	} else {
		albedo = v_color;
	}

	float metallic = 0.0;
	float roughness = 0.0;
	if (USE_METALLIC_ROUGHNESS_TEX) {
		vec3 metallic_roughness = texture(METALLIC_ROUGHNESS_TEX, v_texcoord_0).rgb;
		metallic_roughness = normalize(metallic_roughness);
		metallic = metallic_roughness.r;
		roughness = metallic_roughness.g;
	} else {
		metallic = METALLIC;
		roughness = ROUGHNESS;
	}

	float ao = 0.0;
	if (USE_OCCLUSION_TEX) {
		ao = texture(OCCLUSION_TEX, v_texcoord_0).r;
	} else {
		ao = OCCLUSION;
	}

	vec3 F0 = vec3(0.04); 
	F0 = mix(F0, albedo, metallic);

	vec3 light_out = vec3(0.0);
	{	
		vec3 L = normalize(light.position - v_world_position);
		vec3 H = normalize(V + L);
		float distance = length(light.position - v_world_position);
		float attenuation = 1.0 / (distance * distance);
		attenuation = 1.0;
		vec3 radiance = light.color * attenuation;

		float NDF = DistributionGGX(N, H, roughness);
		float G = GeometrySmith(N, V, L, roughness);
		vec3 F = fresnelSchlick(max(dot(H, V), 0.0), F0);

		vec3 kS = F;
		vec3 kD = vec3(1.0) - kS;
		kD *= 1.0 - metallic;

		vec3 numerator = NDF * G * F;
		float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0);
		vec3 specular = numerator / max(denominator, 0.0000001);  

		float NdotL = max(dot(N, L), 0.0);
		light_out = (kD * albedo / PI + specular) * radiance * NdotL;
	}

	float ambient_strength = 0.03;
	vec3 ambient = vec3(ambient_strength) * albedo.xyz * ao;

	vec3 color = ambient + light_out;

    color = vec3(v_position.y);
	frag_color = vec4(color, 1.0);
}

vec3 getNormalFromMap() {
	vec3 norm = vec3(0.0);
	if (USE_NORMAL_TEX) {
		vec3 tangentNormal = texture(NORMAL_TEX, v_texcoord_0).xyz * 2.0 - 1.0;
		tangentNormal = normalize(tangentNormal);

		vec3 Q1 = dFdx(v_world_position);
		vec3 Q2 = dFdy(v_world_position);
		vec2 st1 = dFdx(v_texcoord_0);
		vec2 st2 = dFdy(v_texcoord_0);

		vec3 N = normalize(v_world_normal);
		vec3 T = normalize(Q1*st2.t - Q2*st1.t);
		vec3 B = -normalize(cross(N, T));
		mat3 TBN = mat3(T, B, N);

		return normalize(TBN * tangentNormal);
	} else {
		norm = v_world_normal;
	}

	norm = v_world_normal;

	return normalize(norm);
}
float DistributionGGX(vec3 N, vec3 H, float roughness) {
	float a = roughness*roughness;
	float a2 = a*a;
	float NdotH = max(dot(N, H), 0.0);
	float NdotH2 = NdotH*NdotH;

	float nom = a2;
	float denom = (NdotH2 * (a2 - 1.0) + 1.0);
	denom = PI * denom * denom;

	return nom / denom;
}
float GeometrySchlickGGX(float NdotV, float roughness) {
	float r = (roughness + 1.0);
	float k = (r*r) / 8.0;

	float nom = NdotV;
	float denom = NdotV * (1.0 - k) + k;

	return nom / denom;
}
float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness) {
	float NdotV = max(dot(N, V), 0.0);
	float NdotL = max(dot(N, L), 0.0);
	float ggx2 = GeometrySchlickGGX(NdotV, roughness);
	float ggx1 = GeometrySchlickGGX(NdotL, roughness);

	return ggx1 * ggx2;
}
vec3 fresnelSchlick(float cosTheta, vec3 F0) {
	return F0 + (1.0 - F0) * pow(max(1.0 - cosTheta, 0.0), 5.0);
}
