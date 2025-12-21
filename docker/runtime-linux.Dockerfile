FROM gcr.io/distroless/cc-debian12:nonroot
COPY --chmod=0755 kuack-checker /kuack-checker
ENTRYPOINT ["/kuack-checker"]
