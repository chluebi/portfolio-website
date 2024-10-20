rm -rf ./src/generated
mkdir ./src/generated
protoc --plugin=protoc-gen-ts=./node_modules/.bin/protoc-gen-ts --ts_opt=esModuleInterop=true --js_out=./src/generated --ts_out=./src/generated --proto_path=./src/proto portfolio.proto