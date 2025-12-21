FROM gcr.io/distroless/cc-debian12:nonroot
COPY kuack-checker /kuack-checker
ENTRYPOINT ["/kuack-checker"]
