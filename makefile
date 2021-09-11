build:
	wasm-pack build ./rust --release --target web --out-name web --out-dir ./dist 
	npx spack
	cp ./rust/dist/web_bg.wasm ./dist/web_bg.wasm

optimize:
	wasm-opt -Oz -o ./dist/web_bg.wasm ./dist/web_bg.wasm

solid:
	cd ./solid-ui && npm run build