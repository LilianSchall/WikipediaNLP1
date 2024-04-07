prepare:
	mkdir input output
build:
	docker build -t category_fetcher:latest .

run:
	docker run --name fetcher --env "OUTPUT_DIR=/output" --env "INPUT_DIR=/input" -v "$$(pwd)/input:/input" -v "$$(pwd)/output:/output" --env "RAYON_NUM_THREADS=$$(( 2 * $$(echo $$(lscpu | grep "CPU(s):" | head -n 1 | cut -d ":" -f2))))" category_fetcher
