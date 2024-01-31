dev_simulator_run:

	cd device_simulator && yarn start

rmq-build:

	cd rmq-bridge && cargo build 

rmq-run:

	cd rmq-bridge && cargo run

consumer-build:

	cd rmq-consumer && cargo build

consumer-run:

	cd rmq-consumer && cargo run