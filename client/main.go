package main

import (
	"context"
	"log"
	"os"

	pb "github.com/ryutah/hello-grpc/client/helloworld"

	"google.golang.org/grpc"
)

func main() {
	name := "Default"
	if len(os.Args) > 1 {
		name = os.Args[1]
	}
	conn, err := grpc.Dial("127.0.0.1:8080", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("failed to connect to grpc server %v", err)
	}
	defer conn.Close()

	client := pb.NewGreeterClient(conn)
	reply, err := client.SayHello(context.Background(), &pb.HelloRequest{Name: name})
	if err != nil {
		log.Fatalf("failed to get replay: %v", err)
	}

	log.Printf("Get Replay: %q", reply.GetMessage())
}
