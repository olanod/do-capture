FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y firefox-esr
COPY geckodriver /bin
COPY run.sh /bin
RUN chmod +x /bin/run.sh
COPY target/release/capture /bin
RUN chmod +x /bin/capture

CMD ["-s"]
ENTRYPOINT ["run.sh"]
