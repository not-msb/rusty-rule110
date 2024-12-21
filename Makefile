debug:
	cargo run

release:
	cargo run --release

video:
	ffmpeg -r 10 -i tmp/image-%d.png output.mp4

clean:
	rm -rf tmp
	rm output.mp4
