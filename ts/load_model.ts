import { GltfLoader, GltfAsset } from './gltf/gltf-loader.js';
import { GLTF_ELEMENTS_PER_TYPE } from './gltf/gltf-asset.js';
import { Accessor, BufferView} from './gltf/gltf.js';

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