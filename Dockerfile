FROM rust:latest
RUN mkdir /app
WORKDIR /app
COPY . /app
RUN cd /app/server && cargo build -r && mv /app/server/target/release/performance_tester_server /usr/bin/
RUN cd /app/client && cargo build -r && mv /app/client/target/release/performance_tester_client /usr/bin/
EXPOSE 8000