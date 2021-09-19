import { default as wasm, start } from "../pkg/web_render_rs.js";
import { load_model } from './load_model.js';

export class Config {
	constructor(
		public engine_config_uri: string,
		public scene_config_uri: string,
	) {}
}

export async function main() {
	Promise.all([
		// load_model("TriangleWithoutIndices", "assets/glTF-Samples/TriangleWithoutIndices/glTF/TriangleWithoutIndices.gltf"),
		// load_model("Triangle", "assets/glTF-Samples/Triangle/glTF/Triangle.gltf"),
		// load_model("Cube", "assets/glTF-Samples/Cube/glTF/Cube.gltf"),
		// load_model("Box", "assets/glTF-Samples/Box/glTF/Box.gltf"),
		// load_model("BoxInterleaved", "assets/glTF-Samples/BoxInterleaved/glTF/BoxInterleaved.gltf"),
		// load_model("BoxTextured", "assets/glTF-Samples/BoxTextured/glTF/BoxTextured.gltf"),
		// load_model("Suzanne", "assets/glTF-Samples/Suzanne/glTF/Suzanne.gltf"),
		load_model("Avocado", "assets/glTF-Samples/Avocado/glTF/Avocado.gltf"),
		// load_model("Lantern", "assets/glTF-Samples/Lantern/glTF/Lantern.gltf"),
		// load_model("SciFiHelmet", "assets/glTF-Samples/SciFiHelmet/glTF/SciFiHelmet.gltf"),
		// load_model("DamagedHelmet", "assets/glTF-Samples/DamagedHelmet/glTF/DamagedHelmet.gltf"),
		// load_model("Sponza", "assets/glTF-Samples/Sponza/glTF/Sponza.gltf"),
	])
	.then((_assets) => {
		wasm().then((_module) => {
			let config = new Config("engine_config.json", "scene_config.json");
			start(config).then((_res: any) => {
				console.log(_res);
			})
			.catch((err: string) => {
				console.error(err);
			});
		});
	});
}

main();