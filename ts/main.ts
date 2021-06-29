import { default as wasm, start } from "../pkg/web_render_rs.js";

import { GltfLoader, GltfAsset } from './gltf/gltf-loader.js';
import { GLTF_ELEMENTS_PER_TYPE } from './gltf/gltf-asset.js';
import { Accessor, BufferView} from './gltf/gltf.js';

export class Config {
	public canvas_id: string;
	public width: number;
	public height: number;
	public camera: Camera;
	public shaders: Shader[];
	public programs: Program[];
	public models: Asset[];
}
export class Camera {
	constructor(
		public fovy: number,
		public znear: number,
		public zfar: number,
		public eye: number[],
		public center: number[],
		public up: number[]
	) {}
}
export class Shader {
	constructor(
		public id: string,
		public shader_type: number,
		public uri: string
	) {}
}
export class Program {
	constructor(
		public id: string,
		public vertex: string,
		public fragment: string,
		public attributes: string[],
		public uniforms: string[]
	) {}
}
export class Asset {
	constructor(
		public id: string,
		public buffers: Uint8Array[],
		public meshes: Mesh[],
		public texture_wrapper_id: string,
		public materials: Material[] | null,
		public samplers: Sampler[] | null,
		public textures: Texture[],
	) {}
}
export class Texture {
	constructor(
		public source: number,
		public sampler: number,
	) {}
}
export class Sampler {
	constructor(
		public min_filter: number,
		public mag_filter: number,
		public wrap_s: number,
		public wrap_t: number,
	) {}
}
export class Material {
	constructor(
		public id: string,
		public diffuse: number,
		public normal: number,
		public metallic_roughness: number,
		public occlusion: number,
	) {}
}
export class Mesh {
	constructor(
		public index_view: MyBufferView,
		public buffer_views: MyBufferView[],
		public material: number,
	) {}
}
export class MyBufferView {
	public id: string;
	public buffer: number;
	public length: number;
	public buffer_offset: number | undefined;
	public offset: number | undefined;
	public stride: number | undefined;
	public component_size: number;
	public component_count: number;
	public component_type: number;
	// public min: number[];
	// public max: number[];
	constructor(id: string, raw_view: BufferView, accessor: Accessor) {
		this.id = id;
		this.buffer = raw_view.buffer;
		this.length = raw_view.byteLength;
		this.buffer_offset = raw_view.byteOffset;
		this.offset = accessor.byteOffset;
		this.stride = raw_view.byteStride;
		this.component_size = GLTF_ELEMENTS_PER_TYPE[accessor.type];
		this.component_count = accessor.count;
		this.component_type = accessor.componentType;
	}
}
export async function load_model(name: string, uri: string): Promise<Asset> {
	let loader = new GltfLoader();
	let raw_asset: GltfAsset = await loader.load(uri);
	await raw_asset.preFetchAll();
	let gltf = raw_asset.gltf;

	let meshes = [];
	for (let mesh of gltf.meshes) {
		for (let primitive of mesh.primitives) {
			let index_view = undefined;
			let indicies = primitive.indices;
			if (indicies != undefined) {
				let index_accessor = gltf.accessors[indicies];
				let raw_index_view = gltf.bufferViews[index_accessor.bufferView];
				index_view = new MyBufferView("INDEX", raw_index_view, index_accessor);
			}
			let buffer_views = [];
			for (let [key, value] of Object.entries(primitive.attributes)) {
				let accessor = gltf.accessors[value];
				let raw_view = gltf.bufferViews[accessor.bufferView];

				let view = new MyBufferView(key, raw_view, accessor);

				buffer_views.push(view);
			}
			meshes.push(new Mesh(index_view, buffer_views, primitive.material));
		}
	}

	let images_div = document.getElementById('image_wrapper');
	await raw_asset.imageData.preFetchAll();
	let imageCache = raw_asset.imageData.imageCache;

	let asset_image_wrapper = document.createElement('div');
	asset_image_wrapper.setAttribute('id', name + '-images');

	let i = 0;
	for (let image of imageCache) {
		image.setAttribute('id', name + '-texture-' + i);
		asset_image_wrapper.appendChild(image);
		i += 1;
	}

	let samplers = [];
	if (gltf.samplers) {
		for (let sampler of gltf.samplers) {
			let min_filter = 9729;
			if (sampler.minFilter) {
				min_filter = sampler.minFilter;
			}
			let mag_filter = 9729;
			if (sampler.magFilter) {
				mag_filter = sampler.magFilter;
			}
			let wrap_s = 33071;
			if (sampler.wrapS) {
				wrap_s = sampler.wrapS;
			}
			let wrap_t = 33071;
			if (sampler.wrapT) {
				wrap_t = sampler.wrapT;
			}
			samplers.push(new Sampler(min_filter, mag_filter, wrap_s, wrap_t));
		}
	}

	let textures = [];
	if (gltf.textures) {
		for (let texture of gltf.textures) {
			let source = texture.source;
			let sampler = texture.sampler;
			textures.push({source, sampler});
		}
	}
	images_div.appendChild(asset_image_wrapper);


	let materials = [];
	if (gltf.materials) {
		for (let material of gltf.materials) {
			let base_color = null;
			let metallic_roughness = null;
			if (material.pbrMetallicRoughness) {
				if (material.pbrMetallicRoughness.baseColorTexture) {
					base_color = material.pbrMetallicRoughness.baseColorTexture.index;
				}
				if (material.pbrMetallicRoughness.metallicRoughnessTexture) {
					metallic_roughness = material.pbrMetallicRoughness.metallicRoughnessTexture.index;
				}
			}
			let normal = null;
			if (material.normalTexture) {
				normal = material.normalTexture.index;
			}
			let occlusion = null;
			if (material.occlusionTexture) {
				occlusion = material.occlusionTexture.index;
			}
			let my_material = new Material(
				name, 
				base_color,
				normal,
				metallic_roughness,
				occlusion
			);
			materials.push(my_material);
		}
	}

	return new Asset(name, raw_asset.bufferData.bufferCache, meshes, 'image_wrapper', materials, samplers, textures);
}

export async function main() {
	let config = new Config();
	config.canvas_id = "webgl-canvas";
	config.width = 800;
	config.height = 600;
	config.camera = new Camera(
		45.0,   // fovy
		0.1,    // znear
		2000.0, // zfar
		[0.0, 0.0, 10.0], // eye
		[0.0, 0.0, 0.0], // center
		[0.0, 1.0, 0.0]  // up
	);
	config.shaders = [		
		new Shader(
			"general_vert",
			WebGLRenderingContext.VERTEX_SHADER,
			"shaders/general.vert"
		),
		new Shader(
			"pbr_frag", 
			WebGLRenderingContext.FRAGMENT_SHADER,
			"shaders/pbr.frag"
		),
		new Shader(
			"terrain_frag",
			WebGLRenderingContext.FRAGMENT_SHADER,
			"shaders/terrain.frag"
		),
	];
	let attributes = [
		"POSITION",
		"NORMAL",
		"TANGENT",
		"BITANGENT",
		"COLOR",
		"TEXCOORD_0",
		"TEXCOORD_1",
		"TEXCOORD_2",
		"TEXCOORD_3",
	];
	let uniforms = [
		"PROJECTION_MATRIX",
		"VIEW_MATRIX",
		"MODEL_MATRIX",
		"CAMERA_POS",

		"DIFFUSE_TEX",
		"NORMAL_TEX",
		"METALLIC_ROUGHNESS_TEX",
		"OCCLUSION_TEX",

		"USE_DIFFUSE_TEX",
		"USE_NORMAL_TEX",
		"USE_METALLIC_ROUGHNESS_TEX",
		"USE_OCCLUSION_TEX",

		"METALLIC",
		"ROUGHNESS",
		"OCCLUSION",
	];
	config.programs = [
		new Program(
			"pbr",
			"general_vert",
			"pbr_frag",
			attributes,
			uniforms
		),
		new Program(
			"terrain",
			"general_vert",
			"terrain_frag",
			attributes,
			uniforms
		)
	];

	Promise.all([
		// load_model("TriangleWithoutIndices", "assets/glTF-Samples/TriangleWithoutIndices/glTF/TriangleWithoutIndices.gltf"),
		// load_model("Triangle", "assets/glTF-Samples/Triangle/glTF/Triangle.gltf"),
		// load_model("Cube", "assets/glTF-Samples/Cube/glTF/Cube.gltf"),
		// load_model("Box", "assets/glTF-Samples/Box/glTF/Box.gltf"),
		// load_model("BoxInterleaved", "assets/glTF-Samples/BoxInterleaved/glTF/BoxInterleaved.gltf"),
		// load_model("BoxTextured", "assets/glTF-Samples/BoxTextured/glTF/BoxTextured.gltf"),
		// load_model("Suzanne", "assets/glTF-Samples/Suzanne/glTF/Suzanne.gltf"),
		// load_model("Avocado", "assets/glTF-Samples/Avocado/glTF/Avocado.gltf"),
		// load_model("Lantern", "assets/glTF-Samples/Lantern/glTF/Lantern.gltf"),
		// load_model("SciFiHelmet", "assets/glTF-Samples/SciFiHelmet/glTF/SciFiHelmet.gltf"),
		// load_model("DamagedHelmet", "assets/glTF-Samples/DamagedHelmet/glTF/DamagedHelmet.gltf"),
		// load_model("Sponza", "assets/glTF-Samples/Sponza/glTF/Sponza.gltf"),
	])
	.then((assets) => {
		config.models = assets;
		wasm().then((_module) => {
			start(config).then((_res: any) => {

			})
			.catch((err: string) => {
				console.log(err);
			});
		});
	});
}

main();