FROM golang:1.21-alpine AS builder

WORKDIR /app
COPY services/coordinator /app
RUN go mod download
RUN CGO_ENABLED=0 GOOS=linux go build -o coordinator main.go

FROM alpine:latest
RUN apk --no-cache add ca-certificates
WORKDIR /root/
COPY --from=builder /app/coordinator .

EXPOSE 8080
CMD ["./coordinator"]
