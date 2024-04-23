
compile:
	arduino-cli compile --warnings all --build-cache-path build_cache/ --build-path build/

upload:
	arduino-cli compile -u --warnings all --build-cache-path build_cache/ --build-path build/

monitor:
	arduino-cli monitor -p /dev/ttyACM0

clean:
	rm -rf build_cache build
