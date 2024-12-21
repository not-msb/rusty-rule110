debug:
	cargo run

release:
	cargo run --release

video:
	ffmpeg -hide_banner -loglevel error -r 10 -i tmp/image-%d.png output.mp4

clean:
	rm -rf tmp
	rm -f output.mp4
