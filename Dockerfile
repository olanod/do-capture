FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y firefox-esr wget
RUN wget https://github.com/mozilla/geckodriver/releases/download/v0.25.0/geckodriver-v0.25.0-linux64.tar.gz && \
	tar -xzf geckodriver-v0.25.0-linux64.tar.gz -C /usr/local/bin && \
    	rm geckodriver-v0.25.0-linux64.tar.gz
COPY target/release/capture /usr/local/bin
RUN chmod +x /usr/local/bin/capture

CMD ["-s"]
ENTRYPOINT ["capture"]
