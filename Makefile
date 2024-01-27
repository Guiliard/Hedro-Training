dev_simulator_run:

	cd device_simulator && yarn start

rmq-build:

	cd rmq-bridge && cargo build 

rmq-run:

	cd rmq-bridge && cargo run