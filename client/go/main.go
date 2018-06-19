//go:generate protoc -I $PWD/proto/ $PWD/proto/helloworld.proto --go_out=plugins=grpc:$PWD/client/go/helloworld

package main

import (
	"context"
	"log"
	"os"

	pb "github.com/ryutah/hello-grpc/client/go/helloworld"

	"google.golang.org/grpc"
)

func greet(ctx context.Context, cli pb.GreeterClient, name string) error {
	reply, err := cli.SayHello(ctx, &pb.HelloRequest{Name: name})
	if err != nil {
		return err
	}
	log.Printf("Get Message: %q", reply.GetMessage())
	return nil
}

func main() {
	name := "Default"
	if len(os.Args) > 1 {
		name = os.Args[1]
	}

	conn, err := grpc.Dial("[::]:8080", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("failed to connect to grpc server %v", err)
	}
	defer conn.Close()

	client := pb.NewGreeterClient(conn)
	if err := greet(context.Background(), client, name); err != nil {
		log.Fatalf("failed to get replay: %v", err)
	}
}
